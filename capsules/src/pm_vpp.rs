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
use kernel::capabilities::ProcessManagementCapability;
use kernel::common::cells::TakeCell;
use kernel::{debug, AppId};
use kernel::hil::uart;
use kernel::introspection::KernelInfo;
use kernel::Kernel;
use kernel::ReturnCode;
use kernel::procs::{State, ProcessType, Process};
use crate::mloi::MK_ERROR_e;
use crate::mloi::MK_ERROR_e::{MK_ERROR_NONE, MK_ERROR_UNKNOWN_HANDLE, MK_ERROR_ACCESS_DENIED};
use crate::mloi::*;

pub struct VppProcess<'a> {
    process: &'a mut dyn ProcessType,
    vppstate:  VppState,
}
pub struct VppProcessManager<'a, C: ProcessManagementCapability> {
    kernel: &'static Kernel,
    vpp_process: VppProcess<'a>,
    capability: C,
}

impl<'a,C: ProcessManagementCapability> VppProcessManager<'a,C> {
    // pub fn new<'a>(
    //     kernel: &'static Kernel,
    //     vpp_process: VppProcess<'a,C>,
    //     capability: C ) ->
    //     VppProcessManager<C> {
    //     VppProcessManager {
    //         kernel: kernel,
    //         vpp_process: VppProcess { process: &(), vppstate: VppState::READY },
    //         capability: capability,
    //     }
    // }
    pub fn start(&self) -> ReturnCode {
        debug!("Starting process console");
        ReturnCode::SUCCESS
    }
    /// # Brief:
    /// Get the Process kernel Handle for itself or for one of its descendants
    /// # Description:
    /// This function gets a Process kernel Handle through its Process identifier.
    /// The process retrieving the Process Handle does not inherit the rights of its owner.
    /// # Parameter:
    /// _eProcess_ID   (_MK_PROCESS_ID_u) identifier of the Process
    fn _mk_get_process_handle(&self, _eProcess_ID: AppId) -> MK_ERROR_e {
            unimplemented!();
        }

    fn _mk_get_process_priority(&self){unimplemented!();}
    fn _mk_set_process_priority(&self){unimplemented!();}

    /// # Brief:
    /// Suspend a Process. A Process can suspend itself or any of its descendants.
    /// # Description:
    /// This function suspends a Process. The suspended Process is no longer scheduled
    /// for execution. If a Process suspends itself, then this call will only return
    /// upon resumption by the Parent Process.
    /// # Parameter:
    /// _hProcess   (_MK_HANDLE_t)  Handle of the Process to be suspended

     fn _mk_suspend_process(&self, mut _hProcess: VppProcess) -> MK_ERROR_e {
        //Check if the Process Handle is valid
        // if _hProcess.process.
        // MK_ERROR_UNKNOWN_HANDLE

        // Check if the Process is not itself or any of its descendants
        // MK_ERROR_ACCESS_DENIED

        match _hProcess.vppstate {
            VppState::READY => _hProcess.vppstate = VppState::SUSPENDED_R,
            VppState::RUNNING => _hProcess.vppstate = VppState::SUSPENDED_R,
            VppState::WAITING => _hProcess.vppstate = VppState::SUSPENDED_W,
            VppState::SYNC => _hProcess.vppstate = VppState::SUSPENDED_S,
            _ => {}
        }
        let proc_name = _hProcess.process.get_process_name();
        _hProcess.process.stop();
        debug!("Process {} Suspended", proc_name);

        // self.kernel.process_each_capability(
        //         &self.capability,
        //         |_hProcess| {
        //             let proc_name = _hProcess.get_process_name();
        //             _hProcess.tock_pointer.stop();
        //             debug!("Process {} Suspended", proc_name);
        //         }
        //  );
        MK_ERROR_NONE
    }
    /// # Brief:
    /// Resume a Process
    /// # Description:
    /// This function resumes a Process. A resumed Process must be a descendant of
    /// the running Process
    /// # Parameter:
    /// _hProcess   (_MK_HANDLE_t)  Handle of the Process to be suspended

    fn _mk_resume_process(&self, mut _hProcess: VppProcess ) -> MK_ERROR_e {
        //Check if the Process Handle is valid
        // if _hProcess.process.
        // MK_ERROR_UNKNOWN_HANDLE

        // Check if the Process is not itself or any of its descendants
        // MK_ERROR_ACCESS_DENIED

        match _hProcess.vppstate {
            VppState::SUSPENDED_R => _hProcess.vppstate = VppState::READY,
            VppState::SUSPENDED_W => _hProcess.vppstate = VppState::WAITING,
            VppState::SUSPENDED_S => _hProcess.vppstate = VppState::SYNC,
            _ => {},
        }
        let proc_name = _hProcess.process.get_process_name();
        _hProcess.process.resume();
        debug!("Process {} Resumed", proc_name);

        MK_ERROR_NONE
    }
    fn _mk_commit(&self){unimplemented!();}
    fn _mk_rollback(&self){unimplemented!();}
    /// # Brief:
    /// Return the control to the kernel scheduler.
    /// # Description:
    /// Let the caller Process ask the kernel to yield its execution, causing the kernel
    /// to switch the caller to "Ready" State. This call will return when the Process is
    /// scheduled to run by the scheduler.
    /// # Parameter:
    /// void  None
    fn _mk_yield(&self){

        self.kernel.process_each_capability(
            &self.capability,
            |process| {
                process.set_yielded_state();
                debug!("Process yielded");
            }
        )
    }
    // Process the command in the command buffer and clear the buffer.
    // fn read_command(&self) {
    //     self.command_buffer.map(|command| {
    //         let mut terminator = 0;
    //         let len = command.len();
    //         for i in 0..len {
    //             if command[i] == 0 {
    //                 terminator = i;
    //                 break;
    //             }
    //         }
    //         //debug!("Command: {}-{} {:?}", start, terminator, command);
    //         // A command is valid only if it starts inside the buffer,
    //         // ends before the beginning of the buffer, and ends after
    //         // it starts.
    //         if terminator > 0 {
    //             let cmd_str = str::from_utf8(&command[0..terminator]);
    //             match cmd_str {
    //                 Ok(s) => {
    //                     let clean_str = s.trim();
    //                     if clean_str.starts_with("help") {
    //                         debug!("Welcome to the process console.");
    //                         debug!("Valid commands are: help status list stop start fault");
    //                     } else if clean_str.starts_with("start") {
    //                         let argument = clean_str.split_whitespace().nth(1);
    //                         argument.map(|name| {
    //                             self.kernel.process_each_capability(
    //                                 &self.capability,
    //                                 |proc| {
    //                                     let proc_name = proc.get_process_name();
    //                                     if proc_name == name {
    //                                         proc.resume();
    //                                         debug!("Process {} resumed.", name);
    //                                     }
    //                                 },
    //                             );
    //                         });
    //
    //                         // } else if clean_str.starts_with("stop") {
    //                         //     let argument = clean_str.split_whitespace().nth(1);
    //                         //     argument.map(|name| {
    //                         //         self.kernel.process_each_capability(
    //                         //             &self.capability,
    //                         //             |proc| {
    //                         //                 let proc_name = proc.get_process_name();
    //                         //                 if proc_name == name {
    //                         //                     proc.stop();
    //                         //                     debug!("Process {} stopped", proc_name);
    //                         //                 }
    //                         //             },
    //                         //         );
    //                         //     });
    //                     } else if clean_str.starts_with("fault") {
    //                         let argument = clean_str.split_whitespace().nth(1);
    //                         argument.map(|name| {
    //                             self.kernel.process_each_capability(
    //                                 &self.capability,
    //                                 |proc| {
    //                                     let proc_name = proc.get_process_name();
    //                                     if proc_name == name {
    //                                         proc.set_fault_state();
    //                                         debug!("Process {} now faulted", proc_name);
    //                                     }
    //                                 },
    //                             );
    //                         });
    //                     } else if clean_str.starts_with("list") {
    //                         debug!(" PID    Name                Quanta  Syscalls  Dropped Callbacks  Restarts    State  Grants");
    //                         self.kernel
    //                             .process_each_capability(&self.capability, |proc| {
    //                                 let info: KernelInfo = KernelInfo::new(self.kernel);
    //
    //                                 let pname = proc.get_process_name();
    //                                 let appid = proc.appid();
    //                                 let (grants_used, grants_total) = info.number_app_grant_uses(appid, &self.capability);
    //
    //                                 debug!(
    //                                     "  {:?}\t{:<20}{:6}{:10}{:19}{:10}  {:?}{:5}/{}",
    //                                     appid,
    //                                     pname,
    //                                     proc.debug_timeslice_expiration_count(),
    //                                     proc.debug_syscall_count(),
    //                                     proc.debug_dropped_callback_count(),
    //                                     proc.get_restart_count(),
    //                                     proc.get_state(),
    //                                     grants_used,
    //                                     grants_total
    //                                 );
    //                             });
    //                     } else if clean_str.starts_with("status") {
    //                         let info: KernelInfo = KernelInfo::new(self.kernel);
    //                         debug!(
    //                             "Total processes: {}",
    //                             info.number_loaded_processes(&self.capability)
    //                         );
    //                         debug!(
    //                             "Active processes: {}",
    //                             info.number_active_processes(&self.capability)
    //                         );
    //                         debug!(
    //                             "Timeslice expirations: {}",
    //                             info.timeslice_expirations(&self.capability)
    //                         );
    //                     } else if clean_str.starts_with("Unstart") {
    //                         let argument = clean_str.split_whitespace().nth(1);
    //                         argument.map(|name| {
    //                             //let manager=VppProcessManager::new();
    //                             self.kernel.process_each_capability(
    //                                 &self.capability,
    //                                 |proc| {
    //                                     let proc_name = proc.get_process_name();
    //                                     if proc_name == name {
    //                                         proc.set_state(State::Unstarted);
    //                                         // let handle= MK_HANDLE_t{ process: proc };
    //                                         // manager._mk_suspend_process(handle);
    //
    //                                         debug!("Process {} Unstarted", proc_name);
    //                                     }
    //                                 },
    //                             );
    //                         });
    //                     } else if clean_str.starts_with("test") {
    //                         let argument = clean_str.split_whitespace().nth(1);
    //                         argument.map(|name| {
    //                             self.kernel.process_each_capability(
    //                                 &self.capability,
    //                                 |proc| {
    //                                     let proc_name = proc.get_process_name();
    //                                     if proc_name == name {
    //                                         //proc.set_state(State::Unstarted);
    //                                         // let handle= MK_HANDLE_t{ process: proc };
    //                                         // manager._mk_suspend_process(handle);
    //                                         self._mk_suspend_process(proc);
    //                                         debug!("Process {} Suspended", proc_name);
    //                                     }
    //                                 },
    //                             );
    //                         });
    //                     } else {
    //                         debug!("Valid commands are: help status list stop start fault");
    //                     }
    //                 }
    //                 Err(_e) => debug!("Invalid command: {:?}", command),
    //             }
    //         }
    //     });
    //     self.command_buffer.map(|command| {
    //         command[0] = 0;
    //     });
    //     self.command_index.set(0);
    // }
}
