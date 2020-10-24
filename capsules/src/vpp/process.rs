#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::vpp::mloi::*;
use core::cell::Cell;
use kernel::procs::{State, ProcessType, Process};
use crate::vpp::mloi::VppState::*;
use crate::vpp::mloi::MK_Process_ID_u;
use crate::vpp::mailbox::mbox;

#[derive(Clone)]
pub struct VppProcess {
    pub(crate) tockprocess: Option<&'static dyn ProcessType>,
    pub(crate) vppstate: Cell<VppState>,
    pub(crate) vpppriority: Cell<MK_PROCESS_PRIORITY_e>,
    pub(crate) vppid: Cell<MK_Process_ID_u>,
    pub(crate) m_xKernel_Mailbox: Option<&'static mbox>,
}

impl  VppProcess{
    pub fn create_vpp_process(
        tockprocess: Option<&'static dyn ProcessType>,
        pid : MK_Process_ID_u,
        mailbox  : Option<&'static mbox> )-> VppProcess{
        VppProcess {
            tockprocess: tockprocess,
            vppstate: Cell::new(VppState::SUSPENDED_R),
            vpppriority: Cell::new(MK_PROCESS_PRIORITY_e::MK_PROCESS_PRIORITY_NORMAL),
            vppid: Cell::new(pid),
            m_xKernel_Mailbox: mailbox
        }
    }

    pub(crate) fn snyc_tock_vpp_states(&self){
        let tock_state = self.tockprocess.unwrap().get_state();
        match tock_state {
            State::Unstarted => self.vppstate.set(VppState::READY),
            State::Yielded => self.vppstate.set(VppState::READY),
            State::Running => self.vppstate.set(VppState::RUNNING),
            State::StoppedYielded => self.vppstate.set(VppState::SUSPENDED_R),
            State::StoppedRunning => self.vppstate.set(VppState::SUSPENDED_R),
            State::StoppedFaulted => self.vppstate.set(VppState::DEAD),
            State::Fault => self.vppstate.set(VppState::DEAD),
        }
    }
     pub(crate) fn sync_vpp_tock_states(&self) {
        let vpp_state = self.vppstate.get();
        let tock_process = self.tockprocess.unwrap()  ;
        match vpp_state {
            VppState::READY =>  tock_process.set_state(State::Yielded)  ,
            VppState::RUNNING => tock_process.set_state(State::Running),
            VppState::SUSPENDED_R => tock_process.set_state(State::StoppedYielded),
            VppState::DEAD => tock_process.set_state(State::StoppedFaulted),
            _ => {},
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
        self.snyc_tock_vpp_states();
        match self.vppstate.get() {
            VppState::READY    => self.vppstate.set(SUSPENDED_R),
            VppState::RUNNING  => self.vppstate.set(SUSPENDED_R),
            VppState::WAITING  => self.vppstate.set(SUSPENDED_W),
            VppState::SYNC     => self.vppstate.set(SUSPENDED_S),
            _                  => {},
        }
    }

    pub(crate) fn resume_vpp_process(&self) {
        self.snyc_tock_vpp_states();
        match self.vppstate.get() {
            VppState::SUSPENDED_R => self.vppstate.set(READY),
            VppState::SUSPENDED_W => self.vppstate.set(WAITING),
            VppState::SUSPENDED_S => self.vppstate.set(SYNC),
            _                     => {},
        }
    }

    pub(crate) fn yield_vpp_process(&self) {
        self.snyc_tock_vpp_states();
        match self.vppstate.get() {
            VppState::RUNNING => self.vppstate.set(READY),
            _                 => {},
        }
    }

    pub(crate) fn set_vpp_id(&self, id :MK_Process_ID_u ) {
        self.vppid.set(id);
    }

}

