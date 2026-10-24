# SigmaOS Boot State Machine

## Overview
SigmaOS enforces a strict 11-stage boot state machine from firmware initialization to shell readiness. Each stage requires explicit entry preconditions, initialization routines, failure recovery policies, and diagnostic log markers.

## The 11 Boot States

```text
1. firmware -> 2. bootloader -> 3. kernel_entry -> 4. arch_init ->
5. memory_init -> 6. interrupt_init -> 7. scheduler_init ->
8. vfs_init -> 9. driver_init -> 10. userspace_start -> 11. shell_ready
```

## State Definitions & Invariants

| State # | State Name | Requirements & Output | Failure Behavior |
|---:|---|---|---|
| 1 | `firmware` | System BIOS/UEFI initializes CPU and passes control | Halted by hardware |
| 2 | `bootloader` | Loads kernel ELF image and initrd into memory | System reboot |
| 3 | `kernel_entry` | Sets up boot stack and parses early command-line parameters | Early serial panic |
| 4 | `arch_init` | Configures GDT/IDT/MSRs and platform CPU structures | Panic with register dump |
| 5 | `memory_init` | Initializes physical frame allocator and 4-level x86_64 paging | Panic with memory map diagnostic |
| 6 | `interrupt_init` | Configures APIC/GIC, enables timer interrupts, and sets ISRs | Panic with interrupt vector report |
| 7 | `scheduler_init` | Spawns `init` process and configures per-CPU runqueues | Panic with process table report |
| 8 | `vfs_init` | Mounts root `ramfs`, populates `/proc`, `/sys`, and `/dev` | Panic with VFS mount diagnostic |
| 9 | `driver_init` | Probes PCI devices, storage controllers, and console TTY | Log warning & disable device |
| 10 | `userspace_start` | Transfers execution to init binary in Ring 3 userspace | Kernel panic (init failure) |
| 11 | `shell_ready` | Emits `[BOOT_STAGE_OK] shell_ready` marker and awaits input | Fallback to emergency shell |
