#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::vpp::mloi::*;
use core::cell::Cell;
use kernel::{AppSlice, Shared, Grant, Kernel, AppId,debug};
use kernel::capabilities::MemoryAllocationCapability;
use core::borrow::{BorrowMut, Borrow};

#[derive(Default)]
pub struct ipcData {
    /// An array of app slices that this application has shared with other
    /// applications.
    pub(crate) shared_memory:Option<AppSlice<Shared, u8>>,
}
pub struct ipc {
    ipc_id: Cell<MK_IPC_ID_u>,
    m_uLength_IPC: Cell<u16>,
    writer_process_index: Cell<MK_Index_t> ,
    reader_process_index: Cell<MK_Index_t>,
    pub(crate) data: Grant<ipcData>,
}

impl  ipc{
    pub fn new(ipc_id : MK_IPC_ID_u,
               length: u16,
               writer_proc: MK_Index_t,
               reader_proc: MK_Index_t,
               kernel: &'static Kernel,
               capability: &dyn MemoryAllocationCapability) -> ipc {
        ipc{
            ipc_id: Cell::new(ipc_id),
            m_uLength_IPC: Cell::new(length),
            writer_process_index: Cell::new(writer_proc),
            reader_process_index: Cell::new(reader_proc),
            data: kernel.create_grant(capability),
        }
    }
    pub fn create_mgt_main_ipc(kernel: &'static Kernel,
                               capability: &dyn MemoryAllocationCapability) -> ipc {
        ipc{
            ipc_id: Cell::new(MK_IPC_MGT_MAIN_ID),
            m_uLength_IPC: Cell::new(MK_IPC_MGT_LENGTH),
            writer_process_index: Cell::new(0),
            reader_process_index: Cell::new(2),
            data: kernel.create_grant(capability),
        }
    }
    pub fn create_com_main_ipc(kernel: &'static Kernel,
                               capability: &dyn MemoryAllocationCapability) -> ipc {
        ipc{
            ipc_id: Cell::new(MK_IPC_COM_MAIN_ID),
            m_uLength_IPC: Cell::new(MK_IPC_COM_LENGTH),
            writer_process_index: Cell::new(1),
            reader_process_index: Cell::new(2),
            data: kernel.create_grant(capability),
        }
    }
    pub fn create_main_mgt_ipc(kernel: &'static Kernel,
                               capability: &dyn MemoryAllocationCapability) -> ipc {
        ipc{
            ipc_id: Cell::new(MK_IPC_MAIN_MGT_ID),
            m_uLength_IPC: Cell::new(MK_IPC_MGT_LENGTH),
            writer_process_index: Cell::new(2),
            reader_process_index: Cell::new(0),
            data: kernel.create_grant(capability),
        }
    }
    pub fn create_main_com_ipc(kernel: &'static Kernel,
                               capability: &dyn MemoryAllocationCapability) -> ipc {
        ipc{
            ipc_id: Cell::new(MK_IPC_MAIN_COM_ID),
            m_uLength_IPC: Cell::new(MK_IPC_COM_LENGTH),
            writer_process_index: Cell::new(2),
            reader_process_index: Cell::new(1),
            data: kernel.create_grant(capability),
        }
    }
    pub fn get_ipc_id (&self) -> MK_IPC_ID_u {
        self.ipc_id.get()
    }

    pub fn get_ipc_len(&self) -> u16 {
        self.m_uLength_IPC.get()
    }
    pub fn get_writer_proc_i(&self) -> MK_Index_t {
        self.writer_process_index.get()
    }
    pub fn get_reader_proc_i(&self) -> MK_Index_t {
        self.reader_process_index.get()
    }


    pub fn expose_slice_to_app(&self, caller_id: AppId, exposer_id: AppId) -> bool{
        debug!("wsilt hne");
        self.data.enter(exposer_id,|data2,_|{
                // expose slices in data2 to data1
                let slice = data2.shared_memory.as_ref().unwrap();
                unsafe {slice.expose_to(caller_id)};
                true
        }).unwrap_or(false)
    }
}
