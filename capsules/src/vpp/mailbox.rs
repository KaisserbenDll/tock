#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::vpp::mloi::*;
use core::cell::Cell;
use crate::vpp::mloi::VppState::*;
use kernel::common::RingBuffer;
use crate::vpp::mloi::MK_ERROR_e::{MK_ERROR_NONE, MK_ERROR_UNKNOWN_PRIORITY, MK_ERROR_UNKNOWN_ID};

#[derive(Clone,Copy)]
pub struct vnp  {}
pub struct mbox {
    mailbox_id: Cell<MK_MAILBOX_ID_u>,
    send_queue:RingBuffer<'static, vnp>,
    recieve_queue: RingBuffer<'static, vnp> ,
    caller_process_id: Cell<MK_Process_ID_u>
}

impl  mbox {
    pub fn new(&self,
               sq_packets: &'static mut [vnp],
               rq_packets: &'static mut [vnp],
               caller_proc_id: MK_Process_ID_u )
        -> mbox {
        mbox {
            mailbox_id: Cell::new(0),
            send_queue: RingBuffer::new(sq_packets),
            recieve_queue: RingBuffer::new(rq_packets),
            caller_process_id: Cell::new(caller_proc_id)
        }
    }
    pub fn get_mb_id(&self) -> MK_MAILBOX_ID_u {self.mailbox_id.get()}
}
pub struct MailboxManager {
    mailboxes: [Option<mbox>; MK_MAILBOX_LIMIT],
    last_error: Cell<MK_ERROR_e>,
}
impl MailboxManager {
    pub fn new(mailboxes: [Option<mbox>;MK_MAILBOX_LIMIT]) -> MailboxManager {
        // fix the size of the array. Populate array with the argument and fill the rest with None.
        MailboxManager  {
            mailboxes: mailboxes,
            last_error: Cell::new(MK_ERROR_NONE)
        }
    }
    pub fn Get_Mailbox_ref_internal(&self, handle: MK_HANDLE_t) -> Option<&mbox> {
        let MailboxID = convert_handle_to_mbid(handle);
        let mut return_pointer: Option<&mbox> = None ;
        for mailbox in self.mailboxes.iter() {
            match mailbox{
                Some(mb) =>{
                    if mb.get_mb_id() == MailboxID {
                        self.last_error.set(MK_ERROR_NONE);
                        return_pointer = Some(mb);
                        break;
                    } else {
                        self.last_error.set(MK_ERROR_UNKNOWN_ID);
                        return_pointer = None;
                    }
                },
                None => {
                    self.last_error.set(MK_ERROR_UNKNOWN_ID);
                    return_pointer = None;
                }
            }
        }
        return_pointer
    }
    /// Get a Mailbox Handle from a Mailbox identifier
    pub fn _mk_Get_Mailbox_Handle(&self,_eMailboxID: MK_MAILBOX_ID_u) -> Option<MK_HANDLE_t>{
      let handle = convert_mbid_to_handle(_eMailboxID);
      let mailbox = self.Get_Mailbox_ref_internal(handle);
      // None is the equivalent of NULL in rust, that is why i am wrapping this with
        // the Option Box
        if mailbox.is_some() {Some(handle)} else {None}

    }
    /// When waiting for Signal on any Mailbox owned by the caller Process, get the Mailbox
    /// identifier of a Process that has a pending Signal.
    /// This function retrieves the identifier of a Mailbox with a pending signal when the
    /// Process waits on any Mailbox of the caller Process.
    pub fn _mk_Get_Mailbox_ID_Activated(&self)-> Option<MK_MAILBOX_ID_u>{
        unimplemented!();
    }
    /// This function sends Signals to a Mailbox. The Signals sent are represented as a bitmap
    /// of Signal values and there is no priority among Signals as to the order of their arrival
    /// within the Mailbox.
    pub fn _mk_Send_Signal(&self,_hMailbox: MK_MAILBOX_ID_u,_eSignal: MK_SIGNAL_e) ->  MK_ERROR_e{
        unimplemented!()
    }
    /// Wait for a Signal on a Mailbox
    /// This function waits for a Signal on one or any Mailboxes of the caller Process,
    /// either for given time or without a time limit. This call is blocking
    /// and will return when a signal is received or when the timeout occurred.
    ///
    /// * When a Process waits on any Mailbox, the Signals MK_SIGNAL_TIME_OUT,
    /// MK_SIGNAL_ERROR, and MK_SIGNAL_EXCEPTION are sent only to its kernel Mailbox.
    ///
    /// * When a Process waits on a Mailbox, the Signals MK_SIGNAL_TIME_OUT,
    /// MK_SIGNAL_ERROR, and MK_SIGNAL_EXCEPTION are sent to that Mailbox.
    ///
    /// * Only the owner of the Mailbox can wait on it.
    pub fn _mk_Wait_Signal(&self, _hMailbox: MK_HANDLE_t, _uTime: u32) {
        unimplemented!()
    }

    /// Get a Signal from a Mailbox.
    /// This function gets a Signal on a Mailbox. A Process can only retrieve the Signal
    /// from its own Mailbox. The _mk_Get_Signal should be repeatedly called until 0 is returned.
    /// The pending Signals are cleared once they have been read.
    pub fn _mk_Get_Signal(&self, _hMailbox: MK_HANDLE_t) -> MK_SIGNAL_e {
        unimplemented!()
    }
}

