use capsules::virtual_uart::{MuxUart, UartDevice};
use kernel::capabilities;
use kernel::component::Component;
use kernel::hil;
use kernel::static_init;
use capsules::vpp::ProcessManagerConsoleCap;

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
    type Output = &'static ProcessManagerConsoleCap::VppProcessManager<'static,Capability>;


    unsafe fn finalize(self, _s: Self::StaticInput) -> Self::Output
    {
        // Create virtual device for console.
        let console_uart = static_init!(UartDevice, UartDevice::new(self.uart_mux, true));
        console_uart.setup();

        let console = static_init!(
            ProcessManagerConsoleCap::VppProcessManager<'static,Capability>,
            ProcessManagerConsoleCap::VppProcessManager::new(
                console_uart,
                &mut ProcessManagerConsoleCap::WRITE_BUF,
                &mut ProcessManagerConsoleCap::READ_BUF,
                &mut ProcessManagerConsoleCap::COMMAND_BUF,
                self.board_kernel,
                Capability
            )
        );

            hil::uart::Transmit::set_transmit_client(console_uart, console);
            hil::uart::Receive::set_receive_client(console_uart, console);

        console
    }
}
