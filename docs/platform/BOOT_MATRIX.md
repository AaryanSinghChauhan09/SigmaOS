# SigmaOS Platform Boot Readiness Matrix

This document tracks platform boot capabilities, boot smoke validation statuses, shell entry verification, and validation history across QEMU virtualized and bare-metal targets.

## Status Definitions
- **Code Exists (`0`)**: Subsystem logic or boot stage code is present in repository.
- **Tests Pass (`1`)**: Standalone or unit test suite compiles and passes cleanly.
- **System Behavior Verified (`2`)**: Boot-to-shell or diagnostic prompt marker verified in execution logs.

---

## Target Architecture Boot Status

| Platform / Target | Target Triple / Machine | Bootloader | Shell Marker Reached | Verification Level | Last Validated |
|---|---|---|---|---|---|
| **QEMU x86_64** | `qemu-system-x86_64 -machine q35` | Multiboot2 / Limine | `SIGMAOS_BOOT_SUCCESS` | System Behavior Verified (`2`) | Monthly Rehearsal |
| **QEMU AArch64** | `qemu-system-aarch64 -machine virt` | UEFI / EDK2 | `SIGMAOS_AARCH64_INIT` | Tests Pass (`1`) | Monthly Rehearsal |
| **QEMU RISC-V 64** | `qemu-system-riscv64 -machine virt` | OpenSBI | `SIGMAOS_RISCV_INIT` | Tests Pass (`1`) | Monthly Rehearsal |
| **x86_64 Bare-Metal (UART Serial)** | Generic x86_64 16550 UART | Native Boot | `SIGMAOS_SERIAL_READY` | System Behavior Verified (`2`) | Monthly Rehearsal |

---

## Boot Sequence Pipeline Stages

1. **Stage 0 (Firmware Entry)**: Multiboot2 header verification, GDT/IDT initial setup, 64-bit long mode transition.
2. **Stage 1 (Memory Initializer)**: Physical frame allocator initialization, 4-level paging map setup (`0x0000_0000_0000` to kernel space).
3. **Stage 2 (Subsystem Initialization)**: Process manager PCB table, scheduler core, storage block manager, VFS root mount.
4. **Stage 3 (Userspace Init)**: PID 1 process execution (`/bin/sigma_init`), interactive shell prompt output (`sigma_shell>`).

---

## Boot Smoke Validation Command Reference

```bash
# Execute automated QEMU x86_64 boot smoke harness
./scripts/ci/qemu_boot_smoke.sh

# Run Python boot-to-shell log analyzer
python3 tests/boot/boot_to_shell_test.py
```
