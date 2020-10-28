#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use crate::vpp::mloi::*;
use core::cell::Cell;
use crate::vpp::mloi::VppState::*;
use kernel::common::{RingBuffer, Queue};
use crate::vpp::mloi::MK_ERROR_e::{MK_ERROR_NONE, MK_ERROR_UNKNOWN_PRIORITY, MK_ERROR_UNKNOWN_ID};
use kernel::debug;
use kernel::common::cells::MapCell;

pub struct mbox {
    pub mailbox_id: Cell<MK_MAILBOX_ID_u>,
    owner_process_index: Cell<MK_Index_t>,
    sender_process_index: Cell<MK_Index_t>,
    pub queue: MapCell<RingBuffer<'static, MK_BITMAP_t>>,
}

impl  mbox {
    pub fn new(mb_id : MK_MAILBOX_ID_u, owner_pi: MK_Index_t, sender_pi: MK_Index_t) -> mbox {
        //static mut signals : [MK_BITMAP_t;3] = [0;3];
        //let signals_cell = RingBuffer::new(& mut signals);
        mbox {
            mailbox_id: Cell::new(mb_id),
            owner_process_index: Cell::new(owner_pi),
            sender_process_index: Cell::new(sender_pi),
            queue: MapCell::empty(),//MapCell::new(signals_cell),
        }
    }

    pub fn create_com_mb() -> mbox {
        mbox{
            mailbox_id: Cell::new(MK_MAILBOX_COM_MAIN_ID),
            owner_process_index: Cell::new(1),
            sender_process_index: Cell::new(2),
            queue: MapCell::empty()
        }
    }
    pub unsafe fn create_mgt_mb() -> mbox {
        static mut signals : [MK_BITMAP_t;10] = [0;10];
        let signals_cell = RingBuffer::new(& mut signals);
        mbox{
            mailbox_id: Cell::new(MK_MAILBOX_MGT_MAIN_ID),
            owner_process_index: Cell::new(0),
            sender_process_index: Cell::new(2),
            queue: MapCell::new(signals_cell)
        }
    }
    pub fn create_main_com_mb() -> mbox {
        mbox{
            mailbox_id: Cell::new(MK_MAILBOX_MAIN_COM_ID),
            owner_process_index: Cell::new(2),
            sender_process_index: Cell::new(1),
            queue: MapCell::empty()
        }
    }
    pub fn create_main_mgt_mb() -> mbox {
        mbox{
            mailbox_id: Cell::new(MK_MAILBOX_MAIN_MGT_ID),
            owner_process_index: Cell::new(2),
            sender_process_index: Cell::new(0),
            queue: MapCell::empty()
        }
    }
    pub (crate) fn get_mb_id(&self) -> MK_MAILBOX_ID_u {self.mailbox_id.get()}

    /// Add a signal in the queue of the mailbox and return `true` if it was successfully added
    /// otherwise `false`
    pub (crate) fn add_sig(&self, sig: MK_BITMAP_t ) -> Option<bool> {
         self.queue.map(|queue|queue.enqueue(sig))
    }
    /// Retrieve the first Signal from the top of the queue. Using this function will
    /// automatically clear the retrieved Signal.
    pub (crate) fn retrieve_last_sig(&self) -> Option<MK_BITMAP_t> {
        if self.has_elements().unwrap() {
            self.queue.map(|queue| queue.dequeue().unwrap())
        } else {
            Some(0)
            // we should not return None, instead a 0, so that the caller knows
            // that the signal queue has been read and no signal is yet sent.
        }
    }
    /// Check if the queue has elements. This is used, when _mk_Get_Signal is called
    /// when the queue is empty. If the queue is empty,aka doesn't have elements,
    /// _mk_Get_Signal shall return 0
    pub (crate) fn has_elements(&self) -> Option<bool>{
        self.queue.map(|queue| queue.has_elements())
    }

}
