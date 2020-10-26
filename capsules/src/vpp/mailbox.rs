#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::vpp::mloi::*;
use core::cell::Cell;
use crate::vpp::mloi::VppState::*;
use kernel::common::RingBuffer;
use crate::vpp::mloi::MK_ERROR_e::{MK_ERROR_NONE, MK_ERROR_UNKNOWN_PRIORITY, MK_ERROR_UNKNOWN_ID};

//#[derive(Clone)]
pub struct mbox {
    pub mailbox_id: Cell<MK_MAILBOX_ID_u>,
    owner_process_index: Cell<usize>, //tbc
    sender_process_index: Cell<usize>,
    //queue:RingBuffer<'static, u8>,
}

impl  mbox {
    pub fn new(mb_id : MK_MAILBOX_ID_u, owner_pi: usize, sender_pi: usize) -> mbox {
        mbox {
            mailbox_id: Cell::new(mb_id),
            owner_process_index: Cell::new(owner_pi),
            sender_process_index: Cell::new(sender_pi),
            //queue: RingBuffer::new(signals),
        }
    }
    pub (crate) fn get_mb_id(&self) -> MK_MAILBOX_ID_u {self.mailbox_id.get()}

    /*pub (crate) fn add_sig(&self, sig: MK_BITMAP_t ) {
        self.queue.enqueue(sig);
    }
    pub (crate) fn retrieve_last_sig(&self) -> MK_BITMAP_t{
        self.queue.last_element.dequeue();
    }
     */
}
