# Boot State Machine Architecture

## Boot Sequence States

`firmware` -> `bootloader` -> `kernel_entry` -> `arch_init` -> `memory_init` -> `interrupt_init` -> `scheduler_init` -> `vfs_init` -> `driver_init` -> `userspace_start` -> `shell_ready`

## State Definitions and Requirements

### 1. firmware
- **Entry Condition**: System power-on or hardware reset.
- **Initialization**: EFI / VESA handoff, basic hardware verification.
- **Failure Behavior**: System beep / halt.

### 2. bootloader
- **Entry Condition**: Valid boot signature found.
- **Initialization**: Kernel image decompressed into RAM, initrd loaded.
- **Failure Behavior**: Boot menu fallback or Grub/EFISTUB error.

### 3. kernel_entry
- **Entry Condition**: Direct jump from bootloader to kernel entrypoint in 64-bit long mode.
- **Initialization**: Setup minimal stack and early serial logging.
- **Failure Behavior**: Triple fault / reset.

### 4. arch_init
- **Entry Condition**: Safe kernel entry.
- **Initialization**: CPU features (CPUID, CR0/CR4 flags, MSRs) initialized.
- **Failure Behavior**: Early panic report via serial.

### 5. memory_init
- **Entry Condition**: CPU architecture verified.
- **Initialization**: Physical memory map parsed, 4-level paging initialized, W^X pages configured.
- **Failure Behavior**: Panic report with physical memory map dump.

### 6. interrupt_init
- **Entry Condition**: Memory paging operational.
- **Initialization**: IDT populated, Local APIC / IO APIC configured, timer ticks enabled.
- **Failure Behavior**: Panic report with vector registration failure details.

### 7. scheduler_init
- **Entry Condition**: Timers operational.
- **Initialization**: Red-Black CFS / EEVDF scheduler queues created, idle task spawned.
- **Failure Behavior**: Panic report with process queue diagnostics.

### 8. vfs_init
- **Entry Condition**: Scheduler operational.
- **Initialization**: VFS root mount point mounted, devtmpfs, procfs, sysfs initialized.
- **Failure Behavior**: Panic report with mount point failure details.

### 9. driver_init
- **Entry Condition**: VFS mounted.
- **Initialization**: PCI bus scan, USB xHCI driver probe, storage controllers probed.
- **Failure Behavior**: Device warning log; continue boot if root drive available.

### 10. userspace_start
- **Entry Condition**: Storage drivers active.
- **Initialization**: Execve `/sbin/init` or `/bin/sh` ELF binary, Ring 0 to Ring 3 privilege transition.
- **Failure Behavior**: Panic with missing init message.

### 11. shell_ready
- **Entry Condition**: Userspace shell initialized.
- **Initialization**: Terminal prompt rendered, user input listening active.
- **Diagnostic Output**: `[BOOT_STAGE_OK] shell_ready` log marker emitted.
