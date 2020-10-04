//! Implementation of VPP Process Management dedicated Functions
//! This module VppProcessManager can be used as a component to control and "inspect"
//! userspace processes.

#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::cell::Cell;
use core::cmp;
use core::str;
use crate::vpp::mloi::MK_ERROR_e::*;
use crate::vpp::mloi::MK_PROCESS_PRIORITY_e::*;
use crate::vpp::mloi::*;
use crate::vpp::process::*;
use crate::vpp::process;
use kernel::{Kernel, capabilities};
use kernel::introspection::KernelInfo;
use kernel::common::cells::TakeCell;
use kernel::procs::ProcessType;
use kernel::capabilities::ProcessManagementCapability;
use kernel::debug;
use kernel::hil::uart;
use kernel::ReturnCode;
use crate::vpp::process::VppProcess;
use kernel::{ListenerType,LISTENER};
use kernel::{Callback, Driver,Grant,AppId};
use crate::virtual_uart::{MuxUart, UartDevice};
use kernel::static_init;
use kernel::hil;

pub const DRIVER_NUM: usize = 0x90004 ;

const NUM_PROCS: usize = 4 ; // Number of allowed vpp processes.
// This should always eqaul number of tock processes.

// Since writes are character echoes, we do not need more than 4 bytes:
// the longest write is 3 bytes for a backspace (backspace, space, backspace).
pub static mut WRITE_BUF: [u8; 4] = [0; 4];
// Since reads are byte-by-byte, to properly echo what's typed,
// we can use a very small read buffer.
pub static mut READ_BUF: [u8; 4] = [0; 4];
// Commands can be up to 32 bytes long: since commands themselves are 4-5
// characters, limiting arguments to 25 bytes or so seems fine for now.
pub static mut COMMAND_BUF: [u8; 32] = [0; 32];

pub struct VppProcessManager <'a,C: ProcessManagementCapability>{
    uart: &'a dyn uart::UartData<'a>,
    tx_in_progress: Cell<bool>,
    tx_buffer: TakeCell<'static, [u8]>,
    rx_in_progress: Cell<bool>,
    rx_buffer: TakeCell<'static, [u8]>,
    command_buffer: TakeCell<'static, [u8]>,
    command_index: Cell<usize>,
    /// Flag to mark that the process console is active and has called receive
    /// from the underlying UART.
    running: Cell<bool>,
    /// Internal flag that the process console should parse the command it just
    /// received after finishing echoing the last newline character.
    execute: Cell<bool>,

    vpp_processes: [Option<VppProcess>;NUM_PROCS],
    kernel: &'static Kernel,
    capability: C,
    /// Variable that retrieves the last error generated by a Process
    /// used in _mk_Get_Error
    last_error: Cell<MK_ERROR_e>
}

impl <'a,C: ProcessManagementCapability> VppProcessManager<'a, C> {
    pub fn new(
        uart: &'a dyn uart::UartData<'a>,
        tx_buffer: &'static mut [u8],
        rx_buffer: &'static mut [u8],
        cmd_buffer: &'static mut [u8],
        kernel: &'static Kernel,
         capability : C  // unused for the moment. Trying to find a workaround this.
    )
        -> VppProcessManager<'a,C> {
        // add if condition to see if processes exit.
        let tockprocesses = kernel.processes;
        // debug!("Name of First Process {:?}",
        //        tockprocesses[0].unwrap().get_process_name());
        let mut vppprocesses:[Option<VppProcess>;NUM_PROCS] = Default::default();
        // default initializes the array with None
        for i in 0..tockprocesses.len() {
            if tockprocesses[i].is_some(){
                let proc = Some(process::VppProcess::create_vpp_process(tockprocesses[i], i as MK_Process_ID_u ));
                vppprocesses[i] = proc;
                // debug!("Syncing States");
                // dummy variable
                let process = &vppprocesses[i].clone().unwrap();
                process.sync_vpp_tock_states();
                // vppprocesses[i].unwrap().sync_vpp_tock_states();
            }
        }

        // For testing purposes. Leave them like this for now.
        // vppprocesses[1] = None;
        // vppprocesses[2] = None;
        // The array should look like this [(Some(VppProcess, ID 0)), None, None, Some(VppProcess, ID 3))]
        VppProcessManager {
            uart,
            tx_in_progress: Cell::new(false),
            tx_buffer: TakeCell::new(tx_buffer),
            rx_in_progress: Cell::new(false),
            rx_buffer: TakeCell::new(rx_buffer),
            command_buffer: TakeCell::new(cmd_buffer),
            command_index: Cell::new(0),
            running: Cell::new(false),
            execute: Cell::new(false),
            vpp_processes: vppprocesses,
            kernel,
            capability,
            last_error: Cell::new(MK_ERROR_NONE)
        }
    }
    // pub fn update (&mut self){
    //     let tockprocesses = self.kernel.processes;
    //     let mut vppprocesses:[Option<VppProcess>;NUM_PROCS] = Default::default();
    //     for i in 0..tockprocesses.len() {
    //         if tockprocesses[i].is_some(){
    //             debug!("hellO");
    //             let proc = Some(process::VppProcess::create_vpp_process(tockprocesses[i], i as MK_Process_ID_u ));
    //             vppprocesses[i] = proc;
    //         }
    //     }
    //     self.vpp_processes = vppprocesses;
    //
    //
    // }
    // UART dedicated
    pub fn start(&self) -> ReturnCode {
        if self.running.get() == false {
            self.rx_buffer.take().map(|buffer| {
                self.rx_in_progress.set(true);
                self.uart.receive_buffer(buffer, 1);
                self.running.set(true);
                debug!("Starting process console");
            });
        }
        ReturnCode::SUCCESS
    }
    // Process the command in the command buffer and clear the buffer.
    fn read_command(&self) {
        self.command_buffer.map(|command| {
            let mut terminator = 0;
            let len = command.len();
            for i in 0..len {
                if command[i] == 0 {
                    terminator = i;
                    break;
                }
            }
            //debug!("Command: {}-{} {:?}", start, terminator, command);
            // A command is valid only if it starts inside the buffer,
            // ends before the beginning of the buffer, and ends after
            // it starts.
            if terminator > 0 {
                let cmd_str = str::from_utf8(&command[0..terminator]);
                match cmd_str {
                    Ok(s) => {
                        let clean_str = s.trim();
                        if clean_str.starts_with("help") {
                            debug!("Welcome to the process console.");
                            debug!("Valid commands are: help status list test");
                        }
                        else if clean_str.starts_with("list") {
                            debug!(" PID    Name                Quanta  Syscalls  Dropped Callbacks  Restarts    State  Grants   VPPState  Error");
                            self.kernel
                                .process_each_capability(&self.capability, |proc| {
                                    let info: KernelInfo = KernelInfo::new(self.kernel);

                                    let pname = proc.get_process_name();
                                    let appid = proc.appid();
                                    let (grants_used, grants_total) = info.number_app_grant_uses(appid, &self.capability);

                                    let vpp_proc = self.get_process_ref_internal(0).unwrap();

                                    debug!(
                                        "  {:?}\t{:<20}{:6}{:10}{:19}{:10}  {:?}{:5}/{}{:?}{:#?}",
                                        vpp_proc.get_vpp_id(),
                                        pname,
                                        proc.debug_timeslice_expiration_count(),
                                        proc.debug_syscall_count(),
                                        proc.debug_dropped_callback_count(),
                                        proc.get_restart_count(),
                                        proc.get_state(),
                                        grants_used,
                                        grants_total,
                                        vpp_proc.get_vpp_state(),
                                        self.last_error.get(),
                                        );
                                });
                        }
                        else if clean_str.starts_with("status") {
                            let info: KernelInfo = KernelInfo::new(self.kernel);
                            debug!(
                                "Total processes: {}",
                                info.number_loaded_processes(&self.capability)
                            );
                            debug!(
                                "Active processes: {}",
                                info.number_active_processes(&self.capability)
                            );
                            debug!(
                                "Timeslice expirations: {}",
                                info.timeslice_expirations(&self.capability)
                            );
                        }else if clean_str.starts_with("resume") {
                             let argument = clean_str.split_whitespace().nth(1);
                             if argument.is_some(){
                                 if argument.unwrap() == self.vpp_processes[0].as_ref().unwrap().tockprocess.unwrap().get_process_name() {
                                     let handle=self._mk_get_process_handle(0);
                                     // self._mk_Get_Error();
                                     self._mk_resume_process(handle);
                                     // self._mk_Get_Error();
                                     //  self._mk_get_process_priority(handle);
                                     //  self._mk_set_process_priority(handle,MK_PROCESS_PRIORITY_LOW);
                                     //  self._mk_resume_process(handle);

                                 }else {
                                     debug!("Name of Process non existant");
                                 }
                             } else  {
                                 debug!("No Process");
                             }
                         }
                        else {
                            debug!("Valid commands are: help status list resume");
                        }
                    }
                    Err(_e) => debug!("Invalid command: {:?}", command),
                }
            }
        });
        self.command_buffer.map(|command| {
            command[0] = 0;
        });
        self.command_index.set(0);
    }

    fn write_byte(&self, byte: u8) -> ReturnCode {
        if self.tx_in_progress.get() {
            ReturnCode::EBUSY
        } else {
            self.tx_in_progress.set(true);
            self.tx_buffer.take().map(|buffer| {
                buffer[0] = byte;
                self.uart.transmit_buffer(buffer, 1);
            });
            ReturnCode::SUCCESS
        }
    }

    fn write_bytes(&self, bytes: &[u8]) -> ReturnCode {
        if self.tx_in_progress.get() {
            ReturnCode::EBUSY
        } else {
            self.tx_in_progress.set(true);
            self.tx_buffer.take().map(|buffer| {
                let len = cmp::min(bytes.len(), buffer.len());
                // Copy elements of `bytes` into `buffer`
                (&mut buffer[..len]).copy_from_slice(&bytes[..len]);
                self.uart.transmit_buffer(buffer, len);
            });
            ReturnCode::SUCCESS
        }
    }

    // END UART dedicated

    // VPP dedicated
    // this needs to be re-implemented. It is specific to a Process. So probably
    // it needs to be an attribute of the VPP Process. Look Into This.
     pub  fn _mk_Get_Error(&self) {
       self.last_error.get();
     }

    pub fn get_process_ref_internal(&self, handle: MK_HANDLE_t) -> Option<&VppProcess> {
        // Mapping id to handle. For the time being,
        // we consider the handle as the id but in 32 bits. This will probably be changed later.
        let id = convert_to_id(handle);
        let mut return_pointer: Option<&VppProcess>  = None;
        for process in self.vpp_processes.iter() {
            match process {
                Some(proc) => {
                    if proc.get_vpp_id() == id {
                        // even if id found, the Process must not be in "DEAD" state
                         if proc.get_vpp_state() == VppState::DEAD {
                            self.last_error.set(MK_ERROR_e::MK_ERROR_UNKNOWN_ID);
                             // debug!("VPP Process is DEAD");
                            return_pointer =  None;
                        }
                        // if the Process in any other state, a pointer to
                        // that process is delivered with a success flag and
                        // break of the loop
                        else {
                            self.last_error.set(MK_ERROR_e::MK_ERROR_NONE);
                            //debug!("Found a Process with  ID {:?}", proc.get_vpp_id());
                            return_pointer = Some(proc);
                            break;
                        }
                    }
                    else {
                        self.last_error.set(MK_ERROR_UNKNOWN_ID);
                        // debug!("VPP Process ID is NOT found");
                        return_pointer = None;
                    }
                }

                None => {
                    self.last_error.set(MK_ERROR_UNKNOWN_ID);
                    // debug!("No VPP Process Exists");
                    return_pointer = None;
                }
            }
        }
        return_pointer

        // Leaving this buggy implementation here. It might be helpfull.

        // // Mapping id to handle. For the time being,
        // // we consider the handle as the id but in 32 bits. This will probably be changed later.
        // let id = convert_to_id(handle);
        //
        // self.vpp_processes.iter().find_map(|proc| {
        //     if proc.get_vpp_id() == id {
        //         // even if id found, the Process must not be in "DEAD" state
        //         if proc.get_vpp_state() == VppState::DEAD {
        //             self.last_error.set(MK_ERROR_UNKNOWN_ID);
        //             debug!("VPP Process is in DEAD state");
        //             None
        //         }
        //         // if the Process in any other state, a pointer to
        //         // that process is delivered with a success flag
        //         else {
        //             self.last_error.set(MK_ERROR_NONE) ;
        //             debug!("VPP Process is found");
        //             proc.as_ref()
        //         }
        //     // if the id was not found, Unknown ID error is raised.
        //     } else {
        //         self.last_error.set(MK_ERROR_UNKNOWN_ID) ;
        //         debug!("VPP Process ID is not  found");
        //         None
        //     }
        // })
    }

    /// Replication of get_proc_ref_internal and return each valid Process Reference
    /// without the need of a handle. Used for Debugging Only
    // pub fn get_each_process_ref_internal(&self) -> Option<&VppProcess> {
    //     let mut return_pointer: Option<&VppProcess>  = None;
    //
    //     for process in self.vpp_processes.iter() {
    //         match process {
    //             Some(proc) => { return_pointer = Some(proc) },
    //             None => {return_pointer= None}
    //         }
    //     }
    //     return_pointer
    // }


    pub (crate)  fn _mk_get_process_handle(& self, _eProcess_ID: MK_Process_ID_u)
                                                 -> MK_HANDLE_t {
        let handle = convert_to_handle(_eProcess_ID);
        let process =self.get_process_ref_internal(handle);
        if process.is_some() {handle}  else { 0 }
        // there is a problem when returning 0 as a handle. This might be in fact
        // the id of another handle. Whether, a Process ID as 0 is not allowed
        // or wrap this with an Option.
    }

    // Concerning Priorities there is another missing function that needs to be implemented.
    // Based on the index of Tock Processes, Vpp Priorities are mapped accordingly.

    pub (crate) fn _mk_get_process_priority(& self, _hProcess: MK_HANDLE_t) -> MK_PROCESS_PRIORITY_e {
        let process = self.get_process_ref_internal(_hProcess);
        if process.is_some(){
            let prio = process.unwrap().get_vpp_priority();
            debug!("Process Priority is {:?}", prio );
            prio
        }
        else {
            //self.last_error.set(MK_ERROR_UNKNOWN_HANDLE);
            MK_PROCESS_PRIORITY_ERROR

        }
    }


    pub (crate) fn _mk_set_process_priority(&self, _hProcess: MK_HANDLE_t,
                                       _xPriority: MK_PROCESS_PRIORITY_e) -> MK_ERROR_e {

        // Check for UNKNOWN_PRIORITY by figuring out the encoding of the enum in rust
        // Check for the value _xPriority if different from those 4 values
        // TO-DO

        let process =self.get_process_ref_internal(_hProcess);
         if process.is_some() {
            process.unwrap().set_vpp_priority(_xPriority);
             debug!("Process Priority set to {:?}",_xPriority );
             MK_ERROR_NONE
        } else {
            MK_ERROR_UNKNOWN_HANDLE
        }

        // TO DO
        // Depending of the Scheduler Type, this can be implemented as follows:
        // Based on the index on the PROCESSES Array, priorities can be defined
        // index 0: MK_PROCESS_PRIORITY_HIGH
        // index 1: MK_PROCESS_PRIORITY_NORMAL
        // index 2: MK_PROCESS_PRIORITY_LOW
        // match _xPriority {
        //     // check for the index of the PROCESSES ARRAY and change accordingly
        //     MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_LOW => {
        //         // _hProcess.tockprocess.appid.index changes
        //     }
        //     MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_NORMAL => {
        //     }
        //     MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_HIGH => {
        //     }
        //     MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_ERROR => {
        //         MK_ERROR_UNKNOWN_PRIORITY
        //         // Is this the right use case ?
        //     }
        // }

    }

    pub (crate) fn _mk_suspend_process(&self, mut _hProcess: MK_HANDLE_t) -> MK_ERROR_e {
        let vppprocess = self.get_process_ref_internal(_hProcess);
        if vppprocess.is_some() {
            let process = vppprocess.unwrap();
            //debug!("Vpp Process State before suspending {:?}", process.get_vpp_state());
            // Suspend Vpp Process State
            process.suspend_vpp_process();
           // debug!("Vpp Process State after suspending {:?}", process.get_vpp_state());
            // Suspend Tock Process State
            // You may have a VPP Process existing however it has no Tock Process.
            // If You try to run this function on a that VPP PRocess it will crush
            // because there is no Tock Process linked.
            // To understand the bug, run this function on a VPP Process that has
            // None as a Tock Process
            // This applies to all other functions when a tockprocess is concerned.

            let vppproc_name = process.tockprocess.unwrap().get_process_name();
            // process.tockprocess.unwrap().set_yielded_state();
            process.tockprocess.unwrap().stop();
            debug!("Tock Process {} suspended", vppproc_name);

            // self.kernel.process_each_capability(
            //     &self.capability,
            //     |proc| {
            //         let vppproc_name = process.tockprocess.unwrap().get_process_name();
            //         if vppproc_name ==   proc.get_process_name() {
            //             proc.stop();
            //             debug!("Tock Process {} Suspended", vppproc_name);
            //         }
            //     }
            // );
            MK_ERROR_NONE
        } else {
            MK_ERROR_UNKNOWN_HANDLE
        }

    }

    pub (crate) fn _mk_resume_process(&self, mut _hProcess: MK_HANDLE_t) -> MK_ERROR_e {
        let vppprocess = self.get_process_ref_internal(_hProcess);
        if vppprocess.is_some() {
            let process = vppprocess.unwrap();
            //debug!("Vpp Process State before resuming {:?}", process.get_vpp_state());
            // Resume Vpp Process State
            process.resume_vpp_process();
            //debug!("Vpp Process State after resuming {:?}", process.get_vpp_state());

            // Resume Tock Process State

            let vppproc_name = process.tockprocess.unwrap().get_process_name();
            process.tockprocess.unwrap().resume();
            debug!("Tock Process {} Resumed", vppproc_name);

            // self.kernel.process_each_capability(
            //     &self.capability,
            //     |proc| {
            //         let vppproc_name = process.tockprocess.unwrap().get_process_name();
            //         if vppproc_name ==   proc.get_process_name() {
            //             process.tockprocess.unwrap().resume();
            //             debug!("Tock Process {} Resumed", vppproc_name);
            //         }
            //     }
            // );
            MK_ERROR_NONE
        } else {
            MK_ERROR_UNKNOWN_HANDLE
        }

    }

   pub fn  _mk_yield (&self,_hProcess: MK_HANDLE_t) {
       // Change state of VppState
       let vpp_process = self.get_process_ref_internal(_hProcess);
       if vpp_process.is_some(){
            vpp_process.unwrap().yield_vpp_process();
           // Change Tock State
           vpp_process.unwrap().tockprocess.unwrap().set_yielded_state();
       }
   }
    // VPP dedicated
}


impl<'a,C: ProcessManagementCapability> uart::TransmitClient for VppProcessManager<'a,C> {
    fn transmitted_buffer(&self, buffer: &'static mut [u8], _tx_len: usize, _rcode: ReturnCode) {
        self.tx_buffer.replace(buffer);
        self.tx_in_progress.set(false);

        // Check if we just received and echoed a newline character, and
        // therefore need to process the received message.
        if self.execute.get() {
            self.execute.set(false);
            self.read_command();
        }
    }
}
impl<'a,C: ProcessManagementCapability> uart::ReceiveClient for VppProcessManager<'a,C> {
    fn received_buffer(
        &self,
        read_buf: &'static mut [u8],
        rx_len: usize,
        _rcode: ReturnCode,
        error: uart::Error,
    ) {
        if error == uart::Error::None {
            match rx_len {
                0 => debug!("ProcessConsole had read of 0 bytes"),
                1 => {
                    self.command_buffer.map(|command| {
                        let index = self.command_index.get() as usize;
                        if read_buf[0] == ('\n' as u8) || read_buf[0] == ('\r' as u8) {
                            self.execute.set(true);
                            self.write_bytes(&['\r' as u8, '\n' as u8]);
                        } else if read_buf[0] == ('\x08' as u8) && index > 0 {
                            // Backspace, echo and remove last byte
                            // Note echo is '\b \b' to erase
                            self.write_bytes(&['\x08' as u8, ' ' as u8, '\x08' as u8]);
                            command[index - 1] = '\0' as u8;
                            self.command_index.set(index - 1);
                        } else if index < (command.len() - 1) && read_buf[0] < 128 {
                            // For some reason, sometimes reads return > 127 but no error,
                            // which causes utf-8 decoding failure, so check byte is < 128. -pal

                            // Echo the byte and store it
                            self.write_byte(read_buf[0]);
                            command[index] = read_buf[0];
                            self.command_index.set(index + 1);
                            command[index + 1] = 0;
                        }
                    });
                }
                _ => debug!(
                    "ProcessConsole issues reads of 1 byte, but receive_complete was length {}",
                    rx_len
                ),
            };
        }
        self.rx_in_progress.set(true);
        self.uart.receive_buffer(read_buf, 1);
    }
}

// Instantiating  Component
pub struct Capability;
unsafe impl capabilities::ProcessManagementCapability for Capability {}

pub struct ProcessConsoleComponent {
    board_kernel: &'static kernel::Kernel,
    uart_mux: &'static MuxUart<'static>,
}

impl ProcessConsoleComponent {
    pub fn new(
        board_kernel: &'static kernel::Kernel,
        uart_mux: &'static MuxUart,
    ) -> ProcessConsoleComponent {
        ProcessConsoleComponent {
            board_kernel: board_kernel,
            uart_mux: uart_mux,
        }
    }
    pub unsafe fn finalize(self, _s: ()) -> &'static VppProcessManager<'static,Capability>
    {
        // Create virtual device for console.
        let console_uart = static_init!(UartDevice, UartDevice::new(self.uart_mux, true));
        console_uart.setup();

        let console = static_init!(
            VppProcessManager<'static,Capability>,
            VppProcessManager::new(
                console_uart,
                &mut WRITE_BUF,
                &mut READ_BUF,
                &mut COMMAND_BUF,
                self.board_kernel,
                Capability
            )
        );

        hil::uart::Transmit::set_transmit_client(console_uart, console);
        hil::uart::Receive::set_receive_client(console_uart, console);

        console
    }
}

// Implementing a Syscall Driver for Vpp Process Manager (VPM)

pub struct VPMDriver {
    vpm: &'static VppProcessManager<'static,Capability>,
    // apps: Grant<Option<Callback>>,
}
impl VPMDriver{
    pub fn new( vpm:  &'static VppProcessManager<'static,Capability>) -> VPMDriver{
                // grant: Grant<Option<Callback>>,) -> VPMDriver{
        VPMDriver{
            vpm,
            // apps: grant
        }
    }
}


impl Driver  for VPMDriver {
    fn command(&self,
               command_num: usize,
               data: usize,
               _data2: usize,
               appid: AppId) -> ReturnCode {
        match command_num {
            0 => ReturnCode::SUCCESS,
            1 =>
                {
                    debug!("Suspending Process");
                    //debug!("Data is {:?}", data);
                    let handle = convert_to_handle(data as u16);
                    //let handle = self.vpm._mk_get_process_handle(0);
                    self.vpm._mk_suspend_process(handle);
                    self.vpm._mk_Get_Error();
                    debug!("Last Error {:?}", self.vpm.last_error.get());
                    ReturnCode::SUCCESS
                },
            2 =>
                {
                    debug!("Resuming Process");
                    let handle = convert_to_handle(data as u16);
                    self.vpm._mk_resume_process(handle);
                    self.vpm._mk_Get_Error();
                    ReturnCode::SUCCESS
                },
            3 =>
                {
                    debug!("Yielding  Process");
                    let handle = convert_to_handle(appid.id() as u16  );
                    debug!("ID of process {:?}",appid);
                    self.vpm._mk_yield(handle);
                    self.vpm._mk_Get_Error();
                    ReturnCode::SUCCESS
                },
            4 =>
                {
                    debug!("Testing States");
                    let process = self.vpm.get_process_ref_internal(data as u32);
                    process.unwrap().sync_vpp_tock_states();
                    let tock_state = process.unwrap().tockprocess.unwrap().get_state();
                    let vpp_state = process.unwrap().vppstate.get();
                    debug!("Tock State {:?} , Vpp Process {:?}", tock_state, vpp_state);
                    ReturnCode::SUCCESS
                },
            // 4 =>
            //     {
            //         debug!("Getting  Process Handle");
            //         let handle = self.vpm._mk_get_process_handle(data as u16);
            //         let data = handle ;
            //         self.vpm._mk_Get_Error();
            //         ReturnCode::SUCCESS
            //     },
            _ => ReturnCode::ENOSUPPORT,
        }
    }
}


