# Formal Boot State Machine Specification

## Boot Lifecycle States

```text
firmware ──► bootloader ──► kernel_entry ──► arch_init ──► memory_init
                                                                │
shell_ready ◄── userspace_start ◄── driver_init ◄── vfs_init ◄── scheduler_init ◄── interrupt_init
```

---

## State Transitions & Criteria

| State | Entry Condition | Required Initialization | Failure Behavior | Diagnostic Output | Test Evidence |
|---|---|---|---|---|---|
| `firmware` | Hardware power on / reset | POST, BIOS/UEFI handoff | System halt / BIOS error | Serial firmware code | Firmware compliance test |
| `bootloader` | UEFI/GRUB payload start | Load kernel image, memory map setup | Boot error screen | Bootloader version & ELF header log | ISO smoke test |
| `kernel_entry` | ELF jump to `_start` | Set stack, disable IRQs, clear BSS | Early panic dump | `[BOOT] Kernel entry reached` | `qemu_boot_smoke.sh` |
| `arch_init` | `kernel_entry` complete | GDT, IDT, CPU feature probe, CR0/CR4 | Arch panic / reset | `[BOOT] CPU architecture initialized` | IDT/GDT unit tests |
| `memory_init` | `arch_init` complete | PMM, 4-level Paging, VMM, Slab alloc | Memory panic | `[BOOT] Memory manager online (pages: X)` | Memory test suite |
| `interrupt_init` | `memory_init` complete | APIC/PIC, STI, IDT handlers, Timer | IRQ exception trap | `[BOOT] Interrupts enabled` | Timer/IRQ tests |
| `scheduler_init` | `interrupt_init` complete | Run queues, Idle task, Context switch | Scheduler panic | `[BOOT] Scheduler active (CPUs: N)` | Context switch test |
| `vfs_init` | `scheduler_init` complete | Rootfs mount, devtmpfs, procfs, sysfs | Mount panic | `[BOOT] VFS rootfs mounted` | VFS unit tests |
| `driver_init` | `vfs_init` complete | Serial, Keyboard, Storage, Network | Isolated driver error | `[BOOT] Drivers probed: X/Y` | Hardware matrix audit |
| `userspace_start` | `driver_init` complete | Launch init process (`/sbin/init` or shell) | Init panic | `[BOOT] Spawning init process (PID 1)` | Execve/Process tests |
| `shell_ready` | `userspace_start` complete | Shell prompt displayed & accepting input | Interactive fallback shell | `[BOOT] SigmaOS Shell Ready` | `tests/boot/boot_to_shell_test.py` |
