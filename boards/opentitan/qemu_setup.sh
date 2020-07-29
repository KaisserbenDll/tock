#!/bin/bash

cd ../../tools
git clone https://github.com/KaisserbenDll/qemu --depth 1 -b riscv-tock.next
cd qemu && ./configure --target-list=riscv32-softmmu
make -j $(nproc)
# add $TOCK_DIR/tools/qemu to .bashrc PATH ENV
