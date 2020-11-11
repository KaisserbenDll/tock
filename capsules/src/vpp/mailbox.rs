#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use crate::vpp::mloi::*;
use core::cell::Cell;
use crate::vpp::mloi::VppState::*;
use crate::vpp::mloi::MK_ERROR_e::{MK_ERROR_NONE, MK_ERROR_UNKNOWN_PRIORITY, MK_ERROR_UNKNOWN_ID};
use kernel::debug;

pub struct mbox {
    mailbox_id: Cell<MK_MAILBOX_ID_u>,
    owner_process_index: Cell<MK_Index_t>,
    sender_process_index: Cell<MK_Index_t>,
    signals: Cell<MK_BITMAP_t>,
}

impl  mbox {
    pub fn new(mb_id : MK_MAILBOX_ID_u, owner_pi: MK_Index_t, sender_pi: MK_Index_t) -> mbox {
        //static mut signals : [MK_BITMAP_t;3] = [0;3];
        //let signals_cell = RingBuffer::new(& mut signals);
        mbox {
            mailbox_id: Cell::new(mb_id),
            owner_process_index: Cell::new(owner_pi),
            sender_process_index: Cell::new(sender_pi),
            signals: Cell::new(0)
        }
    }

    pub fn create_com_mb() -> mbox {
        mbox{
            mailbox_id: Cell::new(MK_MAILBOX_COM_MAIN_ID),
            owner_process_index: Cell::new(1),
            sender_process_index: Cell::new(2),
            signals: Cell::new(0)
        }
    }
    pub  fn create_mgt_mb() -> mbox {
        mbox{
            mailbox_id: Cell::new(MK_MAILBOX_MGT_MAIN_ID),
            owner_process_index: Cell::new(0),
            sender_process_index: Cell::new(2),
            signals: Cell::new(0)
        }
    }
    pub fn create_main_com_mb() -> mbox {
        mbox{
            mailbox_id: Cell::new(MK_MAILBOX_MAIN_COM_ID),
            owner_process_index: Cell::new(2),
            sender_process_index: Cell::new(1),
            signals: Cell::new(0)
        }
    }
    pub fn create_main_mgt_mb() -> mbox {
        mbox{
            mailbox_id: Cell::new(MK_MAILBOX_MAIN_MGT_ID),
            owner_process_index: Cell::new(2),
            sender_process_index: Cell::new(0),
            signals: Cell::new(0)
        }
    }
    pub (crate) fn get_mb_id(&self) -> MK_MAILBOX_ID_u {self.mailbox_id.get()}

    pub(crate) fn get_owner_proc_i (&self) -> MK_Index_t {
        self.owner_process_index.get()
    }
    pub(crate) fn get_sender_proc_i (&self) -> MK_Index_t {
        self.sender_process_index.get()
    }
    pub (crate) fn add_sig(&self, mut new_sigs_bitmap: MK_BITMAP_t ) {
        let old_sigs_bitmap = self.signals.get();
        new_sigs_bitmap |= old_sigs_bitmap ;
        self.signals.set(new_sigs_bitmap);
    }

    pub (crate) fn get_sig(&self) -> MK_BITMAP_t {
        let retrieved_sig = self.signals.get();
        //Once read, clear pending Signals from mailbox
        self.clear_sig();
        retrieved_sig
    }
    pub(crate) fn clear_sig(&self) {
        self.signals.set(0);
    }

}
