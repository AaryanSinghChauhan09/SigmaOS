# SigmaOS Boot State Machine

## Overview
The SigmaOS Kernel Boot State Machine defines the 11 sequential initialization states from hardware reset to user shell readiness. Every state enforces entry conditions, required initialization routines, failure recovery policies, and diagnostic log markers.

---

## The 11 Sequential Boot States

```text
firmware -> bootloader -> kernel_entry -> arch_init ->
memory_init -> interrupt_init -> scheduler_init ->
vfs_init -> driver_init -> userspace_start -> shell_ready
```

---

## State Specifications

### 1. `firmware`
- **Entry Condition**: System reset / power-on.
- **Initialization**: POST, UEFI/BIOS initialization, handoff to bootloader.
- **Diagnostic Marker**: `[BOOT] 01: Firmware handoff complete`
- **Failure Policy**: Hardware fault / Halt.

### 2. `bootloader`
- **Entry Condition**: Limine or GRUB loader executed.
- **Initialization**: Read kernel image, parse ELF headers, construct physical memory map.
- **Diagnostic Marker**: `[BOOT] 02: Bootloader loaded kernel binary`
- **Failure Policy**: Abort boot with screen error.

### 3. `kernel_entry`
- **Entry Condition**: Control transferred to `_start`.
- **Initialization**: Clear BSS, initialize stack pointer, jump to Rust kernel main.
- **Diagnostic Marker**: `[BOOT] 03: Kernel entry reached`
- **Failure Policy**: Early serial panic diagnostic.

### 4. `arch_init`
- **Entry Condition**: Early kernel execution.
- **Initialization**: Set up CPU control registers, GDT, control CR0/CR4 flags.
- **Diagnostic Marker**: `[BOOT] 04: Architecture init complete`
- **Failure Policy**: Panic with diagnostic log.

### 5. `memory_init`
- **Entry Condition**: Memory map parsed from bootloader.
- **Initialization**: Initialize physical frame allocator, setup 4-level page tables, enable paging, construct virtual memory manager.
- **Diagnostic Marker**: `[BOOT] 05: Memory & paging initialized`
- **Failure Policy**: Panic with diagnostic log.

### 6. `interrupt_init`
- **Entry Condition**: Paging active.
- **Initialization**: Setup IDT, configure LAPIC/IOAPIC, enable PIT/APIC timers, enable interrupts (`sti`).
- **Diagnostic Marker**: `[BOOT] 06: Interrupts & timers active`
- **Failure Policy**: Panic with diagnostic log.

### 7. `scheduler_init`
- **Entry Condition**: Timer interrupts firing.
- **Initialization**: Construct PCB table, initialize priority run-queues, create idle task and PID 1 task container.
- **Diagnostic Marker**: `[BOOT] 07: Scheduler & run-queues active`
- **Failure Policy**: Panic with diagnostic log.

### 8. `vfs_init`
- **Entry Condition**: Scheduler operational.
- **Initialization**: Setup VFS root node, register devfs/procfs/sysfs, mount root partition.
- **Diagnostic Marker**: `[BOOT] 08: VFS & root filesystem mounted`
- **Failure Policy**: Panic with diagnostic log.

### 9. `driver_init`
- **Entry Condition**: VFS mounted.
- **Initialization**: Probe PCI bus, initialize serial, console, virtio-block, keyboard, and display drivers.
- **Diagnostic Marker**: `[BOOT] 09: Core drivers probed and initialized`
- **Failure Policy**: Log non-critical driver warning, degrade functionality gracefully.

### 10. `userspace_start`
- **Entry Condition**: Drivers ready.
- **Initialization**: Parse init executable, map user pages, transition CPU from Ring 0 to Ring 3.
- **Diagnostic Marker**: `[BOOT] 10: Transitioned to Ring 3 userspace init (PID 1)`
- **Failure Policy**: Emergency recovery shell / Kernel panic.

### 11. `shell_ready`
- **Entry Condition**: PID 1 spawns login/shell process.
- **Initialization**: Interactive prompt presented, input/output attached to console.
- **Diagnostic Marker**: `[BOOT] 11: Shell prompt ready`
- **Failure Policy**: Respawn shell session.
