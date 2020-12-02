//! Board file for LowRISC OpenTitan RISC-V development platform.
//!
//! - <https://opentitan.org/>

#![no_std]
// Disable this attribute when documenting, as a workaround for
// https://github.com/rust-lang/rust/issues/62184.
#![cfg_attr(not(doc), no_main)]
#![feature(const_in_array_repeat_expressions)]

use capsules::virtual_alarm::{MuxAlarm, VirtualMuxAlarm};
use capsules::virtual_hmac::VirtualMuxHmac;
use earlgrey::chip::EarlGreyDefaultPeripherals;
use kernel::capabilities;
use kernel::common::dynamic_deferred_call::{DynamicDeferredCall, DynamicDeferredCallClientState};
use kernel::component::Component;
use kernel::hil;
use kernel::hil::i2c::I2CMaster;
use kernel::hil::time::Alarm;
use kernel::Chip;
use kernel::Platform;
use kernel::{create_capability, debug, static_init};
use rv32i::csr;
use capsules::vpp::process::VppProcess;
use capsules::vpp::vppkernel::VppKernel;

#[allow(dead_code)]
mod aes_test;

#[allow(dead_code)]
mod multi_alarm_test;

pub mod io;
pub mod usb;

use capsules::vpp::vppkernel::NUM_PROCS;
use capsules::vpp::mloi::{MK_MAILBOX_LIMIT, MK_IPC_LIMIT};
use capsules::vpp::mailbox::mbox;
use capsules::vpp::ipc::ipc;

// Actual memory for holding the active process structures. Need an empty list
// at least.
static mut PROCESSES: [Option<&'static dyn kernel::procs::ProcessType>; NUM_PROCS] =
    [None;NUM_PROCS];
static mut VPP_PROCESSES: [Option<VppProcess>;NUM_PROCS] = [None;NUM_PROCS];
static mut MBOX_ARRAY: [Option<mbox>;MK_MAILBOX_LIMIT] = [None;MK_MAILBOX_LIMIT];
static mut IPC_ARRAY : [Option<ipc>; MK_IPC_LIMIT] = [None;MK_IPC_LIMIT];
static mut CHIP: Option<
    &'static earlgrey::chip::EarlGrey<VirtualMuxAlarm<'static, earlgrey::timer::RvTimer>>,
> = None;

// How should the kernel respond when a process faults.
const FAULT_RESPONSE: kernel::procs::FaultResponse = kernel::procs::FaultResponse::Panic;

/// Dummy buffer that causes the linker to reserve enough space for the stack.
#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x1000] = [0; 0x1000];

/// A structure representing this platform that holds references to all
/// capsules for this platform. We've included an alarm and console.
struct OpenTitan {
    led: &'static capsules::led::LED<'static, earlgrey::gpio::GpioPin<'static>>,
    gpio: &'static capsules::gpio::GPIO<'static, earlgrey::gpio::GpioPin<'static>>,
    console: &'static capsules::console::Console<'static>,
    ipc: kernel::ipc::IPC,
    alarm: &'static capsules::alarm::AlarmDriver<
        'static,
        VirtualMuxAlarm<'static, earlgrey::timer::RvTimer<'static>>,
    >,
    hmac: &'static capsules::hmac::HmacDriver<
        'static,
        VirtualMuxHmac<'static, lowrisc::hmac::Hmac<'static>, [u8; 32]>,
        [u8; 32],
    >,
    lldb: &'static capsules::low_level_debug::LowLevelDebug<
        'static,
        capsules::virtual_uart::UartDevice<'static>,
    >,
    i2c_master: &'static capsules::i2c_master::I2CMasterDriver<lowrisc::i2c::I2c<'static>>,
    pm : &'static capsules::vpp::pmsyscall::ProcessManager,
    vpp_driver: &'static capsules::vpp::vppkernel::vpp_kernel_driver,
    nonvolatile_storage: &'static capsules::nonvolatile_storage_driver::NonvolatileStorage<'static>,

    // vpmdriver: &'static capsules::vpp::ProcessManagerConsoleCap::VPMDriver,
    // testdriver: &'static capsules::vpp::SubTest::Test,
}

/// Mapping of integer syscalls to objects that implement syscalls.
impl Platform for OpenTitan {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&dyn kernel::Driver>) -> R,
    {
        match driver_num {
            capsules::vpp::vppkernel::DRIVER_NUM => f(Some(self.vpp_driver)),
            // 0x9000A => f(Some(self.testdriver)),
            0x90003 => f(Some(self.pm)),
            // 0x90004 => f(Some(self.vpmdriver)),
            capsules::led::DRIVER_NUM => f(Some(self.led)),
            capsules::hmac::DRIVER_NUM => f(Some(self.hmac)),
            capsules::gpio::DRIVER_NUM => f(Some(self.gpio)),
            capsules::console::DRIVER_NUM => f(Some(self.console)),
            kernel::ipc::DRIVER_NUM => f(Some(&self.ipc)),
            capsules::alarm::DRIVER_NUM => f(Some(self.alarm)),
            capsules::low_level_debug::DRIVER_NUM => f(Some(self.lldb)),
            capsules::i2c_master::DRIVER_NUM => f(Some(self.i2c_master)),
            capsules::nonvolatile_storage_driver::DRIVER_NUM => f(Some(self.nonvolatile_storage)),
            _ => f(None),
        }
    }
}

/// Reset Handler.
///
/// This function is called from the arch crate after some very basic RISC-V
/// setup.
#[no_mangle]
pub unsafe fn reset_handler() {
    // Basic setup of the platform.
    rv32i::init_memory();
    // Ibex-specific handler
    earlgrey::chip::configure_trap_handler();

    let peripherals = static_init!(
        EarlGreyDefaultPeripherals,
        EarlGreyDefaultPeripherals::new()
    );
    // initialize capabilities
    let process_mgmt_cap = create_capability!(capabilities::ProcessManagementCapability);
    let memory_allocation_cap = create_capability!(capabilities::MemoryAllocationCapability);
    let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);
    let main_loop_cap = create_capability!(capabilities::MainLoopCapability);

    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(&PROCESSES));


    let dynamic_deferred_call_clients =
        static_init!([DynamicDeferredCallClientState; 1], Default::default());
    let dynamic_deferred_caller = static_init!(
        DynamicDeferredCall,
        DynamicDeferredCall::new(dynamic_deferred_call_clients)
    );
    DynamicDeferredCall::set_global_instance(dynamic_deferred_caller);

    // Configure kernel debug gpios as early as possible
    kernel::debug::assign_gpios(
        Some(&earlgrey::gpio::PORT[7]), // First LED
        None,
        None,
    );

    // Create a shared UART channel for the console and for kernel debug.
    let uart_mux = components::console::UartMuxComponent::new(
        &earlgrey::uart::UART0,
        earlgrey::uart::UART0_BAUDRATE,
        dynamic_deferred_caller,
    )
    .finalize(());
    uart_mux.initialize();
    // Setup the console.
    let console = components::console::ConsoleComponent::new(board_kernel, uart_mux).finalize(());
    // let process_console =
    //    components::process_console::ProcessConsoleComponent::new(board_kernel, uart_mux)
    //    .finalize(());

    // Create the debugger object that handles calls to `debug!()`.
    components::debug_writer::DebugWriterComponent::new(uart_mux).finalize(());

    let lldb = components::lldb::LowLevelDebugComponent::new(board_kernel, uart_mux).finalize(());

    // LEDs
    // Start with half on and half off
    let led = components::led::LedsComponent::new(components::led_component_helper!(
        earlgrey::gpio::GpioPin,
        (
            &earlgrey::gpio::PORT[8],
            kernel::hil::gpio::ActivationMode::ActiveHigh
        ),
        (
            &earlgrey::gpio::PORT[9],
            kernel::hil::gpio::ActivationMode::ActiveHigh
        ),
        (
            &earlgrey::gpio::PORT[10],
            kernel::hil::gpio::ActivationMode::ActiveHigh
        ),
        (
            &earlgrey::gpio::PORT[11],
            kernel::hil::gpio::ActivationMode::ActiveHigh
        ),
        (
            &earlgrey::gpio::PORT[12],
            kernel::hil::gpio::ActivationMode::ActiveHigh
        ),
        (
            &earlgrey::gpio::PORT[13],
            kernel::hil::gpio::ActivationMode::ActiveHigh
        ),
        (
            &earlgrey::gpio::PORT[14],
            kernel::hil::gpio::ActivationMode::ActiveHigh
        ),
        (
            &earlgrey::gpio::PORT[15],
            kernel::hil::gpio::ActivationMode::ActiveHigh
        )
    ))
    .finalize(components::led_component_buf!(earlgrey::gpio::GpioPin));

    let gpio = components::gpio::GpioComponent::new(
        board_kernel,
        components::gpio_component_helper!(
            earlgrey::gpio::GpioPin,
            0 => &earlgrey::gpio::PORT[0],
            1 => &earlgrey::gpio::PORT[1],
            2 => &earlgrey::gpio::PORT[2],
            3 => &earlgrey::gpio::PORT[3],
            4 => &earlgrey::gpio::PORT[4],
            5 => &earlgrey::gpio::PORT[5],
            6 => &earlgrey::gpio::PORT[6],
            7 => &earlgrey::gpio::PORT[15]
        ),
    )
    .finalize(components::gpio_component_buf!(earlgrey::gpio::GpioPin));

    let alarm = &earlgrey::timer::TIMER;

    // Create a shared virtualization mux layer on top of a single hardware
    // alarm.
    let mux_alarm = static_init!(
        MuxAlarm<'static, earlgrey::timer::RvTimer>,
        MuxAlarm::new(alarm)
    );
    hil::time::Alarm::set_alarm_client(&earlgrey::timer::TIMER, mux_alarm);

    // Alarm
    let virtual_alarm_user = static_init!(
        VirtualMuxAlarm<'static, earlgrey::timer::RvTimer>,
        VirtualMuxAlarm::new(mux_alarm)
    );
    let scheduler_timer_virtual_alarm = static_init!(
        VirtualMuxAlarm<'static, earlgrey::timer::RvTimer>,
        VirtualMuxAlarm::new(mux_alarm)
    );
    let alarm = static_init!(
        capsules::alarm::AlarmDriver<'static, VirtualMuxAlarm<'static, earlgrey::timer::RvTimer>>,
        capsules::alarm::AlarmDriver::new(
            virtual_alarm_user,
            board_kernel.create_grant(&memory_allocation_cap)
        )
    );
    hil::time::Alarm::set_alarm_client(virtual_alarm_user, alarm);
    // let vpp_kernel = static_init!(VppKernel,
    // VppKernel::new(&VPP_PROCESSES,&MBOX_ARRAY,&IPC_ARRAY,board_kernel,virtual_alarm_user));
    let vpp_kernel = static_init!(VppKernel,
    VppKernel::new(&VPP_PROCESSES,&MBOX_ARRAY,&IPC_ARRAY,board_kernel));

    let chip = static_init!(
        earlgrey::chip::EarlGrey<VirtualMuxAlarm<'static, earlgrey::timer::RvTimer>>,
        earlgrey::chip::EarlGrey::new(scheduler_timer_virtual_alarm)
    );
    scheduler_timer_virtual_alarm.set_alarm_client(chip.scheduler_timer());
    CHIP = Some(chip);

    // Need to enable all interrupts for Tock Kernel
    chip.enable_plic_interrupts();
    // enable interrupts globally
    csr::CSR
        .mie
        .modify(csr::mie::mie::msoft::SET + csr::mie::mie::mtimer::SET + csr::mie::mie::mext::SET);
    csr::CSR.mstatus.modify(csr::mstatus::mstatus::mie::SET);


    let hmac_data_buffer = static_init!([u8; 64], [0; 64]);
    let hmac_dest_buffer = static_init!([u8; 32], [0; 32]);

    let mux_hmac = components::hmac::HmacMuxComponent::new(&earlgrey::hmac::HMAC).finalize(
        components::hmac_mux_component_helper!(lowrisc::hmac::Hmac, [u8; 32]),
    );

    let hmac = components::hmac::HmacComponent::new(
        board_kernel,
        &mux_hmac,
        hmac_data_buffer,
        hmac_dest_buffer,
    )
    .finalize(components::hmac_component_helper!(
        lowrisc::hmac::Hmac,
        [u8; 32]
    ));

    let i2c_master = static_init!(
        capsules::i2c_master::I2CMasterDriver<lowrisc::i2c::I2c<'static>>,
        capsules::i2c_master::I2CMasterDriver::new(
            &earlgrey::i2c::I2C,
            &mut capsules::i2c_master::BUF,
            board_kernel.create_grant(&memory_allocation_cap)
        )
    );

    earlgrey::i2c::I2C.set_master_client(i2c_master);
    // Syscall driver to vpp kernel
    let vpp_driver = static_init!(capsules::vpp::vppkernel::vpp_kernel_driver,
    capsules::vpp::vppkernel::vpp_kernel_driver::new(vpp_kernel));
    // useless Syscall Driver
    let pm = static_init!(capsules::vpp::pmsyscall::ProcessManager,
    capsules::vpp::pmsyscall::ProcessManager::new());
    // Capsules to test Subscribe Syscall using grant

    // let testdriver = static_init!(capsules::vpp::SubTest::Test,
    // capsules::vpp::SubTest::Test::new(board_kernel.create_grant(&memory_allocation_cap)));
    // testdriver.trigger_callback();
    extern "C" {
        /// Beginning on the ROM region containing app images.
        static _sstorage: u8;
        static _estorage: u8;
    }

    // Flash
    let nonvolatile_storage = components::nonvolatile_storage::NonvolatileStorageComponent::new(
        board_kernel,
        &peripherals.flash_ctrl,
        0x20000000,                       // Start address for userspace accessible region
        0x8000,                           // Length of userspace accessible region
        &_sstorage as *const u8 as usize, // Start address of kernel region
        &_estorage as *const u8 as usize - &_sstorage as *const u8 as usize, // Length of kernel region
    )
        .finalize(components::nv_storage_component_helper!(
        lowrisc::flash_ctrl::FlashCtrl
    ));

    let vpp_process_console = capsules::vpp::ProcessManagerConsoleCap
    ::ProcessConsoleComponent::new(vpp_kernel,uart_mux)
        .finalize(());
    vpp_process_console.start();
    debug!("OpenTitan initialisation complete. Entering main loop");
    let opentitan = OpenTitan {
        gpio: gpio,
        led: led,
        console: console,
        ipc: kernel::ipc::IPC::new(board_kernel,&grant_cap),
        alarm: alarm,
        hmac,
        lldb: lldb,
        //usb,
        i2c_master,
        pm,
        vpp_driver,
        nonvolatile_storage
    };
    // USB support is currently broken in the OpenTitan hardware
    // See https://github.com/lowRISC/opentitan/issues/2598 for more details
    // let usb = usb::UsbComponent::new(board_kernel).finalize(());

    capsules::vpp::process::load_vpp_processes(
        board_kernel,
        chip,
        // app_flash,
        // app_memory,
        &mut PROCESSES,
        &mut VPP_PROCESSES,
        &mut MBOX_ARRAY,
        &mut IPC_ARRAY,
        FAULT_RESPONSE,
        &process_mgmt_cap)
         .unwrap_or_else(|err| {
             debug!("Error loading processes!");
             debug!("{:?}", err);
         });

    //_vpp_kernel._mk_resume_process(0);
        //_vpp_kernel._mk_resume_process(1);
        //panic!("Panic");
    let scheduler = components::sched::priority::PriorityComponent::new(board_kernel).finalize(());
    board_kernel.kernel_loop(&opentitan, chip, Some(&opentitan.ipc), scheduler, &main_loop_cap);
}
