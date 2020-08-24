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
use crate::vpp::vppprocess_v2::*;
use crate::vpp::vppprocess_v2;
use kernel::Kernel;
use kernel::capabilities::ProcessManagementCapability;
use kernel::debug;


/// Global Variable that retrieves the last error generated
/// by a Process (useful for _mk_Get_Error)
static mut LAST_ERR: MK_ERROR_e = MK_ERROR_NONE;

pub struct VppProcessManager <C: ProcessManagementCapability>{
    vpp_processes: &'static  [VppProcess],
    kernel: &'static Kernel,
    capability: C,
}

impl <C: ProcessManagementCapability> VppProcessManager<C> {
    // pub fn new(vpp_processes:&'static [Option<&'static dyn process_vpp::VppProcessType>])
    //     ->  VppProcessManager {
    //     VppProcessManager {
    //         vpp_processes: vpp_processes,
    //     }
    // }

    pub fn new(
        vpp_processes:&'static [VppProcess;2],
        kernel: &'static Kernel,
        capability : C)
               -> VppProcessManager<C> {
        VppProcessManager {
            vpp_processes,
            kernel,
            capability

        }
    }
    // pub unsafe fn _mk_Get_Error() {
    //    debug!("LAST_ERR is  {:?}", LAST_ERR )
    //    // println!("LAST ERR IS {:?}", LAST_ERR);
    // }


    // pub fn get_process_ref_interal(&self, handle: MK_HANDLE_t) -> (MK_ERROR_e,&VppProcess ){
    pub unsafe fn get_process_ref_interal(&self, handle: MK_HANDLE_t)
                                          -> Option<&VppProcess> {
        //println!("get_process_ref_started");
        let id = convert_to_id(handle);

        self.vpp_processes.iter().find_map(|proc| {
            if proc.get_vpp_id() == id {
                if proc.get_vpp_state() == VppState::DEAD {
                    LAST_ERR = MK_ERROR_UNKNOWN_ID ;
                }
                LAST_ERR = MK_ERROR_NONE ;
                Some(proc)

            } else {
                LAST_ERR = MK_ERROR_UNKNOWN_ID ;
                None
            }
        })


    }


    // pub(crate) fn handle_is_valid(&self, handle: MK_HANDLE_t) -> bool {
    //     self.vpp_processes.get(handle as usize)
    //         .map_or(false, |p| {
    //             if p.get_vpp_handle() != handle {
    //                 false
    //             } else {
    //                 true
    //             }
    //         })
    // }

    pub (crate) unsafe fn _mk_get_process_handle(&self, _eProcess_ID: MK_PROCESS_ID_u)
                                                 -> MK_HANDLE_t {
        let handle = convert_to_handle(_eProcess_ID);
        let process =self.get_process_ref_interal(handle);
        if process.is_some() {
            handle
        } else {
            0
        }
    }

    unsafe fn _mk_get_process_priority(&self, _hProcess: MK_HANDLE_t) -> MK_PROCESS_PRIORITY_e {

        let process = self.get_process_ref_interal(_hProcess);
        if process.is_some(){
            process.unwrap().get_vpp_priority()
        }
        else {
            MK_PROCESS_PRIORITY_ERROR
        }

    }


    unsafe fn _mk_set_process_priority(&self, _hProcess: MK_HANDLE_t, _xPriority: MK_PROCESS_PRIORITY_e)
                                       -> MK_ERROR_e {


        // How to check for UNKNOWN PRIORITY ???
        let process =self.get_process_ref_interal(_hProcess);
        if process.is_some(){
            process.unwrap().set_vpp_priority(_xPriority);
            return MK_ERROR_NONE
        }
        else {
            return MK_ERROR_UNKNOWN_HANDLE

        }


        // match _xPriority {
        //     // check for the index of the PROCESSES ARRAY and change accordingly
        //     MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_LOW => {
        //         // _hProcess.tockprocess.appid.index changes
        //
        //     }
        //     MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_NORMAL => {
        //
        //
        //     }
        //     MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_HIGH => {
        //
        //
        //     }
        //     MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_ERROR => {
        //         MK_ERROR_UNKNOWN_PRIORITY
        //         // Is this the right use case ??
        //
        //     }
        //
        // }


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




    }
    pub unsafe fn _mk_suspend_process(&self, mut _hProcess: MK_HANDLE_t) -> MK_ERROR_e {
        // if  self.handle_is_valid(_hProcess) == false  {
        //     return MK_ERROR_UNKNOWN_HANDLE
        // }
        let process = self.get_process_ref_interal(_hProcess);
        process.unwrap().suspend_vpp_process();

        self.kernel.process_each_capability(
            &self.capability,
            |proc| {
                let vppproc_name = process.unwrap().tockprocess.unwrap().get_process_name();
                if vppproc_name ==   proc.get_process_name() {
                    process.unwrap().tockprocess.unwrap().stop();
                    debug!("Process {} Suspended", vppproc_name);
                }
            }

        );



        MK_ERROR_NONE

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

    }


}
