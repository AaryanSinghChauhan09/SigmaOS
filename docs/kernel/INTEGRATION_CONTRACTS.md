# SigmaOS Kernel Subsystem Integration Contracts

## Specification Format

Every kernel subsystem must publish an integration contract defining its public entry points, dependencies, failure policies, and verification status.

---

## Subsystem Contracts

### `subsystem.paging`
```toml
[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "unmap_range"]
requires = ["physical_allocator", "page_tables", "interrupt_safety"]
exposes_tests = ["tests/mm/paging.rs"]
runtime_entry = "kernel::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"
```

### `subsystem.scheduler`
```toml
[subsystem.scheduler]
public_api = ["schedule", "yield_now", "wake_up", "enqueue_process"]
requires = ["timer_interrupt", "process_table", "cpu_affinity"]
exposes_tests = ["tests/scheduler/core.rs"]
runtime_entry = "kernel::scheduler::core"
boot_dependency = "required"
failure_policy = "reschedule_or_panic"
```

### `subsystem.vfs`
```toml
[subsystem.vfs]
public_api = ["open", "read", "write", "close", "mount", "unmount"]
requires = ["inode_manager", "permission_engine", "fd_table"]
exposes_tests = ["tests/filesystem/core.rs"]
runtime_entry = "kernel::fs::vfs"
boot_dependency = "required"
failure_policy = "return_posix_errno"
```
