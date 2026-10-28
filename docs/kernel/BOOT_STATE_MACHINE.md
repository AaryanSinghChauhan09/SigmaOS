# SigmaOS Boot State Machine Specification

The SigmaOS kernel boot sequence transitions deterministically through 11 states:

```text
firmware -> bootloader -> kernel_entry -> arch_init -> memory_init ->
interrupt_init -> scheduler_init -> vfs_init -> driver_init ->
userspace_start -> shell_ready
```

## State Definitions & Invariants

| State | Entry Condition | Required Initialization | Failure Policy | Diagnostic Marker |
|---|---|---|---|---|
| `firmware` | Power on / reset | Firmware memory map & ACPI tables | Hard halt | `[BOOT] Reached firmware - OK` |
| `bootloader` | Handover from firmware | Load kernel image into RAM | Hard halt | `[BOOT] Reached bootloader - OK` |
| `kernel_entry` | Jump to `_start` | Stack pointer setup & CPU mode check | Panic | `[BOOT] Reached kernel_entry - OK` |
| `arch_init` | `kernel_entry` complete | GDT, IDT, TSS, CPU feature detection | Panic | `[BOOT] Reached arch_init - OK` |
| `memory_init` | `arch_init` complete | Physical frame allocator & 4-level paging | Panic | `[BOOT] Reached memory_init - OK` |
| `interrupt_init` | `memory_init` complete | LAPIC / IOAPIC / PIC vector remapping | Panic | `[BOOT] Reached interrupt_init - OK` |
| `scheduler_init` | `interrupt_init` complete | Idle process & `BoundedRunQueue` setup | Panic | `[BOOT] Reached scheduler_init - OK` |
| `vfs_init` | `scheduler_init` complete | Root mount point & process FD table init | Panic | `[BOOT] Reached vfs_init - OK` |
| `driver_init` | `vfs_init` complete | Serial, console, block & input drivers | Warning / Continue | `[BOOT] Reached driver_init - OK` |
| `userspace_start` | `driver_init` complete | Spawn `init` process / ELF loader | Panic | `[BOOT] Reached userspace_start - OK` |
| `shell_ready` | `userspace_start` complete | Interactive shell prompt ready | Fallback recovery | `[BOOT] Reached shell_ready - OK` |
