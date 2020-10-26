#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::vpp::mloi::*;
use core::cell::Cell;

//#[derive(Clone)]
pub struct ipc {
    ipc_id: Cell<MK_IPC_ID_u>,
    m_uLength_IPC: u16,
    writer_process_index: Cell<usize> ,//tbc
    reader_process_index: Cell<usize>,
    data: u8,
}

impl  ipc{
    pub fn new(ipc_id : MK_IPC_ID_u, length: u16, writer_proc: usize, reader_proc: usize)
        -> ipc {
        ipc{
            ipc_id: Cell::new(ipc_id),
            m_uLength_IPC: length,
            writer_process_index: Cell::new(writer_proc),
            reader_process_index: Cell::new(reader_proc),
            data: 0
        }
    }

}
