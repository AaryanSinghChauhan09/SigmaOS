# SigmaOS Platform Boot Matrix and Validation Policy

## Target Architecture Support Matrix

| Architecture | Setup / Emulator | Firmware | Boot Stages Validated | Status | Evidence Link |
|---|---|---|---|---|---|
| `x86_64` | QEMU System x86_64 | SeaBIOS / OVMF UEFI | Firmware -> Bootloader -> Kernel -> Shell | Verified | `tests/boot/boot_to_shell_test.py` |
| `aarch64` | QEMU Virt | EDK2 / U-Boot | Firmware -> Bootloader -> Kernel Entry | Partial | `scripts/ci/qemu_boot_smoke.sh` |
| `riscv64` | QEMU Virt | OpenSBI | Firmware -> Bootloader -> Memory Init | Experimental | `tests/test_kernel_and_boot_v64.rs` |

---

## Boot Lifecycle Stages

1. **firmware**: Low-level hardware POST and BIOS/UEFI initial handoff.
2. **bootloader**: Multiboot2/UEFI image load, memory map retrieval, kernel image transfer.
3. **kernel_entry**: High-level Rust `kernel_main` entry, early serial console initialization.
4. **arch_init**: GDT/IDT/TSS setup (`x86_64`) or MMU/vector table setup (`aarch64`/`riscv64`).
5. **memory_init**: Physical frame allocator and 4-level paging virtual memory manager initialization.
6. **interrupt_init**: APIC/PIC or GIC interrupt controller setup and interrupt safety rules.
7. **scheduler_init**: Run-queue initialization, scheduler core timer ticks, idle task spawning.
8. **vfs_init**: Root VFS mount, devfs/sysfs/procfs synthesis.
9. **driver_init**: PCIe/USB enumerator and primary block device probe.
10. **userspace_start**: First init process execution (`/bin/init` or `/bin/shell`).
11. **shell_ready**: Diagnostic or interactive shell prompt available.

---

## Monthly Boot Rehearsal Protocol

- **Script**: `scripts/ci/qemu_boot_smoke.sh`
- **Harness**: `tests/boot/boot_to_shell_test.py`
- **Artifact**: `build/qemu_boot_smoke.log`
