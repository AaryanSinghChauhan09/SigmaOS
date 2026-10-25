# Kernel Subsystem Integration Contracts

## Principle
Every kernel subsystem must declare an explicit contract defining public APIs, runtime entry points, boot requirements, test coverage, and failure policies.

---

## Subsystem Contracts

### Paging Subsystem Contract
```toml
[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "unmap_range", "protect_range"]
requires = ["physical_allocator", "page_tables", "interrupt_safety"]
exposes_tests = ["unit", "integration"]
runtime_entry = "kernel::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"
```

### Scheduler Subsystem Contract
```toml
[subsystem.scheduler]
public_api = ["schedule", "add_task", "yield_task", "wake_up", "block_task"]
requires = ["timer_interrupt", "process_table", "context_switch_arch"]
exposes_tests = ["unit", "integration", "fairness", "affinity"]
runtime_entry = "kernel::scheduler::core"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"
```

### VFS Subsystem Contract
```toml
[subsystem.vfs]
public_api = ["sys_open", "sys_read", "sys_write", "sys_close", "sys_mount", "sys_unmount"]
requires = ["block_cache", "process_fd_table", "capability_policy"]
exposes_tests = ["unit", "conformance", "mount_lifecycle"]
runtime_entry = "kernel::fs::core"
boot_dependency = "required"
failure_policy = "return_posix_errno"
```

---

## Integration Verification Suite
All contracts are programmatically evaluated by `tests/kernel/integration/subsystem_contracts_test.rs`.
