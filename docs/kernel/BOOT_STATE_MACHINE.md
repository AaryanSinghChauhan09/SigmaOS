# SigmaOS Boot State Machine

## Overview

The SigmaOS kernel boot process is modeled as an explicit, deterministic 11-stage finite state machine. Transitioning to a subsequent state requires satisfying all prerequisites of the current state and emitting structured boot diagnostic logs.

---

## State Machine Sequence

```text
firmware -> bootloader -> kernel_entry -> arch_init ->
memory_init -> interrupt_init -> scheduler_init ->
vfs_init -> driver_init -> userspace_start -> shell_ready
```

---

## State Invariants & Diagnostic Markers

| State | Entry Condition | Required Initialization | Failure Policy | Diagnostic Output |
|---|---|---|---|---|
| `firmware` | Power on / Reset | Hardware POST & handover | Halt | BIOS/UEFI handoff code |
| `bootloader` | Valid MBR/GRUB image | Load kernel ELF & Multiboot info | Reboot | Multiboot header verification |
| `kernel_entry` | ELF mapped in RAM | CPU state check & early GDT setup | Panic | `[BOOT] Kernel entry point reached` |
| `arch_init` | `kernel_entry` success | CPU features, IDT, GDT setup | Panic | `[BOOT] CPU architecture initialized` |
| `memory_init` | `arch_init` success | Physical frame allocator & 4-level paging | Panic | `[BOOT] Memory subsystem initialized` |
| `interrupt_init` | `memory_init` success | IOAPIC / LAPIC & timer interrupt vectoring | Panic | `[BOOT] Interrupt handlers attached` |
| `scheduler_init` | `interrupt_init` success | Bounded runqueues & init process PCB creation | Panic | `[BOOT] Scheduler core operational` |
| `vfs_init` | `scheduler_init` success | Root VFS ramfs mount & dev/proc nodes | Panic | `[BOOT] VFS root mounted` |
| `driver_init` | `vfs_init` success | PCI device scan, serial & storage probing | Log error & degrade | `[BOOT] Driver probing complete` |
| `userspace_start` | `driver_init` success | Spawn `/bin/init` or `/bin/sh` Ring 3 task | Retry / Fallback shell | `[BOOT] Transitioning to Ring 3` |
| `shell_ready` | `userspace_start` success | Interactive shell prompt ready | Fallback recovery shell | `[BOOT] SigmaOS Shell Ready` |

---

## Test & Verification

The boot state machine is validated continuously via `scripts/ci/qemu_boot_smoke.sh` and `tests/boot/boot_to_shell_test.py`.
