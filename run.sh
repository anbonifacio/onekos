#!/bin/bash

set -xue

# QEMU file path

QEMU=qemu-system-riscv32

# Build the kernel
cargo build --release

# Start QEMU
$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
      -kernel ./target/riscv32i-unknown-none-elf/release/onekos
