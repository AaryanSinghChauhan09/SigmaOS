# QEMU Reference & Boot Architecture

## Purpose
This document specifies the QEMU platform requirements, memory layout, serial console flags, and boot verification parameters for SigmaOS smoke testing and development.

## Standard QEMU x86_64 Invocation
```bash
qemu-system-x86_64 \
    -m 2048 \
    -smp 2 \
    -drive file=iso_root/boot/sigmaos.iso,format=raw \
    -nographic \
    -serial stdio \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -no-reboot
```

## Key Flags & Parameters
- `-m 2048`: Allocates 2GB RAM to verify page table initialization and physical frame allocator bounds.
- `-smp 2`: Multi-core SMP initialization test.
- `-nographic -serial stdio`: Redirects serial port COM1 to stdout for deterministic CI log processing.
- `isa-debug-exit`: Allows the kernel to signal test completion or failure codes directly to QEMU exit codes.

## Boot Verification Protocol
1. Bootloader stage: GRUB2 / Limine / custom stage 1 load kernel binary into memory.
2. Kernel entry point: `kernel_main()` initializes serial log driver and outputs `[KERNEL_INIT_START]`.
3. Subsystem contract checks: Physical memory manager, page tables, scheduler, VFS, and drivers initialize.
4. Shell prompt: Interactive shell or automated runner outputs `SIGMAOS_BOOT_OK_SHELL_READY`.
5. Automated exit: Smoke harness triggers `isa-debug-exit` port `0xf4` with status code `0` (Success) or `1` (Failure).
