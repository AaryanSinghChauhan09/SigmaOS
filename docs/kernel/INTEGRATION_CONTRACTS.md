# SigmaOS Kernel Integration Contracts

## Overview

Subsystem integration contracts make explicit the requirements, APIs, dependencies, and failure policies for every kernel module in SigmaOS.

---

## Subsystem Integration Contracts

```toml
[subsystem.physical_allocator]
public_api = ["alloc_frame", "free_frame", "available_frames"]
requires = ["bootloader_memory_map"]
runtime_entry = "crate::memory::physical"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "unmap_range", "switch_page_table"]
requires = ["physical_allocator", "page_tables", "interrupt_safety"]
runtime_entry = "crate::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.virtual_memory]
public_api = ["allocate_vma", "free_vma", "protect_vma"]
requires = ["paging", "physical_allocator"]
runtime_entry = "crate::mm::virtual"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.swap]
public_api = ["swap_out_page", "swap_in_page", "enqueue_swap_op"]
requires = ["virtual_memory", "block_storage"]
runtime_entry = "crate::swap::core"
boot_dependency = "optional"
failure_policy = "degrade_and_log"

[subsystem.scheduler]
public_api = ["schedule_next", "add_thread", "exit_thread", "yield_now"]
requires = ["run_queue", "timer_interrupt", "context_switch"]
runtime_entry = "crate::scheduler::core"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.syscall_dispatcher]
public_api = ["dispatch_syscall", "register_syscall_handler"]
requires = ["capability_checker", "virtual_memory", "scheduler"]
runtime_entry = "crate::kernel::syscalls"
boot_dependency = "required"
failure_policy = "return_errno"

[subsystem.vfs]
public_api = ["vfs_open", "vfs_read", "vfs_write", "vfs_mount", "vfs_close"]
requires = ["fd_table", "inode_manager", "mount_manager", "permission_engine"]
runtime_entry = "crate::fs"
boot_dependency = "required"
failure_policy = "return_errno"

[subsystem.usb_subsystem]
public_api = ["probe_usb_device", "dispatch_urb", "reset_port"]
requires = ["pci_bus", "interrupt_manager"]
runtime_entry = "crate::drivers::usb_subsystem"
boot_dependency = "optional"
failure_policy = "disable_device_and_log"
```

---

## Contract Rules

1. Every subsystem MUST specify its `public_api`, `requires`, `runtime_entry`, `boot_dependency`, and `failure_policy`.
2. Changes to public APIs require updating integration tests in `tests/kernel/integration/`.
3. Unrecoverable failures in `required` boot dependencies must yield structured diagnostic panic reports.
