OpenTitan RISC-V Board
=================

- https://opentitan.org/

OpenTitan is the first open source project building a transparent, high-quality reference design and integration guidelines for silicon root of trust (RoT) chips.

Tock currently supports the OpenTitan snapshot-20191101-2 release on a Nexys Video FPGA, as described here: https://docs.opentitan.org/doc/ug/getting_started_fpga/index.html.

You can get started with OpenTitan using either the Nexys Video FPGA board or simulation. See the OpenTitan [getting started](https://docs.opentitan.org/doc/ug/getting_started/index.html) for more details.

There are two alternatives to run tock on OpenTitan. You program the board or run qemu. The two options are explained in the following sections.

Programming the board 
---------------------

Tock on OpenTitan requires lowRISC/opentitan@4429c362900713c059fbd870db140e0058e1c0eb or newer. In general it is recommended that users start with the latest OpenTitan bitstream and if that results in issues try the one mentioned above.

For more information you can follow the [OpenTitan development flow](https://docs.opentitan.org/doc/ug/getting_started_fpga/index.html#testing-the-demo-design) to flash the image.

First setup the development board using the steps here: https://docs.opentitan.org/doc/ug/getting_started_fpga/index.html. You need to make sure the boot ROM is working and that your machine can communicate with the OpenTitan ROM. You will need to use the `PROG` USB port on the board for this.

To use `make flash` you first need to clone the OpenTitan repo and build the `spiflash` tool.

Export the `OPENTITAN_TREE` enviroment variable to point to the OpenTitan tree.  
<!--- This export is not needed. Because if the instructions of the OT setup have been followed, $REPO_TOP can 
be used instead of exporting a new variable. This is lazy developement.-->


```shell
export OPENTITAN_TREE=/home/opentitan/
```

Back in the Tock directory run `make flash`

If everything works you should see something like this on the console. If you need help getting console access check the [testing the design](https://docs.opentitan.org/doc/ug/getting_started_fpga/index.html#testing-the-demo-design) section in the OpenTitan documentation.

```
bootstrap: DONE!
Jump!
OpenTitan initialisation complete. Entering main loop
```
Programming Apps
----------------

Tock apps for OpenTitan must be included in the Tock binary file flashed with the steps mentioned above.

Apps are built out of tree. Currently [libtock-rs](https://github.com/tock/libtock-rs) apps work well while [libtock-c](https://github.com/tock/libtock-c) apps require a special branch and complex work arounds. It is recomended that libtock-rs apps are used.

Once an app is built and a tbf file is generated, you can use `riscv32-unknown-elf-objcopy` with `--update-section` to create an ELF image with the
apps included. This procedure basically updates the section, where the apps are executed, of the executable of the kernel `opentita.elf` and outputs the 
`opentitan-app.elf`, which represents the executable including the apps.

The .apps section need to be updated with the `.tbf` file of the app. (This is generated in the libtock-rs).

```shell
$ riscv32-unknown-elf-objcopy \
    --update-section .apps=<...>/libtock-rs/target/riscv32imc-unknown-none-elf/tab/opentitan/hello_world/rv32imc.tbf \
    <...>/tock/target/riscv32imc-unknown-none-elf/release/opentitan.elf\
    <...>/tock/target/riscv32imc-unknown-none-elf/release/opentitan-app.elf
```
The board cannot be flashed using an elf file that is why you will then to convert  `opentitan-app.elf` to a binary file by issuing:

```shell
$ riscv32-unknown-elf-objcopy --output-target=binary \
    <...>/tock/target/riscv32imc-unknown-none-elf/release/opentitan-app.elf \
    <...>/tock/target/riscv32imc-unknown-none-elf/release/opentitan-app.bin
```

The OpenTitan Makefile can also handle this process automatically. Run the `flash-app` make target:

```shell
$ make flash-app APP=<...> OPENTITAN_TREE=/home/opentitan/
```

You will need to have the GCC version of RISC-V 32-bit objcopy installed as the LLVM one doesn't support updating sections.


Running in QEMU
---------------
The OpenTitan application can be run in the QEMU emulation platform, allowing quick and easy testing. QEMU can be started 
with Tock using the `qemu` make target. To quit qemu, use Ctrl-A and x:
 Qemu is way faster than Verilator, which is the supported simulator of OT 

```shell
$ make OPENTITAN_BOOT_ROM=<path_to_opentitan>/sw/device/boot_rom/boot_rom_fpga_nexysvideo.elf qemu
```

Where OPENTITAN_BOOT_ROM is set to point to the OpenTitan ELF file. This is usually located at `build-out/sw/device/boot_rom/boot_rom_fpga_nexysvideo.elf` in the OpenTitan build output. QEMU can be started with Tock and a userspace app with the `qemu-app` make target:

```shell
$ make OPENTITAN_BOOT_ROM=<path_to_opentitan/build-out/sw/device/boot_rom/boot_rom_fpga_nexysvideo.elf> APP=/path/to/app.tbf qemu-app
```

The TBF must be compiled for the OpenTitan board which is, at the time of writing, supported for Rust userland apps using libtock-rs. For example, you can build
the Hello World exmple app from the libtock-rs repository by running:
```
$ cd [LIBTOCK-RS-DIR]
$ make flash-opentitan
$ tar xf target/riscv32imac-unknown-none-elf/tab/opentitan/hello_world.tab
$ cd [TOCK_ROOT]/boards/opentitan
$ make APP=[LIBTOCK-RS-DIR]/rv32imac.tbf qemu-app
```

