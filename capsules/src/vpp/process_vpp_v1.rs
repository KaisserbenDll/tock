#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kernel::{debug, AppId};
use kernel::procs::{State, ProcessType, Process};
use crate::mloi::*;
use core::cell::Cell;
use crate::mloi::VppState::*;
use crate::mloi::MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_LOW;

type MK_Process_ID_u = u16 ;

pub struct VppProcess {
    pub(crate) tockprocess: Option<&'static dyn ProcessType>,
    pub(crate) vppstate: Cell<VppState>,
    pub(crate) vpppriority: Cell<MK_PROCESS_PRIORITY_e>,
    pub(crate) vppid: Cell<MK_Process_ID_u>,
}

pub trait VppProcessType {
    fn get_vpp_id(&self) -> MK_Process_ID_u ;

    fn get_vpp_state(&self) -> VppState ;

    fn get_vpp_priority (&self) -> MK_PROCESS_PRIORITY_e;

    fn set_vpp_priority(&self, prio: MK_PROCESS_PRIORITY_e);

    fn suspend_vpp_process(&self) ;

    fn resume_vpp_process(&self) ;

    fn yield_vpp_process(&self) ;

    fn get_tock_process_ref(&self) -> &dyn ProcessType;
}



impl VppProcessType for VppProcess {

    fn get_vpp_id(&self) -> MK_Process_ID_u {
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


// impl VppProcess {
//     pub(crate) unsafe fn create_vpp_process(tockprocess:Option<&'static dyn ProcessType>)
//         -> Option<&'static dyn VppProcessType> {
//         Some(&VppProcess{
//             tockprocess: tockprocess,
//             vppstate: Cell::new(READY),
//             vpppriority: Cell::new(MK_PROCESS_PRIORITY_LOW),
//             vppid: Cell::new(1),
//         })
//     }
// }

impl VppProcess {
    pub(crate) fn create_vpp_process(tockprocess: Option<&'static dyn ProcessType>)
                                     -> VppProcess {
        VppProcess{
            tockprocess: tockprocess,
            vppstate: Cell::new(READY),
            vpppriority: Cell::new(MK_PROCESS_PRIORITY_LOW),
            vppid: Cell::new(1),
        }
    }
}
