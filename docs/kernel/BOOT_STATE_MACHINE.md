# SigmaOS Boot State Machine

## Overview

The Boot State Machine enforces an 11-stage sequential initialization sequence from system firmware to interactive shell availability.

## Boot States

1. **`firmware`**: Hardware power-on and UEFI/BIOS stage.
2. **`bootloader`**: Bootloader execution and kernel image loading into RAM.
3. **`kernel_entry`**: Kernel binary execution entry point, initial stack setup.
4. **`arch_init`**: Architecture-specific CPU initialization (GDT/IDT/control registers).
5. **`memory_init`**: Physical memory manager, page table initialization, and heap setup.
6. **`interrupt_init`**: Interrupt controller (APIC/PIC) enabling and handler registration.
7. **`scheduler_init`**: Process table and run-queue initialization, idle thread setup.
8. **`vfs_init`**: Virtual Filesystem root mount and device node setup.
9. **`driver_init`**: Peripheral driver probing (PCI, Serial, Display, Storage, Network).
10. **`userspace_start`**: Execution of initial userspace init process (`/sbin/init`).
11. **`shell_ready`**: Launch of interactive system shell and user session readiness.

## Verification

The boot state sequence is validated during QEMU boot smoke testing and kernel diagnostic logging. Failure at any critical stage triggers diagnostic report emission.
