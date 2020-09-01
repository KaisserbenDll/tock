use capsules::virtual_uart::{MuxUart, UartDevice};
use kernel::capabilities;
use kernel::component::Component;
use kernel::hil;
use kernel::static_init;
use capsules::vpp::ProcessManagerConsole;


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

impl Component  for ProcessConsoleComponent {
    type StaticInput = ();
    type Output = &'static ProcessManagerConsole::VppProcessManager<'static,Capability>;



    unsafe fn finalize(self, _s: Self::StaticInput) -> Self::Output {
        // Create virtual device for console.
        let console_uart = static_init!(UartDevice, UartDevice::new(self.uart_mux, true));
        console_uart.setup();

        let console = static_init!(
            ProcessManagerConsole::VppProcessManager<'static,Capability>,
            ProcessManagerConsole::VppProcessManager::new(
                console_uart,
                &mut ProcessManagerConsole::WRITE_BUF,
                &mut ProcessManagerConsole::READ_BUF,
                &mut ProcessManagerConsole::COMMAND_BUF,
                self.board_kernel,
                Capability,
            )
        );


            hil::uart::Transmit::set_transmit_client(console_uart, console);
            hil::uart::Receive::set_receive_client(console_uart, console);

        console
    }
}
