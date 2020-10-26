//! Implementation of VPP Process Management dedicated Functions
//! This module VppProcessManager can be used as a component to control and "inspect"
//! userspace processes.

#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::cell::Cell;
use core::cmp;
use core::str;
use crate::vpp::mloi::MK_ERROR_e::*;
use crate::vpp::mloi::MK_PROCESS_PRIORITY_e::*;
use crate::vpp::mloi::*;
use crate::vpp::process::*;
use crate::vpp::mloi::VppState::*;
use crate::vpp::process;
use kernel::{Kernel, capabilities};
use kernel::introspection::KernelInfo;
use kernel::common::cells::TakeCell;
use kernel::procs::ProcessType;
use kernel::capabilities::ProcessManagementCapability;
use kernel::debug;
use kernel::hil::uart;
use kernel::ReturnCode;
use crate::vpp::process::VppProcess;
use kernel::{ListenerType,LISTENER};
use kernel::{Callback, Driver,Grant,AppId};
use crate::virtual_uart::{MuxUart, UartDevice};
use kernel::static_init;
use kernel::hil;
use kernel::procs::{FunctionCall,FunctionCallSource,Task};
use crate::vpp::mloi::VppState::RUNNING;
use core::convert::TryInto;
use crate::vpp::mailbox::mbox;
use core::borrow::{BorrowMut, Borrow};
use crate::vpp::vppkernel::VppKernel;

pub const DRIVER_NUM: usize = 0x90004 ;

// Since writes are character echoes, we do not need more than 4 bytes:
// the longest write is 3 bytes for a backspace (backspace, space, backspace).
pub static mut WRITE_BUF: [u8; 4] = [0; 4];
// Since reads are byte-by-byte, to properly echo what's typed,
// we can use a very small read buffer.
pub static mut READ_BUF: [u8; 4] = [0; 4];
// Commands can be up to 32 bytes long: since commands themselves are 4-5
// characters, limiting arguments to 25 bytes or so seems fine for now.
pub static mut COMMAND_BUF: [u8; 32] = [0; 32];

pub struct ProcessConsole <'a,C: ProcessManagementCapability>{
    uart: &'a dyn uart::UartData<'a>,
    tx_in_progress: Cell<bool>,
    tx_buffer: TakeCell<'static, [u8]>,
    rx_in_progress: Cell<bool>,
    rx_buffer: TakeCell<'static, [u8]>,
    command_buffer: TakeCell<'static, [u8]>,
    command_index: Cell<usize>,
    /// Flag to mark that the process console is active and has called receive
    /// from the underlying UART.
    running: Cell<bool>,
    /// Internal flag that the process console should parse the command it just
    /// received after finishing echoing the last newline character.
    execute: Cell<bool>,
    vppkernel: &'static VppKernel,
    capability: C,
}

impl <'a,C: ProcessManagementCapability> ProcessConsole<'a, C> {
    pub fn new(
        uart: &'a dyn uart::UartData<'a>,
        tx_buffer: &'static mut [u8],
        rx_buffer: &'static mut [u8],
        cmd_buffer: &'static mut [u8],
        vppkernel: &'static VppKernel,
        capability : C
    ) -> ProcessConsole<'a,C> {
        ProcessConsole {
            uart,
            tx_in_progress: Cell::new(false),
            tx_buffer: TakeCell::new(tx_buffer),
            rx_in_progress: Cell::new(false),
            rx_buffer: TakeCell::new(rx_buffer),
            command_buffer: TakeCell::new(cmd_buffer),
            command_index: Cell::new(0),
            running: Cell::new(false),
            execute: Cell::new(false),
            vppkernel,
            capability,
        }
    }
    pub fn start(&self) -> ReturnCode {
        if self.running.get() == false {
            self.rx_buffer.take().map(|buffer| {
                self.rx_in_progress.set(true);
                self.uart.receive_buffer(buffer, 1);
                self.running.set(true);
                debug!("Starting process console");
            });
        }
        ReturnCode::SUCCESS
    }
    // Process the command in the command buffer and clear the buffer.
    fn read_command(&self) {
        self.command_buffer.map(|command| {
            let mut terminator = 0;
            let len = command.len();
            for i in 0..len {
                if command[i] == 0 {
                    terminator = i;
                    break;
                }
            }
            //debug!("Command: {}-{} {:?}", start, terminator, command);
            // A command is valid only if it starts inside the buffer,
            // ends before the beginning of the buffer, and ends after
            // it starts.
            if terminator > 0 {
                let cmd_str = str::from_utf8(&command[0..terminator]);
                match cmd_str {
                    Ok(s) => {
                        let clean_str = s.trim();
                        if clean_str.starts_with("help") {
                            debug!("Welcome to the process console.");
                            debug!("Valid commands are: help status list resume  suspend yield");
                        }
                        else if clean_str.starts_with("list") {
                            debug!(" PID    Name                Quanta  Syscalls  Dropped Callbacks  Restarts    State  Grants   VPPState  Error");
                            self.vppkernel.kernel
                                .process_each_capability(&self.capability, |proc| {
                                    let info: KernelInfo = KernelInfo::new(self.vppkernel.kernel);

                                    let pname = proc.get_process_name();
                                    let appid = proc.appid();
                                    let (grants_used, grants_total) = info.number_app_grant_uses(appid, &self.capability);

                                    debug!(
                                        "  {:?}\t{:<20}{:6}{:10}{:19}{:10}  {:?}{:5}/{}",
                                        appid,
                                        pname,
                                        proc.debug_timeslice_expiration_count(),
                                        proc.debug_syscall_count(),
                                        proc.debug_dropped_callback_count(),
                                        proc.get_restart_count(),
                                        proc.get_state(),
                                        grants_used,
                                        grants_total,
                                        );
                                });
                        }
                        else if clean_str.starts_with("status") {
                            let info: KernelInfo = KernelInfo::new(self.vppkernel.kernel);
                            debug!(
                                "Total processes: {}",
                                info.number_loaded_processes(&self.capability)
                            );
                            debug!(
                                "Active processes: {}",
                                info.number_active_processes(&self.capability)
                            );
                            debug!(
                                "Timeslice expirations: {}",
                                info.timeslice_expirations(&self.capability)
                            );
                        }
                        else if clean_str.starts_with("resume") {
                             let argument = clean_str.split_whitespace().nth(1);
                             if argument.is_some(){
                                 if argument.unwrap() == self.vppkernel.vpp_processes[0].as_ref().unwrap().get_process_name() {
                                     let handle=self.vppkernel._mk_get_process_handle(0);
                                     self.vppkernel._mk_resume_process(handle);
                                     self.vppkernel._mk_resume_process(1);
                                 }else {
                                     debug!("Name of Process non existant");
                                 }
                             } else  {
                                 debug!("No Process");
                             }
                         }
                        else if clean_str.starts_with("suspend") {
                            let argument = clean_str.split_whitespace().nth(1);
                            if argument.is_some(){
                                if argument.unwrap() == self.vppkernel.vpp_processes[0].as_ref().unwrap().get_process_name() {
                                    let handle=self.vppkernel._mk_get_process_handle(0);
                                    self.vppkernel._mk_suspend_process(handle);
                                    self.vppkernel._mk_suspend_process(1);
                                }else {
                                    debug!("Name of Process non existant");
                                }
                            } else  {
                                debug!("No Process");
                            }
                        }
                        else {
                            debug!("Valid commands are: help status list resume suspend yield");
                        }
                    }
                    Err(_e) => debug!("Invalid command: {:?}", command),
                }
            }
        });
        self.command_buffer.map(|command| {
            command[0] = 0;
        });
        self.command_index.set(0);
    }

    fn write_byte(&self, byte: u8) -> ReturnCode {
        if self.tx_in_progress.get() {
            ReturnCode::EBUSY
        } else {
            self.tx_in_progress.set(true);
            self.tx_buffer.take().map(|buffer| {
                buffer[0] = byte;
                self.uart.transmit_buffer(buffer, 1);
            });
            ReturnCode::SUCCESS
        }
    }

    fn write_bytes(&self, bytes: &[u8]) -> ReturnCode {
        if self.tx_in_progress.get() {
            ReturnCode::EBUSY
        } else {
            self.tx_in_progress.set(true);
            self.tx_buffer.take().map(|buffer| {
                let len = cmp::min(bytes.len(), buffer.len());
                // Copy elements of `bytes` into `buffer`
                (&mut buffer[..len]).copy_from_slice(&bytes[..len]);
                self.uart.transmit_buffer(buffer, len);
            });
            ReturnCode::SUCCESS
        }
    }
}


impl<'a,C: ProcessManagementCapability> uart::TransmitClient for ProcessConsole<'a,C> {
    fn transmitted_buffer(&self, buffer: &'static mut [u8], _tx_len: usize, _rcode: ReturnCode) {
        self.tx_buffer.replace(buffer);
        self.tx_in_progress.set(false);

        // Check if we just received and echoed a newline character, and
        // therefore need to process the received message.
        if self.execute.get() {
            self.execute.set(false);
            self.read_command();
        }
    }
}
impl<'a,C: ProcessManagementCapability> uart::ReceiveClient for ProcessConsole<'a,C> {
    fn received_buffer(
        &self,
        read_buf: &'static mut [u8],
        rx_len: usize,
        _rcode: ReturnCode,
        error: uart::Error,
    ) {
        if error == uart::Error::None {
            match rx_len {
                0 => debug!("ProcessConsole had read of 0 bytes"),
                1 => {
                    self.command_buffer.map(|command| {
                        let index = self.command_index.get() as usize;
                        if read_buf[0] == ('\n' as u8) || read_buf[0] == ('\r' as u8) {
                            self.execute.set(true);
                            self.write_bytes(&['\r' as u8, '\n' as u8]);
                        } else if read_buf[0] == ('\x08' as u8) && index > 0 {
                            // Backspace, echo and remove last byte
                            // Note echo is '\b \b' to erase
                            self.write_bytes(&['\x08' as u8, ' ' as u8, '\x08' as u8]);
                            command[index - 1] = '\0' as u8;
                            self.command_index.set(index - 1);
                        } else if index < (command.len() - 1) && read_buf[0] < 128 {
                            // For some reason, sometimes reads return > 127 but no error,
                            // which causes utf-8 decoding failure, so check byte is < 128. -pal

                            // Echo the byte and store it
                            self.write_byte(read_buf[0]);
                            command[index] = read_buf[0];
                            self.command_index.set(index + 1);
                            command[index + 1] = 0;
                        }
                    });
                }
                _ => debug!(
                    "ProcessConsole issues reads of 1 byte, but receive_complete was length {}",
                    rx_len
                ),
            };
        }
        self.rx_in_progress.set(true);
        self.uart.receive_buffer(read_buf, 1);
    }
}

// Instantiating  Component
pub struct Capability;
unsafe impl capabilities::ProcessManagementCapability for Capability {}

pub struct ProcessConsoleComponent {
    vppkernel: &'static VppKernel,
    uart_mux: &'static MuxUart<'static>,
}

impl ProcessConsoleComponent {
    pub fn new(
        vppkernel: &'static VppKernel,
        uart_mux: &'static MuxUart,
    ) -> ProcessConsoleComponent {
        ProcessConsoleComponent {
            vppkernel: vppkernel,
            uart_mux: uart_mux,
        }
    }
    pub unsafe fn finalize(self, _s: ()) -> &'static ProcessConsole<'static,Capability>
    {
        // Create virtual device for console.
        let console_uart = static_init!(UartDevice, UartDevice::new(self.uart_mux, true));
        console_uart.setup();

        let console = static_init!(
            ProcessConsole<'static,Capability>,
            ProcessConsole::new(
                console_uart,
                &mut WRITE_BUF,
                &mut READ_BUF,
                &mut COMMAND_BUF,
                self.vppkernel,
                Capability
            )
        );

        hil::uart::Transmit::set_transmit_client(console_uart, console);
        hil::uart::Receive::set_receive_client(console_uart, console);

        console
    }
}

// Implementing a Syscall Driver for Vpp Process Manager (VPM)
/*
pub struct VPMDriver {
    vpm: &'static VppProcessManager<'static,Capability>,
    // apps: Grant<Option<Callback>>,
}
impl VPMDriver{
    pub fn new( vpm:  &'static VppProcessManager<'static,Capability>) -> VPMDriver{
                // grant: Grant<Option<Callback>>,) -> VPMDriver{
        VPMDriver{
            vpm,
            // apps: grant
        }
    }
}
impl Driver  for VPMDriver {
    fn command(&self,
               command_num: usize,
               data: usize,
               _data2: usize,
               appid: AppId) -> ReturnCode {
        match command_num {
            0 => ReturnCode::SUCCESS,
            1 =>
                {
                    debug!("Suspending Process");
                    //debug!("Data is {:?}", data);
                    let handle = convert_to_handle(data as u16);
                    //let handle = self.vpm._mk_get_process_handle(0);
                    let error_result = self.vpm._mk_suspend_process(handle);
                    let ret = MK_ERROR_e::into(error_result);
                    self.vpm._mk_Get_Error();
                    debug!("Last Error {:?}", self.vpm.last_error.get());
                    ReturnCode::SuccessWithValue {
                        value: ret
                    }
                },
            2 =>
                {
                    debug!("Resuming Process");
                    let handle = convert_to_handle(data as u16);
                    let error_result = self.vpm._mk_resume_process(handle);
                    let ret = MK_ERROR_e::into(error_result);
                    self.vpm._mk_Get_Error();
                    ReturnCode::SuccessWithValue {
                        value: ret
                    }
                },
            3 =>
                {
                    debug!("Yielding  Process");
                    let handle = convert_to_handle(appid.id() as u16  );
                    debug!("ID of process {:?}",appid);
                    self.vpm._mk_yield(handle);
                    self.vpm._mk_Get_Error();
                    ReturnCode::SUCCESS
                },
            4 =>
                {
                    debug!("Testing States");
                    let process = self.vpm.get_process_ref_internal(data as u32);
                    //process.unwrap().sync_vpp_tock_states();
                    let tock_state = process.unwrap().tockprocess.unwrap().get_state();
                    let vpp_state = process.unwrap().vppstate.get();
                    debug!("Tock State {:?} , Vpp Process {:?}", tock_state, vpp_state);
                    ReturnCode::SUCCESS
                },
            // 4 =>
            //     {
            //         debug!("Getting  Process Handle");
            //         let handle = self.vpm._mk_get_process_handle(data as u16);
            //         let data = handle ;
            //         self.vpm._mk_Get_Error();
            //         ReturnCode::SUCCESS
            //     },
            _ => ReturnCode::ENOSUPPORT,
        }
    }
}
*/

