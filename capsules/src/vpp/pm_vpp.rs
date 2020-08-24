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
use crate::mloi::MK_ERROR_e::*;
use crate::mloi::MK_PROCESS_PRIORITY_e::*;
use crate::mloi::*;
use crate::process_vpp::*;
use crate::process_vpp;

/// Global Variable that retrieves the last error generated
/// by a Process (useful for _mk_Get_Error)
static mut LAST_ERR: MK_ERROR_e = MK_ERROR_NONE;

// To change LAST_ERR use unsafe block
pub type MK_Process_ID_u = u16 ;
pub(crate) fn convert_to_handle(id: MK_PROCESS_ID_u) -> MK_HANDLE_t{
    id as u32
}
pub(crate) fn convert_to_id(handle: MK_HANDLE_t) -> MK_PROCESS_ID_u{
    handle as u16
}

pub struct VppProcessManager< C: ProcessManagementCapability> {
    kernel: &'static Kernel,
    vpp_processes: &'static [Option<&'static dyn process_vpp::VppProcessType>], //array of PROCESSES [&P1, &P2]
    capability: C,
}

impl<C: ProcessManagementCapability> VppProcessManager<C> {
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

    // /// Testing Function
    // pub fn start(&self) -> MK_ERROR_e {
    //     debug!("Starting process console");
    //     MK_ERROR_e::MK_ERROR_NONE
    // }

    /// Get the process pointer based on an MK_Handle and return an error type
    ///  Not Needed.
    /// Returns a `MK_ERROR_e` if the ID does not exist or the process
    /// is in the dead state.
    pub fn get_process_ref_interal(&self, handle: MK_HANDLE_t) -> (MK_ERROR_e,Option< &dyn VppProcessType> ){
        let mut process_ref_internal = None;
        let mut error :MK_ERROR_e = MK_ERROR_NONE;
        // |p: &Option <&dyn VppProcessType>|
        // |process: &dyn VppProcessType|
        self.vpp_processes.iter().find_map(|p | {
            p.map_or(None,|process | -> Option<&dyn VppProcessType>{
                let id = convert_to_id(handle);
                if process.get_vpp_state() == VppState::DEAD {
                    error = MK_ERROR_UNKNOWN_ID ;
                }
                if process.get_vpp_id() == id {
                    process_ref_internal = Some(process);
                } else {
                    error= MK_ERROR_UNKNOWN_ID;
                }
                unsafe { LAST_ERR = error ;}
                None
            })

        });

        (error,process_ref_internal)
    }
    /// Checks if the provided `id` is valid given the process stored in
    /// the processes array. Returns `true` if the id still refers to a valid
    /// process, and `false` if not.

    pub(crate) fn id_is_valid(&self, id: MK_Process_ID_u) -> bool {
        self.vpp_processes.get(id as usize).map_or(false, |p| {
            p.map_or(false, |process| process.get_vpp_id() == id  )
        })
    }


    /// # Brief:
    /// Get the Process kernel Handle for itself or for one of its descendants
    /// # Description:
    /// This function gets a Process kernel Handle through its Process identifier.
    /// The process retrieving the Process Handle does not inherit the rights of its owner.
    /// # Parameter:
    /// _eProcess_ID   (_MK_PROCESS_ID_u) identifier of the Process

    pub (crate) fn _mk_get_process_handle(&self, _eProcess_ID: MK_PROCESS_ID_u) -> MK_HANDLE_t {



            unimplemented!();
    }


    /// # Brief:
    /// Get the Process priority.
    /// # Description:
    /// This function gets the priority of a Process.
    /// # Parameter:
    /// _hProcess   (_MK_HANDLE_t)  Handle of the Process.
    fn _mk_get_process_priority(&self, _hProcess: VppProcess) -> MK_PROCESS_PRIORITY_e {
        // let priority = _hProcess.priority;
        // priority
        unimplemented!()
    }


    fn _mk_set_process_priority(&self, _hProcess: VppProcess, _xPriority: MK_PROCESS_PRIORITY_e)
        -> MK_ERROR_e {
        // Depending of the Scheduler Type ? How can this be implemented.
        // Based on the index on the PROCESSES Array, priorities can be defined
        // index 0: MK_PROCESS_PRIORITY_HIGH
        // index 1: MK_PROCESS_PRIORITY_NORMAL
        // index 2: MK_PROCESS_PRIORITY_LOW
        //
        // match _hProcess.process.appid().index() {
        //
        //     _ => {}
        // }



        MK_ERROR_e::MK_ERROR_NONE
    }

    /// # Brief:
    /// Suspend a Process. A Process can suspend itself or any of its descendants.
    /// # Description:
    /// This function suspends a Process. The suspended Process is no longer scheduled
    /// for execution. If a Process suspends itself, then this call will only return
    /// upon resumption by the Parent Process.
    /// # Parameter:
    /// _hProcess   (_MK_HANDLE_t)  Handle of the Process to be suspended

    fn _mk_suspend_process(&self, mut _hProcess: MK_HANDLE_t) -> MK_ERROR_e {
        //
        // _hProcess.suspend_vpp_process();
        // //get reference internal => VPP Process
        //
        // //Check if the Process Handle is valid
        // // if _hProcess.process.
        // // MK_ERROR_UNKNOWN_HANDLE
        //
        // // Check if the Process is not itself or any of its descendants
        // // MK_ERROR_ACCESS_DENIED
        //
        // match _hProcess.vppstate {
        //     Cell::from(VppState::READY) => _hProcess.vppstate = Cell::from(VppState::SUSPENDED_R),
        //     Cell::from(VppState::RUNNING) => _hProcess.vppstate = Cell::from(VppState::SUSPENDED_R),
        //     Cell::from(VppState::WAITING) => _hProcess.vppstate = Cell::from(VppState::SUSPENDED_W),
        //     Cell::from(VppState::SYNC) => _hProcess.vppstate = Cell::from(VppState::SUSPENDED_S),
        //     _ => {}
        // }
        // let proc_name = _hProcess.get_tock_process_ref();
        //     get_process_name();
        // _hProcess.process.stop();
        // debug!("Process {} Suspended", proc_name);
        //
        // // self.kernel.process_each_capability(
        // //         &self.capability,
        // //         |_hProcess| {
        // //             let proc_name = _hProcess.get_process_name();
        // //             _hProcess.tock_pointer.stop();
        // //             debug!("Process {} Suspended", proc_name);
        // //         }
        // //  );
        // MK_ERROR_e::MK_ERROR_NONE
        unimplemented!()
    }
    /// # Brief:
    /// Resume a Process
    /// # Description:
    /// This function resumes a Process. A resumed Process must be a descendant of
    /// the running Process
    /// # Parameter:
    /// _hProcess   (_MK_HANDLE_t)  Handle of the Process to be suspended

    fn _mk_resume_process(&self, mut _hProcess: VppProcess) -> MK_ERROR_e {
        // //Check if the Process Handle is valid
        // // if _hProcess.process.
        // // MK_ERROR_UNKNOWN_HANDLE
        //
        // // Check if the Process is not itself or any of its descendants
        // // MK_ERROR_ACCESS_DENIED
        //
        // match _hProcess.vppstate {
        //     Cell::from(VppState::SUSPENDED_R) => _hProcess.vppstate = Cell::from(VppState::READY),
        //     Cell::from(VppState::SUSPENDED_W) => _hProcess.vppstate = Cell::from(VppState::WAITING),
        //     Cell::from(VppState::SUSPENDED_S) => _hProcess.vppstate = Cell::from(VppState::SYNC),
        //     _ => {},
        // }
        // let proc_name = _hProcess.process.get_process_name();
        // _hProcess.process.resume();
        // debug!("Process {} Resumed", proc_name);
        //
        // MK_ERROR_e::MK_ERROR_NONE
        unimplemented!()
    }

    fn _mk_Request_No_Preemption(&self) { unimplemented!(); }
    fn _mk_commit(&self) { unimplemented!(); }
    fn _mk_rollback(&self) { unimplemented!(); }

    /// # Brief:
    /// Return the control to the kernel scheduler.
    /// # Description:
    /// Let the caller Process ask the kernel to yield its execution, causing the kernel
    /// to switch the caller to "Ready" State. This call will return when the Process is
    /// scheduled to run by the scheduler.
    /// # Parameter:
    /// void  None
    fn _mk_yield(&mut self) {
    //     match self.vpp_process.vppstate {
    //         Cell::from(VppState::RUNNING) => self.vpp_process.vppstate = Cell::from(VppState::READY),
    //         _ => {},
    //     }
    //     let proc_name = self.vpp_process.process.get_process_name();
    //     self.vpp_process.process.set_yielded_state();
    //     debug!("Process {} Resumed", proc_name);
    unimplemented!()
    }

}



