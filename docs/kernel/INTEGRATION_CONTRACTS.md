# SigmaOS Kernel Integration Contracts

## Overview
Integration contracts define public interfaces, prerequisite subsystems, runtime entry points, boot dependencies, and failure policies for all core kernel subsystems.

---

## Contract Definitions

```toml
[subsystem.physical_memory]
public_api = ["allocate_frame", "free_frame", "get_free_frame_count"]
requires = ["bootloader_memory_map"]
runtime_entry = "crate::memory::physical"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "unmap_range"]
requires = ["physical_memory", "interrupt_safety"]
runtime_entry = "crate::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.virtual_memory]
public_api = ["allocate_user_region", "map_kernel_space", "protect_region"]
requires = ["paging", "physical_memory"]
runtime_entry = "crate::mm::virtual_mem"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.swap]
public_api = ["allocate_swap_slot", "read_swap_slot", "free_swap_slot"]
requires = ["paging", "block_storage"]
runtime_entry = "crate::swap::core"
boot_dependency = "optional"
failure_policy = "log_warning_fallback_oom"

[subsystem.scheduler]
public_api = ["schedule", "add_task", "yield_now", "block_task"]
requires = ["paging", "timer_interrupt"]
runtime_entry = "crate::scheduler::core"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.vfs]
public_api = ["open", "read", "write", "close", "mount", "unmount"]
requires = ["scheduler", "block_storage"]
runtime_entry = "crate::fs"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.syscall_dispatch]
public_api = ["dispatch_syscall", "register_syscall"]
requires = ["scheduler", "vfs", "virtual_memory"]
runtime_entry = "crate::kernel::syscall"
boot_dependency = "required"
failure_policy = "return_errno_or_kill_process"
```
