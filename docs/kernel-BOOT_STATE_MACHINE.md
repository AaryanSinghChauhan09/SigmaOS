# SigmaOS Kernel Boot State Machine

The boot process of SigmaOS follows a strictly ordered, 11-stage deterministic state machine.

## Boot Sequence Flow

```text
firmware -> bootloader -> kernel_entry -> arch_init ->
memory_init -> interrupt_init -> scheduler_init ->
vfs_init -> driver_init -> userspace_start -> shell_ready
```

---

## Boot State Definitions

### 1. `firmware`
- **Description:** UEFI / BIOS initialization and handoff.
- **Entry Condition:** Power-on or system reset.
- **Initialization:** Hardware POST, DRAM training, ACPI tables location.
- **Failure Behavior:** System halt or firmware recovery shell.
- **Test Evidence:** QEMU/OVMF boot entry log.

### 2. `bootloader`
- **Description:** ISO / GRUB / UEFI loader execution.
- **Entry Condition:** Boot device selected.
- **Initialization:** Load kernel ELF, RAMDisk, and device tree / ACPI pointers into memory.
- **Failure Behavior:** Display "Bootloader Error" and stop.
- **Test Evidence:** Multiboot/UEFI header validation test.

### 3. `kernel_entry`
- **Description:** Handoff from bootloader to kernel `_start`.
- **Entry Condition:** Long mode active, page table base set, jump to kernel address.
- **Initialization:** Set up initial stack pointer, clear BSS, parse kernel command line.
- **Failure Behavior:** Triple fault / early serial panic.
- **Test Evidence:** Serial banner `[SIGMAOS KERNEL STARTING]`.

### 4. `arch_init`
- **Description:** CPU-specific architecture initialization.
- **Entry Condition:** Kernel entry completed.
- **Initialization:** GDT, IDT, TSS, CR0/CR4 CPU features (SSE, AVX, SMEP/SMAP).
- **Failure Behavior:** Panic with CPU feature error diagnostic.
- **Test Evidence:** CPU feature flag serial dump.

### 5. `memory_init`
- **Description:** Physical frame allocator and virtual paging initialization.
- **Entry Condition:** Arch init complete.
- **Initialization:** Buddy/frame allocator online, kernel higher-half 4-level paging active, heap slab online.
- **Failure Behavior:** Panic `OOM_DURING_BOOT`.
- **Test Evidence:** Physical page count and kernel heap test assertion.

### 6. `interrupt_init`
- **Description:** APIC/PIC and timer interrupt setup.
- **Entry Condition:** Memory online.
- **Initialization:** LAPIC/IOAPIC routing, PIT/HPET timer calibration, enable CPU interrupts (`sti`).
- **Failure Behavior:** Panic `INTERRUPT_SETUP_FAILED`.
- **Test Evidence:** Timer tick interrupt increment check.

### 7. `scheduler_init`
- **Description:** Preemptive scheduler and idle process creation.
- **Entry Condition:** Interrupts online.
- **Initialization:** Primary run queue setup, PID 0 (Idle) and PID 1 (Init) process control blocks created.
- **Failure Behavior:** Panic `SCHEDULER_INIT_FAILED`.
- **Test Evidence:** Context switch test execution.

### 8. `vfs_init`
- **Description:** Virtual File System root mount and RamFS setup.
- **Entry Condition:** Scheduler online.
- **Initialization:** Mount `/` (RamFS / DevFS / ProcFS), populate initial device nodes (`/dev/console`, `/dev/null`).
- **Failure Behavior:** Panic `VFS_ROOT_MOUNT_FAILED`.
- **Test Evidence:** VFS path lookup test for `/dev/console`.

### 9. `driver_init`
- **Description:** Built-in device driver probe and startup.
- **Entry Condition:** VFS online.
- **Initialization:** Serial, VirtIO-Block, VirtIO-Net, PCI bus scan, console display framebuffer.
- **Failure Behavior:** Log driver failure and attempt fallback console.
- **Test Evidence:** PCI device table enumeration log.

### 10. `userspace_start`
- **Description:** Transition from kernel ring 0 to user ring 3.
- **Entry Condition:** Built-in drivers initialized.
- **Initialization:** Load init process ELF (`/sbin/init`), setup ring 3 stack and segment registers, `sysret` / `iretq`.
- **Failure Behavior:** Panic `INIT_SPAWN_FAILED`.
- **Test Evidence:** Ring 3 entry serial message.

### 11. `shell_ready`
- **Description:** Interactive shell or automated test harness active.
- **Entry Condition:** Init process spawned user shell (`/bin/sh`).
- **Initialization:** Shell prompt prompt displayed, accepting input.
- **Failure Behavior:** System panic / dropped to kernel debug shell.
- **Test Evidence:** Marker `SIGMAOS_BOOT_SUCCESS_SHELL_READY`.
