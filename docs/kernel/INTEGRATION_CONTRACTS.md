# SigmaOS Kernel Subsystem Integration Contracts

## Overview

SigmaOS kernel components operate under explicit integration contracts. Each subsystem declares its public API surfaces, mandatory dependency preconditions, execution entrypoints, boot dependencies, and panic/failure policies.

## Subsystem Contract Specification

```toml
[subsystem.physical_memory]
public_api = ["allocate_frames", "free_frames", "available_memory"]
requires = ["boot_multiboot_info", "interrupt_safety"]
runtime_entry = "crate::memory::physical"
boot_dependency = "critical"
failure_policy = "panic_with_diagnostic"

[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "unmap_range"]
requires = ["physical_memory", "page_tables", "interrupt_safety"]
runtime_entry = "crate::mm::paging"
boot_dependency = "critical"
failure_policy = "panic_with_diagnostic"

[subsystem.scheduler]
public_api = ["schedule", "add_task", "yield_now", "block_current"]
requires = ["paging", "timer_interrupt", "process_table"]
runtime_entry = "crate::scheduler::core"
boot_dependency = "critical"
failure_policy = "reschedule_or_halt"

[subsystem.vfs]
public_api = ["vfs_open", "vfs_read", "vfs_write", "vfs_close"]
requires = ["scheduler", "inode_manager", "fd_table"]
runtime_entry = "crate::fs::fd_table"
boot_dependency = "required"
failure_policy = "return_posix_errno"

[subsystem.syscall_dispatcher]
public_api = ["dispatch_syscall", "register_syscall_handler"]
requires = ["vfs", "scheduler", "capability_checker"]
runtime_entry = "crate::kernel::tier1_kernel_execution_suite"
boot_dependency = "required"
failure_policy = "return_errno_to_userspace"
```

## Enforcement & Testing

Subsystem integration contracts are programmatically verified at boot time and in CI via `tests/kernel/integration/subsystem_contracts_test.rs`.
