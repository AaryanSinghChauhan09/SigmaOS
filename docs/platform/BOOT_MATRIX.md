# SigmaOS Boot Verification Matrix

## Target Platforms

| Architecture | Boot Engine | QEMU Smoke Status | Hardware Test Status | Evidence Log |
|---|---|---|---|---|
| x86_64 | UEFI / Multiboot2 | Pass | Partial | `artifacts/qemu_x86_64_boot.log` |
| aarch64 | UEFI | Untested | Untested | Pending |
| riscv64 | OpenSBI | Untested | Untested | Pending |

## Boot Acceptance Criteria

1. Firmware loads bootloader and transitions control to kernel entry point.
2. Architecture, physical memory, paging, and APIC interrupts initialize without panic.
3. Scheduler spawns root process, mounts VFS root, and executes init binary.
4. Shell prompt marker `sigmaos-shell-ready` emitted to serial console within 10 seconds.
