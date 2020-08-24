//! Component for ProcessConsole, the command console.
//!
//! This provides one Component, ProcessConsoleComponent, which implements a
//! command console for controlling processes over a UART bus. On imix this is
//! typically USART3 (the DEBUG USB connector).
//!
//! Usage
//! -----
//! ```rust
//! let pconsole = ProcessConsoleComponent::new(board_kernel, uart_mux).finalize(());
//! ```


// use capsules::{process_console_vpp, process_vpp};
use capsules::process_vpp;
use capsules::virtual_uart::{MuxUart, UartDevice};
use kernel::capabilities;
use kernel::component::Component;
use kernel::hil;
use kernel::static_init;
use capsules::process_vpp::VppProcessType;


// Number of allowed process
const NUM_PROCS: usize =4;
//
// Actual memory for holding the active process structures. Need an empty list
// at least.
// static mut VPP_PROCESSES: [Option<&'static dyn process_vpp::VppProcessType>; NUM_PROCS] =
//     [None;NUM_PROCS];

pub struct ProcessConsoleComponent {
    board_kernel: &'static kernel::Kernel,
    uart_mux: &'static MuxUart<'static>,
}

impl ProcessConsoleComponent {
    pub fn new(
        board_kernel: &'static kernel::Kernel,
        uart_mux: &'static MuxUart,
    ) -> ProcessConsoleComponent {
        ProcessConsoleComponent {
            board_kernel: board_kernel,
            uart_mux: uart_mux,
        }
    }
}

pub struct Capability;
unsafe impl capabilities::ProcessManagementCapability for Capability {}

impl Component for ProcessConsoleComponent {
    type StaticInput = ();
    type Output = &'static process_console_vpp::ProcessConsole<'static, Capability>;



    unsafe fn finalize(self, _s: Self::StaticInput) -> Self::Output {
        // Create virtual device for console.
        let console_uart = static_init!(UartDevice, UartDevice::new(self.uart_mux, true));
        console_uart.setup();

        let console = static_init!(
            process_console_vpp::VppProcessConsole<'static, Capability>,
            process_console_vpp::VppProcessConsole::new(
                console_uart,
                &mut process_console_vpp::WRITE_BUF,
                &mut process_console_vpp::READ_BUF,
                &mut process_console_vpp::COMMAND_BUF,
                self.board_kernel,
                Capability,
            )
        );
        let vpp_console = process_console_vpp::VppProcessConsole::new_vpp_console(pconsole,console);

        hil::uart::Transmit::set_transmit_client(console_uart, vpp_console);
        hil::uart::Receive::set_receive_client(console_uart, vpp_console);

        vpp_console
    }
}
