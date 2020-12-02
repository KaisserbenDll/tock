#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_must_use)]
use crate::vpp::mloi::*;
use core::cell::Cell;
use crate::vpp::mloi::VppState::*;
use crate::vpp::mloi::MK_ERROR_e::{MK_ERROR_NONE, MK_ERROR_UNKNOWN_PRIORITY, MK_ERROR_UNKNOWN_ID};
use kernel::{debug, Grant, Callback, Kernel, AppId};
use kernel::{create_capability};
use kernel::capabilities::{ProcessManagementCapability, MemoryAllocationCapability};
use kernel::hil::time::Alarm;
use kernel::hil::time;

#[derive(Default)]
pub struct MbData {
    pub (crate) callback: Option<Callback>,
}
pub struct mbox {
    kernel: &'static Kernel,
    mailbox_id: Cell<MK_MAILBOX_ID_u>,
    owner_process_index: Cell<MK_Index_t>,
    sender_process_index: Cell<MK_Index_t>,
    signals: Cell<MK_BITMAP_t>,
    pub(crate) data: Grant<MbData>,

}

impl  mbox {
    pub fn new(mb_id : MK_MAILBOX_ID_u, owner_pi: MK_Index_t, sender_pi: MK_Index_t
                ,kernel: &'static Kernel,
               capability: &dyn MemoryAllocationCapability) -> mbox {
        //static mut signals : [MK_BITMAP_t;3] = [0;3];
        //let signals_cell = RingBuffer::new(& mut signals);
        mbox {
            kernel,
            mailbox_id: Cell::new(mb_id),
            owner_process_index: Cell::new(owner_pi),
            sender_process_index: Cell::new(sender_pi),
            signals: Cell::new(0),
            data: kernel.create_grant(capability),

        }
    }

    pub fn create_com_mb( kernel: &'static Kernel,
                          capability: &dyn MemoryAllocationCapability) -> mbox {
        mbox{
            kernel,
            mailbox_id: Cell::new(MK_MAILBOX_COM_MAIN_ID),
            owner_process_index: Cell::new(2),
            sender_process_index: Cell::new(1),
            signals: Cell::new(0),
            data: kernel.create_grant(capability),
        }
    }
    pub fn create_main_mgt_mb( kernel: &'static Kernel,
                               capability: &dyn MemoryAllocationCapability) -> mbox {
        mbox{
            kernel,
            mailbox_id: Cell::new(MK_MAILBOX_MAIN_MGT_ID),
            owner_process_index: Cell::new(1),
            sender_process_index: Cell::new(0),
            signals: Cell::new(0),
            data: kernel.create_grant(capability),
        }
    }
    pub  fn create_mgt_mb( kernel: &'static Kernel,
                           capability: &dyn MemoryAllocationCapability) -> mbox {
        mbox{
            kernel,
            mailbox_id: Cell::new(MK_MAILBOX_MGT_MAIN_ID),
            owner_process_index: Cell::new(0),
            sender_process_index: Cell::new(1),
            signals: Cell::new(0),
            data: kernel.create_grant(capability),
        }
    }
    pub fn create_main_com_mb( kernel: &'static Kernel,
                               capability: &dyn MemoryAllocationCapability) -> mbox {
        mbox{
            kernel,
            mailbox_id: Cell::new(MK_MAILBOX_MAIN_COM_ID),
            owner_process_index: Cell::new(1),
            sender_process_index: Cell::new(2),
            signals: Cell::new(0),
            data: kernel.create_grant(capability),
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

        if new_sigs_bitmap != 0 {
            self.fired();
        }
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

    pub(crate) fn fired(&self){
        if self.get_sender_proc_appid().is_some() {
            let appid = self.get_sender_proc_appid().unwrap();
            debug!("Sender Appid {:?}",appid);
            self.data.enter(appid,
                            |data, _| {
                                data.callback.map(|mut cb |{
                                    debug!("Got Here");
                                    cb.schedule(0,0,0);});
                            });
        }
    }

    pub(crate) fn get_owner_proc_appid(&self) -> Option<AppId>{
        let index = self.get_owner_proc_i();
        let appid = self.kernel.lookup_app_by_identifier(index as usize);
        appid
    }
    pub(crate) fn get_sender_proc_appid(&self)-> Option<AppId>{
        let index = self.get_sender_proc_i();
        let appid=self.kernel.lookup_app_by_identifier(index as usize);
        appid
    }

}

