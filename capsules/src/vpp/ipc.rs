#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::vpp::mloi::*;
use core::cell::Cell;

pub struct ipc {
    ipc_id: Cell<MK_IPC_ID_u>,
    m_uLength_IPC: u16,
    writer_process_index: Cell<MK_Index_t> ,
    reader_process_index: Cell<MK_Index_t>,
    // tock_ipc: kernel::ipc::IPC,
    data: u8,
}

impl  ipc{
    pub fn new(ipc_id : MK_IPC_ID_u, length: u16, writer_proc: MK_Index_t, reader_proc: MK_Index_t)
        -> ipc {
        ipc{
            ipc_id: Cell::new(ipc_id),
            m_uLength_IPC: length,
            writer_process_index: Cell::new(writer_proc),
            reader_process_index: Cell::new(reader_proc),
            data: 0
        }
    }
    pub fn create_mgt_main_ipc() -> ipc {
        ipc{
            ipc_id: Cell::new(MK_IPC_MGT_MAIN_ID),
            m_uLength_IPC: 64,
            writer_process_index: Cell::new(0),
            reader_process_index: Cell::new(2),
            data: 0
        }
    }
    pub fn create_com_main_ipc() -> ipc {
        ipc{
            ipc_id: Cell::new(MK_IPC_COM_MAIN_ID),
            m_uLength_IPC: 64,
            writer_process_index: Cell::new(1),
            reader_process_index: Cell::new(2),
            data: 0
        }
    }
    pub fn create_main_mgt_ipc() -> ipc {
        ipc{
            ipc_id: Cell::new(MK_IPC_MAIN_MGT_ID),
            m_uLength_IPC: 64,
            writer_process_index: Cell::new(2),
            reader_process_index: Cell::new(0),
            data: 0
        }
    }
    pub fn create_main_com_ipc() -> ipc {
        ipc{
            ipc_id: Cell::new(MK_IPC_MAIN_COM_ID),
            m_uLength_IPC: 64,
            writer_process_index: Cell::new(2),
            reader_process_index: Cell::new(1),
            data: 0
        }
    }
    pub fn get_ipc_id (&self) -> MK_IPC_ID_u {
        self.ipc_id.get()
    }
}
