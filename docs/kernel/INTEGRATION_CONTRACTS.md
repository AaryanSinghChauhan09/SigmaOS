# Kernel Subsystem Integration Contracts

This document formalizes the integration contracts, public API guarantees, prerequisites, boot dependencies, and failure handling policies for core SigmaOS kernel subsystems.

## Contract Specifications

```toml
[subsystem.physical_allocator]
public_api = ["alloc_frames", "free_frames", "get_free_memory_bytes"]
requires = []
runtime_entry = "kernel::mm::physical"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "unmap_range", "switch_page_table"]
requires = ["physical_allocator", "interrupt_safety"]
runtime_entry = "kernel::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.process_manager]
public_api = ["create_process", "terminate_process", "allocate_pid", "reap_zombies"]
requires = ["paging", "physical_allocator"]
runtime_entry = "kernel::process::manager"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.syscall_dispatcher]
public_api = ["register_syscall_handler", "dispatch_syscall", "validate_user_buffer"]
requires = ["paging", "process_manager"]
runtime_entry = "kernel::syscall::dispatcher"
boot_dependency = "required"
failure_policy = "fail_closed_with_errno"

[subsystem.storage_block]
public_api = ["read_blocks", "write_blocks", "flush_cache"]
requires = ["paging"]
runtime_entry = "kernel::storage::block"
boot_dependency = "optional"
failure_policy = "return_io_error"

[subsystem.vfs]
public_api = ["mount", "unmount", "open", "read", "write", "close"]
requires = ["storage_block", "syscall_dispatcher"]
runtime_entry = "kernel::vfs::core"
boot_dependency = "required"
failure_policy = "return_vfs_error"
```

## Contract Verification Rules
1. **Prerequisite Satisfaction**: No subsystem may execute public API calls before all declared `requires` prerequisites are initialized.
2. **Failure Isolation**: Subsystems marked with `failure_policy = "panic_with_diagnostic"` trigger a kernel panic report if initialization fails during early boot. Non-critical drivers use `return_io_error` or `fail_closed_with_errno`.
