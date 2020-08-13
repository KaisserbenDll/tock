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
use crate::mloi::MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_ERROR;

pub struct VppProcess<'a> {
    process: &'a mut dyn ProcessType,
    vppstate:  VppState,
    priority: MK_PROCESS_PRIORITY_e,
    id: u16,

}

pub struct VppProcessManager<'a, C: ProcessManagementCapability> {
    kernel: &'static Kernel,
    vpp_process: VppProcess<'a>, //array of PROCESSES [&P1, &P2]
    vpp_process_ids: u32, //[7544, 8512]
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

    // fn get_process_reference_internal (MK_HANDLE)
    // iterate the array of the PROCESSES
    // lookup for MK_HANDLE id
    // when match gives
    // if the first 16 bits of the MK_HANDLE matches the id of any process


    fn _mk_get_process_handle(&self, _eProcess_ID: u16) -> & VppProcess<'a> {
        let appid = self.vpp_process.process.appid();
        // if appid =  _eProcess_ID {
        //     self.vpp_process
        // }


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
        -> MK_ERROR_e{
        // Depending of the Scheduler Type ? How can this be implemented.
        // Based on the index on the PROCESSES Array, priorities can be defined
        // index 0: MK_PROCESS_PRIORITY_HIGH
        // index 1: MK_PROCESS_PRIORITY_NORMAL
        // index 2: MK_PROCESS_PRIORITY_LOW
        //
        match _hProcess.process.appid().index() {

            _ => {}
        }



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

    fn _mk_suspend_process(&self, mut _hProcess: VppProcess) -> MK_ERROR_e {
        //get reference internal => VPP Process

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
            VppState::SUSPENDED_R => _hProcess.vppstate = VppState::READY,
            VppState::SUSPENDED_W => _hProcess.vppstate = VppState::WAITING,
            VppState::SUSPENDED_S => _hProcess.vppstate = VppState::SYNC,
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
            VppState::RUNNING => self.vpp_process.vppstate = VppState::READY,
            _ => {},
        }
        let proc_name = self.vpp_process.process.get_process_name();
        self.vpp_process.process.set_yielded_state();
        debug!("Process {} Resumed", proc_name);
    }

}
