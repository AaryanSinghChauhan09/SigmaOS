# SigmaOS Boot State Machine Specification

## Boot State Machine Sequence

```
firmware (1) -> bootloader (2) -> kernel_entry (3) -> arch_init (4) ->
memory_init (5) -> interrupt_init (6) -> scheduler_init (7) ->
vfs_init (8) -> driver_init (9) -> userspace_start (10) -> shell_ready (11)
```

## Stage Descriptions

1. **firmware**: UEFI / BIOS initialization and boot disk selection.
2. **bootloader**: Kernel image ELF binary loading, memory map handover.
3. **kernel_entry**: Long mode / 64-bit vector entry, stack setup.
4. **arch_init**: CPU feature detection, GDT/IDT tables, LAPIC/IOAPIC init.
5. **memory_init**: Frame allocator, page tables, kernel heap, slab init.
6. **interrupt_init**: Interrupt vectors, timer tick interrupt, syscall MSRs.
7. **scheduler_init**: Run queues, idle process creation, initial context switch.
8. **vfs_init**: Root filesystem mount (`devtmpfs`, `procfs`, `sysfs`).
9. **driver_init**: Device enumeration (Serial, RTC, Storage, Input, Network).
10. **userspace_start**: First process spawn (`/sbin/init` or `/bin/sigmaos-shell`).
11. **shell_ready**: Deterministic shell marker emit `shell_ready`.

## Requirements

- Each stage emits a deterministic marker `[BOOT_STAGE_OK] <stage>`.
- Any stage failure triggers panic diagnostic reporting.
