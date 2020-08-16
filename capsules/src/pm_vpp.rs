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


pub struct VppProcessManager<'a, C: ProcessManagementCapability> {
    kernel: &'static Kernel,
    vpp_processes: &'static [Option<&'static dyn process_vpp::VppProcessType>], //array of PROCESSES [&P1, &P2]
    vpp_process_ids: &'static [Cell<MK_IPC_ID_u>], //[7544, 8512]
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

    /// Testing Function
    pub fn start(&self) -> MK_ERROR_e {
        debug!("Starting process console");
        MK_ERROR_e::MK_ERROR_NONE
    }

    /// Run a closure on every valid process. This will iterate the array of
    /// processes and call the closure on every process that exists.
    // pub(crate) fn process_each<F>(&self, closure: F)
    //     where
    //         F: Fn(&dyn process_vpp::VppProcessType),
    // {
    //     for vpp_process in self.vpp_processes.iter() {
    //         match vpp_process {
    //             Some(p) => {
    //                 closure(*p);
    //             }
    //             None => {}
    //         }
    //     }
    // }



    /// # Brief:
    /// Get the Process kernel Handle for itself or for one of its descendants
    /// # Description:
    /// This function gets a Process kernel Handle through its Process identifier.
    /// The process retrieving the Process Handle does not inherit the rights of its owner.
    /// # Parameter:
    /// _eProcess_ID   (_MK_PROCESS_ID_u) identifier of the Process




    pub (crate) fn _mk_get_process_handle(&self, _eProcess_ID: Box<dyn VppProcessType>) -> MK_HANDLE_t {
        // let appid = self.vpp_process.process.appid();
        // if appid =  _eProcess_ID {
        //     self.vpp_process
        // }
        // fn get_process_reference_internal (MK_HANDLE)
        // iterate the array of the PROCESSES
        // lookup for MK_HANDLE id
        // when match gives
        // if the first 16 bits of the MK_HANDLE matches the id of any process
            unimplemented!();
    }


    /// # Brief:
    /// Get the Process priority.
    /// # Description:
    /// This function gets the priority of a Process.
    /// # Parameter:
    /// _hProcess   (_MK_HANDLE_t)  Handle of the Process.
    fn _mk_get_process_priority(&self, _hProcess: VppProcess) -> MK_PROCESS_PRIORITY_e {
        let priority = _hProcess.priority;
        priority
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

    fn _mk_suspend_process(&self, mut _hProcess: Box<dyn VppProcessType>) -> MK_ERROR_e {

        _hProcess.suspend_vpp_process();
        //get reference internal => VPP Process

        //Check if the Process Handle is valid
        // if _hProcess.process.
        // MK_ERROR_UNKNOWN_HANDLE

        // Check if the Process is not itself or any of its descendants
        // MK_ERROR_ACCESS_DENIED

        match _hProcess.vppstate {
            Cell::from(VppState::READY) => _hProcess.vppstate = Cell::from(VppState::SUSPENDED_R),
            Cell::from(VppState::RUNNING) => _hProcess.vppstate = Cell::from(VppState::SUSPENDED_R),
            Cell::from(VppState::WAITING) => _hProcess.vppstate = Cell::from(VppState::SUSPENDED_W),
            Cell::from(VppState::SYNC) => _hProcess.vppstate = Cell::from(VppState::SUSPENDED_S),
            _ => {}
        }
        let proc_name = _hProcess.get_tock_process_ref()
            get_process_name();
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
        MK_ERROR_e::MK_ERROR_NONE
    }
    /// # Brief:
    /// Resume a Process
    /// # Description:
    /// This function resumes a Process. A resumed Process must be a descendant of
    /// the running Process
    /// # Parameter:
    /// _hProcess   (_MK_HANDLE_t)  Handle of the Process to be suspended

    fn _mk_resume_process(&self, mut _hProcess: VppProcess) -> MK_ERROR_e {
        //Check if the Process Handle is valid
        // if _hProcess.process.
        // MK_ERROR_UNKNOWN_HANDLE

        // Check if the Process is not itself or any of its descendants
        // MK_ERROR_ACCESS_DENIED

        match _hProcess.vppstate {
            Cell::from(VppState::SUSPENDED_R) => _hProcess.vppstate = Cell::from(VppState::READY),
            Cell::from(VppState::SUSPENDED_W) => _hProcess.vppstate = Cell::from(VppState::WAITING),
            Cell::from(VppState::SUSPENDED_S) => _hProcess.vppstate = Cell::from(VppState::SYNC),
            _ => {},
        }
        let proc_name = _hProcess.process.get_process_name();
        _hProcess.process.resume();
        debug!("Process {} Resumed", proc_name);

        MK_ERROR_e::MK_ERROR_NONE
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
        match self.vpp_process.vppstate {
            Cell::from(VppState::RUNNING) => self.vpp_process.vppstate = Cell::from(VppState::READY),
            _ => {},
        }
        let proc_name = self.vpp_process.process.get_process_name();
        self.vpp_process.process.set_yielded_state();
        debug!("Process {} Resumed", proc_name);
    }

}
