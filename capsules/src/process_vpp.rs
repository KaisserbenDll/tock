use kernel::{debug, AppId};
use kernel::procs::{State, ProcessType, Process};
use crate::mloi::*;
use core::cell::Cell;
use crate::mloi::VppState::*;

pub trait VppProcessType {
    fn get_vpp_id(&self) -> MK_IPC_ID_u ;

    fn get_vpp_state(&self) -> VppState ;

    fn get_vpp_priority (&self) -> MK_PROCESS_PRIORITY_e;

    fn set_vpp_priority(&self, MK_PROCESS_PRIORITY_e);

    fn suspend_vpp_process(&self) ;

    fn resume_vpp_process(&self) ;

    fn yield_vpp_process(&self) ;

    fn get_tock_process_ref(&self) -> &dyn ProcessType;
}

pub struct VppProcess<'a> {
    pub(crate) tockprocess: &'static dyn ProcessType,
    pub(crate) vppstate: Cell<VppState>,
    pub(crate) vpppriority: Cell<MK_PROCESS_PRIORITY_e>,
    pub(crate) vppid: Cell<MK_IPC_ID_u>,
}

impl VppProcessType for VppProcess<'_> {
    fn get_vpp_id(&self) -> MK_IPC_ID_u {
        self.vppid.get()
    }

    fn  get_vpp_state(&self) -> VppState {
        self.vppstate.get()
    }

    fn get_vpp_priority (&self) -> MK_PROCESS_PRIORITY_e {
        self.vpppriority.get()
    }

    fn set_vpp_priority(&self, prio:MK_PROCESS_PRIORITY_e ) {
        self.vpppriority.set(prio)
    }

    fn suspend_vpp_process(&self) {
        match self.vppstate.get() {
            VppState::READY    => self.vppstate.set(SUSPENDED_R),
            VppState::RUNNING  => self.vppstate.set(SUSPENDED_R),
            VppState::WAITING  => self.vppstate.set(SUSPENDED_W),
            VppState::SYNC     => self.vppstate.set(SUSPENDED_S),
            _                  => {},
        }
    }

    fn resume_vpp_process(&self) {
        match self.vppstate.get() {
            VppState::SUSPENDED_R => self.vppstate.set(READY),
            VppState::SUSPENDED_W => self.vppstate.set(WAITING),
            VppState::SUSPENDED_S => self.vppstate.set(SYNC),
            _                     => {},

        }
    }

    fn yield_vpp_process(&self) {
        match self.vppstate.get() {
            VppState::RUNNING => self.vppstate.set(READY),
            _                 => {},

        }
    }

    fn get_tock_process_ref(&self) -> &dyn ProcessType {
        unimplemented!()
    }
}



