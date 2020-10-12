#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::vpp::mloi::*;
use core::cell::Cell;
use crate::vpp::mloi::VppState::*;
use kernel::common::RingBuffer;

pub struct vnp {}
#[derive(Clone)]
pub struct IPC {
    mailbox_id: Cell<MK_MAILBOX_ID_u>,
    send_queue:RingBuffer<'static, vnp>,
    recieve_queue: RingBuffer<'static, vnp> ,
}

impl  Mailbox{
    pub fn new(&self, sq: RingBuffer<'static, vnp>, rq: RingBuffer<'static, vnp>)
               -> Mailbox {
        Mailbox {
            mailbox_id: Cell::new(0),
            send_queue: RingBuffer::new(sq),
            recieve_queue: RingBuffer::new(rq),
        }
    }
}
pub struct IPCManager {
    mailboxes: Option<[Mailbox]>,
}
impl IPCManager {
    pub fn new(mailboxes: Option<[Mailbox]>) -> IPCManager {
        IPCManager  {
            mailboxes: mailboxes,
        }
    }
}

