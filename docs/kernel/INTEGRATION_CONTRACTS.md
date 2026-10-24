# SigmaOS Kernel Subsystem Integration Contracts

## Overview
This document specifies explicit integration contracts for all kernel subsystems in SigmaOS. Each subsystem defines its public API, requirements, runtime entry point, boot dependency, and failure handling policy.

## Contract Matrix

```toml
[subsystem.boot]
public_api = ["boot_entry", "early_console_init", "parse_cmdline"]
requires = ["firmware_table", "arch_cpu"]
runtime_entry = "kernel::boot::entry"
boot_dependency = "critical"
failure_policy = "panic_with_diagnostic"

[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "unmap_range"]
requires = ["physical_allocator", "page_tables", "interrupt_safety"]
runtime_entry = "kernel::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.interrupts]
public_api = ["register_irq_handler", "enable_irq", "disable_irq", "eoi"]
requires = ["arch_idt_or_gic", "cpu_local"]
runtime_entry = "kernel::arch::interrupts"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.scheduler]
public_api = ["schedule", "add_task", "yield_task", "wake_task"]
requires = ["timer_interrupt", "process_table", "context_switch"]
runtime_entry = "kernel::scheduler::core"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.vfs]
public_api = ["sys_open", "sys_read", "sys_write", "mount", "unmount"]
requires = ["ramfs_root", "fd_table", "permission_engine"]
runtime_entry = "kernel::fs::vfs"
boot_dependency = "required"
failure_policy = "return_error_code"

[subsystem.syscalls]
public_api = ["syscall_dispatch", "register_syscall", "copy_from_user", "copy_to_user"]
requires = ["cpu_msr_or_trap", "capability_policy", "signal_dispatch"]
runtime_entry = "kernel::syscalls::dispatch"
boot_dependency = "required"
failure_policy = "return_errno"

[subsystem.drivers]
public_api = ["probe_device", "register_driver", "irq_handler"]
requires = ["pci_bus", "dma_allocator", "device_tree_or_acpi"]
runtime_entry = "kernel::drivers::manager"
boot_dependency = "optional"
failure_policy = "log_warning_and_disable"
```
