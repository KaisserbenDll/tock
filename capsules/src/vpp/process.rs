#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::vpp::mloi::*;
use core::cell::Cell;
use kernel::procs::{State, ProcessType, Process, FaultResponse, FunctionCall, FunctionCallSource, ProcessLoadError};
use crate::vpp::mloi::VppState::*;
use crate::vpp::mloi::MK_Process_ID_u;
use crate::vpp::mailbox::mbox;
use crate::vpp::ipc::ipc;
use kernel::{Kernel, Chip, config, mpu};
use kernel::capabilities::{ProcessManagementCapability,MemoryAllocationCapability};
use core::convert::TryInto;
use kernel::tbfheader;
use crate::vpp;
use kernel::{debug, static_init,create_capability};
use crate::vpp::vppkernel::NUM_PROCS;
use kernel::mpu::MPU;

#[derive(Clone)]
pub struct VppProcess {
    pub(crate) tockprocess: Option<&'static dyn ProcessType>,
    pub(crate) vppstate: Cell<VppState>,
    pub(crate) vpppriority: Cell<MK_PROCESS_PRIORITY_e>,
    pub(crate) vppid: Cell<MK_Process_ID_u>,
    pub(crate) error: Cell<MK_ERROR_e>,
}
/// This is a replication of `load_processes` function by tock with the addition of VPP
/// Process specification. It returns an array `procs` of VPP Processes.
pub unsafe fn  load_vpp_processes<C: Chip>(
    kernel: &'static Kernel,
    chip: &'static C,
    tock_procs: &'static mut [Option<&'static dyn ProcessType>],
    vpp_procs : &'static mut [Option<VppProcess>],
    mbs: &'static mut [Option<mbox>],
    ipcs: &'static mut [Option<ipc>],
    fault_response: FaultResponse,
    _capability: &dyn ProcessManagementCapability)
    -> Result<(), ProcessLoadError> {
    // I am importing these symbols to calculate SRAM and FLASH Addresses from the main function
    /// These symbols are defined in the linker script.
    /// These variables refer to the start and length of SRAM and FLASH Addresses.
    /// They are used in load_processes function to parse the TBF header and
    /// create Tock Processes based on that header.
    ///
    /// app_flash refers to the address slice, where the TBF Header is first located
    /// and the first entry point function. The first entry point is defined in libtock-c
    /// in ctr0.c
    extern "C" {
        /// Beginning of the ROM region containing app images.
        static _sapps: u8;
        /// End of the ROM region containing app images.
        static _eapps: u8;
        /// Beginning of the RAM region for app memory.
        static mut _sappmem: u8;
        /// End of the RAM region for app memory.
        static _eappmem: u8;
    }
    let app_flash = core::slice::from_raw_parts(
        &_sapps as *const u8,
        &_eapps as *const u8 as usize - &_sapps as *const u8 as usize);
    let app_memory =  core::slice::from_raw_parts_mut(
        &mut _sappmem as *mut u8,
        &_eappmem as *const u8 as usize - &_sappmem as *const u8 as usize,
    );


    if config::CONFIG.debug_load_processes {
        debug!(
            "Loading processes from flash={:#010X}-{:#010X} into sram={:#010X}-{:#010X}",
            app_flash.as_ptr() as usize,
            app_flash.as_ptr() as usize + app_flash.len() - 1,
            app_memory.as_ptr() as usize,
            app_memory.as_ptr() as usize + app_memory.len() - 1
        );
    }
    // Before instantiating any Process, mailboxes and IPCs need to be instantiated. IPCs
    // use grants which can no longer be used after calling the create() function. Declaring
    // them out of the loop is a better option.
    //There are 4 Mailboxes.
    let mgt_main_mb = mbox::create_mgt_mb();
    let com_main_mb = mbox::create_com_mb();
    let main_mgt_mb = mbox::create_main_mgt_mb();
    let main_com_mb = mbox::create_main_com_mb();
    mbs[0] = Some(mgt_main_mb);
    mbs[1] = Some(com_main_mb);
    mbs[2] = Some(main_mgt_mb);
    mbs[3] = Some(main_com_mb);
    // There are 4 ipc structs
    let memory_allocation_cap = create_capability!(MemoryAllocationCapability);
    let main_com_ipc = ipc::create_main_com_ipc(kernel,&memory_allocation_cap);
    let com_main_ipc = ipc::create_com_main_ipc(kernel,&memory_allocation_cap);
    let main_mgt_ipc = ipc::create_main_mgt_ipc(kernel,&memory_allocation_cap);
    let mgt_main_ipc = ipc::create_mgt_main_ipc(kernel,&memory_allocation_cap) ;
    ipcs[0] = Some(main_com_ipc);
    ipcs[1] = Some(com_main_ipc);
    ipcs[2] = Some(main_mgt_ipc);
    ipcs[3] = Some(mgt_main_ipc);
    /* |15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0|
  VPP : 1  0   ->>>>>>>Enumerated ID<<<<<<<<-
   For example MGT ID
        1  0 0  0 | 0  0  0 0| 0 0 0 0|0 0 0 1
            8            0        0        1
            MGT Process ID is 0x8001
    */
   /* debug!("COM_VPP_ID is          {:#06X}",    MK_PROCESS_COM_VPP_ID);
    debug!("MGT_VPP_ID is          {:#06X}",    MK_PROCESS_MGT_VPP_ID);
    debug!("MAIN_APP_ID is         {:#06X}",    MK_PROCESS_MAIN_APP_ID);

    debug!("Mailbox COM_MAIN_ID is {:#06X}",    MK_MAILBOX_COM_MAIN_ID);
    debug!("Mailbox MGT_MAIN_ID is {:#06X}",    MK_MAILBOX_MGT_MAIN_ID);
    debug!("Mailbox MAIN_COM_ID is {:#06X}",    MK_MAILBOX_MAIN_COM_ID);
    debug!("Mailbox MAIN_MGT_ID is {:#06X}",    MK_MAILBOX_MAIN_MGT_ID);

    debug!("IPC MGT_MAIN_ID is     {:#06X}",    MK_IPC_MGT_MAIN_ID);
    debug!("IPC COM_MAIN_ID is     {:#06X}",    MK_IPC_COM_MAIN_ID);
    debug!("IPC MAIN_COM_ID is     {:#06X}",    MK_IPC_MAIN_COM_ID);
    debug!("IPC MAIN_MGT_ID is     {:#06X}",    MK_IPC_MAIN_MGT_ID);
    COM_VPP_ID is          0x8000
MGT_VPP_ID is          0x8001
MAIN_APP_ID is         0x4000
Mailbox COM_MAIN_ID is 0x8000
Mailbox MGT_MAIN_ID is 0x8001
Mailbox MAIN_COM_ID is 0x4000
Mailbox MAIN_MGT_ID is 0x4001
IPC MGT_MAIN_ID is     0x8001
IPC COM_MAIN_ID is     0x8000
IPC MAIN_COM_ID is     0x4000
IPC MAIN_MGT_ID is     0x4001

*/

    let mut remaining_flash = app_flash;
    let mut remaining_memory = app_memory;

    for i in 0..vpp_procs.len(){
        // Get the first eight bytes of flash to check if there is another
        // app.
        let test_header_slice = match remaining_flash.get(0..8) {
            Some(s) => s,
            None => {
                // Not enough flash to test for another app. This just means
                // we are at the end of flash, and there are no more apps to
                // load.
                return Ok(());
            }
        };
        // Pass the first eight bytes to tbfheader to parse out the length of
        // the tbf header and app. We then use those values to see if we have
        // enough flash remaining to parse the remainder of the header.
        let (version, header_length, entry_length) = match tbfheader::parse_tbf_header_lengths(
            test_header_slice
                .try_into()
                .or(Err(ProcessLoadError::InternalError))?,
        ) {
            Ok((v, hl, el)) => (v, hl, el),
            Err(tbfheader::InitialTbfParseError::InvalidHeader(entry_length)) => {
                // If we could not parse the header, then we want to skip over
                // this app and look for the next one.
                (0, 0, entry_length)
            }
            Err(tbfheader::InitialTbfParseError::UnableToParse) => {
                // Since Tock apps use a linked list, it is very possible the
                // header we started to parse is intentionally invalid to signal
                // the end of apps. This is ok and just means we have finished
                // loading apps.
                return Ok(());
            }
        };

        // Now we can get a slice which only encompasses the length of flash
        // described by this tbf header.  We will either parse this as an actual
        // app, or skip over this region.
        let entry_flash = remaining_flash
            .get(0..entry_length as usize)
            .ok_or(ProcessLoadError::NotEnoughFlash)?;

        // Advance the flash slice for process discovery beyond this last entry.
        // This will be the start of where we look for a new process since Tock
        // processes are allocated back-to-back in flash.
        remaining_flash = remaining_flash
            .get(entry_flash.len()..)
            .ok_or(ProcessLoadError::NotEnoughFlash)?;

        // Need to reassign remaining_memory in every iteration so the compiler
        // knows it will not be re-borrowed.
        remaining_memory = if header_length > 0 {
            // If we found an actual app header, try to create a `Process`
            // object. We also need to shrink the amount of remaining memory
            // based on whatever is assigned to the new process if one is
            // created.

            // Try to create a process object from that app slice. If we don't
            // get a process and we didn't get a loading error (aka we got to
            // this point), then the app is a disabled process or just padding.
            let (process_option, unused_memory) =
                Process::create(
                    kernel,
                    chip,
                    entry_flash,
                    header_length as usize,
                    version,
                    remaining_memory,
                    fault_response,
                    i,
                )?;

            process_option.map(|process| {
                if config::CONFIG.debug_load_processes {
                    debug!(
                        "Loaded process[{}] from flash={:#010X}-{:#010X} into sram={:#010X}-{:#010X} = {:?}",
                        i,
                        entry_flash.as_ptr() as usize,
                        entry_flash.as_ptr() as usize + entry_flash.len() - 1,
                        process.mem_start() as usize,
                        process.mem_end() as usize - 1,
                        process.get_process_name()
                    );
                }
                tock_procs[i] = Some(process);


                // let mailbox= static_init!(vpp::mailbox::mbox,
                // vpp::mailbox::mbox::new(0,i,i+1));
                // let ipc = static_init!(vpp::ipc::ipc,
                // vpp::ipc::ipc::new(0,64,0,1));

              /*  let vpp_process = VppProcess::create_vpp_process(
                    tock_procs[i],
                    i as MK_Process_ID_u);
                    // Some(mailbox),
                    // Some(ipc));

                // starting any process with the StoppedYielded State
               vpp_process.tockprocess.map(|proc| {
                    let ccb = FunctionCall {
                        source: FunctionCallSource::Kernel,
                        pc: proc.flash_non_protected_start() as usize,
                        argument0: proc.flash_start() as usize,
                        argument1: proc.mem_start() as usize,
                        argument2: proc.mem_end() as usize - proc.mem_start() as usize,
                        argument3: proc.kernel_memory_break() as usize,
                    };
                    proc.set_process_function(ccb);
                    proc.set_yielded_state();
                    proc.stop();
                    // let _ccb = proc.dequeue_task();
                });

                // Save the reference to this process in the processes array.
                //vpp_procs[i] = Some(vpp_process);*/
            });
            unused_memory
        } else {
            // We are just skipping over this region of flash, so we have the
            // same amount of process memory to allocate from.
            remaining_memory
        };
        // Before instantiating the Vpp Kernel with vpp_processes, ipc structs and mailboxes
        // ,let us make sure that the first Process is the MGT Process, the second Process is
        // the COM Process and the 3rd Process is the MAIN Process (which is the
        // actual Userspace App). Also the ipc and mailbox structs of the MGT/COM/MAIN Processes
        // will be instantiated.
        let mgt_process = VppProcess::create_mgt_process(tock_procs[0]);
        let com_process = VppProcess::create_com_process(tock_procs[1]);
        let main_process = VppProcess::create_main_process(tock_procs[2]);
        main_process.tockprocess.map(|proc| {
            let ccb = FunctionCall {
                source: FunctionCallSource::Kernel,
                pc: proc.flash_non_protected_start() as usize,
                argument0: proc.flash_start() as usize,
                argument1: proc.mem_start() as usize,
                argument2: proc.mem_end() as usize - proc.mem_start() as usize,
                argument3: proc.kernel_memory_break() as usize,
            };
            proc.set_process_function(ccb);
            proc.set_yielded_state();
            proc.stop();});
        vpp_procs[0] = Some(mgt_process);
        vpp_procs[1] = Some(com_process);
        vpp_procs[2] = Some(main_process);
    }
    Ok(())
}

impl  VppProcess{
    pub fn create_vpp_process(
        tockprocess: Option<&'static dyn ProcessType>,
        pid : MK_Process_ID_u
        )-> VppProcess{
        VppProcess {
            tockprocess: tockprocess,
            vppstate: Cell::new(VppState::SUSPENDED_R),
            vpppriority: Cell::new(MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_NORMAL),
            vppid: Cell::new(pid),
            error: Cell::new(MK_ERROR_e::MK_ERROR_NONE)
        }
    }
    pub fn create_mgt_process(tockprocess: Option<&'static dyn ProcessType> ) -> VppProcess{
        VppProcess{
            tockprocess: tockprocess,
            vppstate: Cell::new(VppState::READY),
            vpppriority: Cell::new(MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_NORMAL),
            vppid: Cell::new(MK_PROCESS_MGT_VPP_ID),
            error: Cell::new(MK_ERROR_e::MK_ERROR_NONE)
        }
    }
    pub fn create_com_process(tockprocess: Option<&'static dyn ProcessType>) -> VppProcess{
        VppProcess{
            tockprocess,
            vppstate: Cell::new(VppState::READY),
            vpppriority: Cell::new(MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_NORMAL),
            vppid: Cell::new(MK_PROCESS_COM_VPP_ID),
            error: Cell::new(MK_ERROR_e::MK_ERROR_NONE)
        }
    }
    pub fn create_main_process(tockprocess: Option<&'static dyn ProcessType>) -> VppProcess{
        VppProcess{
            tockprocess,
            vppstate: Cell::new(VppState::READY),
            vpppriority: Cell::new(MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_NORMAL),
            vppid: Cell::new(MK_PROCESS_MAIN_APP_ID),
            error: Cell::new(MK_ERROR_e::MK_ERROR_NONE)
        }
    }

    /*pub(crate) fn snyc_tock_vpp_states(&self){
        let tock_state = self.tockprocess.unwrap().get_state();
        match tock_state {
            State::Unstarted => self.vppstate.set(VppState::READY),
            State::Yielded => self.vppstate.set(VppState::READY),
            State::Running => self.vppstate.set(VppState::RUNNING),
            State::StoppedYielded => self.vppstate.set(VppState::SUSPENDED_R),
            State::StoppedRunning => self.vppstate.set(VppState::SUSPENDED_R),
            State::StoppedFaulted => self.vppstate.set(VppState::DEAD),
            State::Fault => self.vppstate.set(VppState::DEAD),
        }
    }
     pub(crate) fn sync_vpp_tock_states(&self) {
        let vpp_state = self.vppstate.get();
        let tock_process = self.tockprocess.unwrap()  ;
        match vpp_state {
            VppState::READY =>  tock_process.set_state(State::Yielded)  ,
            VppState::RUNNING => tock_process.set_state(State::Running),
            VppState::SUSPENDED_R => tock_process.set_state(State::StoppedYielded),
            VppState::DEAD => tock_process.set_state(State::StoppedFaulted),
            _ => {},
        }
    }  */
    pub(crate) fn get_last_generated_error(&self) -> MK_ERROR_e {
        self.error.get()
    }

    pub(crate)fn get_vpp_id(&self) -> MK_Process_ID_u {
        self.vppid.get()
    }

    pub(crate) fn get_vpp_handle(&self) -> MK_HANDLE_t {
        self.vppid.get() as u32
    }

    pub(crate) fn  get_vpp_state(&self) -> VppState {
        self.vppstate.get()
    }

    pub(crate) fn get_vpp_priority (&self) -> MK_PROCESS_PRIORITY_e {
        self.vpppriority.get()
    }

    pub(crate) fn set_vpp_priority(&self, prio:MK_PROCESS_PRIORITY_e ) {
        self.vpppriority.set(prio)
    }

    pub(crate) fn suspend_vpp_process(&self) {
        // self.snyc_tock_vpp_states();
        match self.vppstate.get() {
            VppState::READY    => self.vppstate.set(SUSPENDED_R),
            VppState::RUNNING  => self.vppstate.set(SUSPENDED_R),
            VppState::WAITING  => self.vppstate.set(SUSPENDED_W),
            VppState::SYNC     => self.vppstate.set(SUSPENDED_S),
            _                  => {},
        }
    }

    pub(crate) fn resume_vpp_process(&self) {
        // self.snyc_tock_vpp_states();
        match self.vppstate.get() {
            VppState::SUSPENDED_R => self.vppstate.set(READY),
            VppState::SUSPENDED_W => self.vppstate.set(WAITING),
            VppState::SUSPENDED_S => self.vppstate.set(SYNC),
            _                     => {},
        }
    }

    pub(crate) fn yield_vpp_process(&self) {
        // self.snyc_tock_vpp_states();
        match self.vppstate.get() {
            VppState::RUNNING => self.vppstate.set(READY),
            _                 => {},
        }
    }

    pub (crate) fn waiting_vpp_process(&self) {
        match self.vppstate.get() {
            VppState::RUNNING => self.vppstate.set(WAITING),
            _                 => {},
        }
    }

    pub(crate) fn set_vpp_id(&self, id :MK_Process_ID_u ) {
        self.vppid.set(id);
    }

    pub(crate) fn get_process_name(&self)-> &'static str{
        self.tockprocess.unwrap().get_process_name()
    }
}

