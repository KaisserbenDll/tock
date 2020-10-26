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
use crate::vpp::mloi::VppState::*;
use crate::vpp::process;
use kernel::{Kernel, capabilities, Chip};
use kernel::introspection::KernelInfo;
use kernel::common::cells::TakeCell;
use kernel::procs::{ProcessType, Process, FaultResponse, ProcessLoadError};
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
use kernel::procs::{FunctionCall,FunctionCallSource,Task};
use crate::vpp::mloi::VppState::RUNNING;
use core::convert::TryInto;
use crate::vpp::mailbox::mbox;
use crate::vpp::mloi::MK_SIGNAL_e::MK_SIGNAL_ERROR;

pub const DRIVER_NUM: usize = 0x90015 ;
pub const NUM_PROCS: usize = 4 ; // Number of allowed vpp processes.

pub struct VppKernel{
    pub(crate) vpp_processes: &'static [Option<VppProcess>;NUM_PROCS],
    pub(crate) kernel: &'static Kernel,
    pub(crate) last_error: Cell<MK_ERROR_e>
}

impl  VppKernel {
    pub fn  new(procs: &'static [Option<VppProcess>;NUM_PROCS],
        tock_kernel: &'static Kernel) -> VppKernel{
        VppKernel {
            vpp_processes: procs,
            kernel: tock_kernel,
            last_error: Cell::new(MK_ERROR_NONE)
        }
    }
    // 1) Generic Functions
    pub (crate) fn _mk_Get_Exception(&self){
        unimplemented!()
    }
    /// Get the last error generated through the execution of any function within a the Kernel
    /// This function retrieves an error stored by the kernel. The access to the last error
    /// is always possible for a Process and any of its descendants regardless of its state
    /// and is persisten during state transitions.
    pub (crate) fn _mk_Get_Error(&self) {
        self.last_error.get();
    }
    /// Get the absolute time (in ticks) since the Primary Platfrom start up
    /// The return value is  bits in length.
    pub (crate) fn _mk_Get_Time(&self) -> MK_TIME_t {
        unimplemented!()
    }

    // 2) Process Management

    /// Helper function to get a reference to a valid process based on a handle. It returns
    /// `None` if the process is in dead state or if the handle is not found.
    pub (crate) fn get_process_ref_internal(&self, handle: MK_HANDLE_t) -> Option<&VppProcess> {
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
                            return_pointer =  None;
                        }
                        // if the Process in any other state, a pointer to
                        // that process is delivered with a success flag and
                        // break of the loop
                        else {
                            self.last_error.set(MK_ERROR_e::MK_ERROR_NONE);
                            return_pointer = Some(proc);
                            break;
                        }
                    }
                    else {
                        self.last_error.set(MK_ERROR_UNKNOWN_ID);
                        return_pointer = None;
                    }
                }

                None => {
                    self.last_error.set(MK_ERROR_UNKNOWN_ID);
                    return_pointer = None;
                }
            }
        }
        return_pointer
        /* Leaving this buggy implementation here. It might be helpful.

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

         */
    }

    pub (crate)  fn _mk_get_process_handle(& self, _eProcess_ID: MK_Process_ID_u)
                                           -> MK_HANDLE_t {
        let handle = convert_to_handle(_eProcess_ID);
        let process =self.get_process_ref_internal(handle);
        if process.is_some() {handle}  else { 0 }
        // there is a problem when returning 0 as a handle. This might be in fact
        // the id of another handle. Whether, a Process ID as 0 is not allowed
        // or wrap this with an Option.
    }


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

    // Concerning Priorities there is another missing function that needs to be implemented.
    // Based on the index of Tock Processes, Vpp Priorities are mapped accordingly.
    pub (crate) fn _mk_set_process_priority(&self, _hProcess: MK_HANDLE_t,_xPriority: MK_PROCESS_PRIORITY_e) -> MK_ERROR_e {

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
            process.suspend_vpp_process();
            process.tockprocess.unwrap().stop();
            MK_ERROR_NONE
        } else {
            MK_ERROR_UNKNOWN_HANDLE
        }
    }

    pub (crate) fn _mk_resume_process(&self, mut _hProcess: MK_HANDLE_t) -> MK_ERROR_e {
        let vppprocess = self.get_process_ref_internal(_hProcess);
        if vppprocess.is_some() {
            let process = vppprocess.unwrap();
            process.resume_vpp_process();
            process.tockprocess.unwrap().resume();
            // this is totally wrong !!!
            // process.vppstate.set(RUNNING);
            MK_ERROR_NONE
        } else {
            MK_ERROR_UNKNOWN_HANDLE
        }
    }

    pub (crate) fn _mk_Commit(&self) {
        unimplemented!()
    }

    pub (crate) fn _mk_Rollback(&self) {
        unimplemented!()
    }

    pub (crate) fn  _mk_yield (&self,_hProcess: MK_HANDLE_t) {
        // Change state of VppState
        let vpp_process = self.get_process_ref_internal(_hProcess);
        if vpp_process.is_some(){
            vpp_process.unwrap().yield_vpp_process();
            // Change Tock State
            vpp_process.unwrap().tockprocess.unwrap().set_yielded_state();
            vpp_process.unwrap().snyc_tock_vpp_states();
            debug!("VPP Process state {:?}", vpp_process.unwrap().get_vpp_state());

        }
    }

    // 3) Mailbox Management
    ///// Helper function to get a reference to a valid Mailbox  based on a handle. It returns
    ///// `None` if the handle is not found.
   /* pub fn Get_Mailbox_ref_internal(&self, handle: MK_HANDLE_t) -> Option<&mbox> {
        let MailboxID = convert_handle_to_mbid(handle);
        let mut return_pointer: Option<&mbox> = None ;
        for process in self.vpp_processes.iter() {
            for mailbox in process.as_ref().unwrap().m_xKernel_Mailbox.iter() {
                if mailbox.get_mb_id() == MailboxID {
                    self.last_error.set(MK_ERROR_NONE);
                    return_pointer = Some(mailbox);
                    break;
                } else {
                    self.last_error.set(MK_ERROR_UNKNOWN_ID);
                    return_pointer = None;
                }
            }
        }
        return_pointer
    }
    /// Get a Mailbox Handle from a Mailbox identifier
    pub fn _mk_Get_Mailbox_Handle(&self,_eMailboxID: MK_MAILBOX_ID_u) -> Option<MK_HANDLE_t>{
        // Missing access control if the Process is not allowed to send a Signal
        // to a Mailbox. Needed a caller id and check if that id is the same in the mailbox struct
        // leave for later
        // ACCESS_DENIED if caller Process is not defined as the sender Process of the Mailbox
        let handle = convert_mbid_to_handle(_eMailboxID);
        let mailbox = self.Get_Mailbox_ref_internal(handle);
        // None is the equivalent of NULL in rust, that is why i am wrapping this with
        // the Option Box
        if mailbox.is_some() {Some(handle)} else {None}
    }

    /// When waiting for Signal on any Mailbox owned by the caller Process, get the Mailbox
    /// identifier of a Process that has a pending Signal.
    /// This function retrieves the identifier of a Mailbox with a pending signal when the
    /// Process waits on any Mailbox of the caller Process.
    pub fn _mk_Get_Mailbox_ID_Activated(&self)-> Option<MK_MAILBOX_ID_u>{
        unimplemented!();
    }
    /// This function sends Signals to a Mailbox. The Signals sent are represented as a bitmap
    /// of Signal values and there is no priority among Signals as to the order of their arrival
    /// within the Mailbox.
    pub fn _mk_Send_Signal(&self,_hMailbox: MK_HANDLE_t,_eSignal: MK_SIGNAL_e) ->  MK_ERROR_e{
       /* let mailbox = self.Get_Mailbox_ref_internal(_hMailbox);

        if mailbox.is_some() {
            mailbox.unwrap().add_sig(_eSignal);
            MK_ERROR_NONE
        } else
        {MK_ERROR_UNKNOWN_HANDLE }
        */

        unimplemented!()
    }
    /// Wait for a Signal on a Mailbox
    /// This function waits for a Signal on one or any Mailboxes of the caller Process,
    /// either for given time or without a time limit. This call is blocking
    /// and will return when a signal is received or when the timeout occurred.
    ///
    /// * When a Process waits on any Mailbox, the Signals MK_SIGNAL_TIME_OUT,
    /// MK_SIGNAL_ERROR, and MK_SIGNAL_EXCEPTION are sent only to its kernel Mailbox.
    ///
    /// * When a Process waits on a Mailbox, the Signals MK_SIGNAL_TIME_OUT,
    /// MK_SIGNAL_ERROR, and MK_SIGNAL_EXCEPTION are sent to that Mailbox.
    ///
    /// * Only the owner of the Mailbox can wait on it.
    pub fn _mk_Wait_Signal(&self, _hMailbox: MK_HANDLE_t, _uTime: u32) {
        unimplemented!()
    }

    /// Get a Signal from a Mailbox.
    /// This function gets a Signal on a Mailbox. A Process can only retrieve the Signal
    /// from its own Mailbox. The _mk_Get_Signal should be repeatedly called until 0 is returned.
    /// The pending Signals are cleared once they have been read.
    pub fn _mk_Get_Signal(&self, _hMailbox: MK_HANDLE_t) -> Option<MK_BITMAP_t> {
        /*let mailbox = self.Get_Mailbox_ref_internal(_hMailbox);
        if mailbox.is_some() {
            let sig = mailbox.unwrap().retrieve_last_sig();
            Some(sig)
        } else {
            // This is a problem => self.last_error.set(MK_SIGNAL_ERROR);
            None
        }*/
    unimplemented!()
    }
    pub (crate) fn handle_signals(&self,_eSignal:MK_BITMAP_t){

    unimplemented!()
    }
*/
    // 4) IPC Management
    // 5) VRE Management
    // 6) Firmware Management
}

// Implementing a Syscall Driver for Vpp Process Manager (VPM)
/*
pub struct VPMDriver {
    vpm: &'static VppKernel<Capability>,
    // apps: Grant<Option<Callback>>,
}
impl VPMDriver{
    pub fn new( vpm:  &'static VppKernel<Capability>) -> VPMDriver{
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
                    let error_result = self.vpm._mk_suspend_process(handle);
                    let ret = MK_ERROR_e::into(error_result);
                    self.vpm._mk_Get_Error();
                    debug!("Last Error {:?}", self.vpm.last_error.get());
                    ReturnCode::SuccessWithValue {
                        value: ret
                    }
                },
            2 =>
                {
                    debug!("Resuming Process");
                    let handle = convert_to_handle(data as u16);
                    let error_result = self.vpm._mk_resume_process(handle);
                    let ret = MK_ERROR_e::into(error_result);
                    self.vpm._mk_Get_Error();
                    ReturnCode::SuccessWithValue {
                        value: ret
                    }
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
                    //process.unwrap().sync_vpp_tock_states();
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


*/