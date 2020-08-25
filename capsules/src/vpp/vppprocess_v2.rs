
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::vpp::mloi::*;
use core::cell::Cell;
use kernel::procs::{State, ProcessType, Process};
use crate::vpp::mloi::VppState::*;
use crate::vpp::mloi::MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_LOW;

#[derive(Clone)]
pub struct VppProcess {
    pub(crate) tockprocess: Option<&'static dyn ProcessType>,
    pub(crate) vppstate: Cell<VppState>,
    pub(crate) vpppriority: Cell<MK_PROCESS_PRIORITY_e>,
    pub(crate) vppid: Cell<MK_Process_ID_u>,
}

impl  VppProcess{

    pub(crate) fn create_vpp_process() -> VppProcess{
        VppProcess {
            tockprocess: None,
            vppstate: Cell::new(VppState::READY),
            vpppriority: Cell::new(MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_LOW),
            vppid: Cell::new(0)
        }
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
        match self.vppstate.get() {
            VppState::READY    => self.vppstate.set(SUSPENDED_R),
            VppState::RUNNING  => self.vppstate.set(SUSPENDED_R),
            VppState::WAITING  => self.vppstate.set(SUSPENDED_W),
            VppState::SYNC     => self.vppstate.set(SUSPENDED_S),
            _                  => {},
        }
    }

    pub(crate) fn resume_vpp_process(&self) {
        match self.vppstate.get() {
            VppState::SUSPENDED_R => self.vppstate.set(READY),
            VppState::SUSPENDED_W => self.vppstate.set(WAITING),
            VppState::SUSPENDED_S => self.vppstate.set(SYNC),
            _                     => {},

        }
    }

    pub(crate) fn yield_vpp_process(&self) {
        match self.vppstate.get() {
            VppState::RUNNING => self.vppstate.set(READY),
            _                 => {},

        }
    }
    pub(crate) fn set_vpp_id(&self, id :MK_Process_ID_u ) {
        self.vppid.set(id);
    }

}

