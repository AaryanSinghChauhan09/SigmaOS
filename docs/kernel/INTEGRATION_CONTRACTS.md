# Kernel Subsystem Integration Contracts

## Subsystem Specifications

### Paging Subsystem
```toml
[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range"]
requires = ["physical_allocator", "page_tables", "interrupt_safety"]
runtime_entry = "sigmaos::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"
```

### Scheduler Subsystem
```toml
[subsystem.scheduler]
public_api = ["schedule", "yield_now", "spawn_kthread"]
requires = ["timer_subsystem", "hardware_task_context", "smp_topology"]
runtime_entry = "sigmaos::kernel::scheduler"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"
```

### Virtual Filesystem (VFS)
```toml
[subsystem.vfs]
public_api = ["mount", "unmount", "open", "read", "write"]
requires = ["block_device", "page_cache", "permission_engine"]
runtime_entry = "sigmaos::fs::vfs"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"
```

### Syscall Dispatcher
```toml
[subsystem.syscalls]
public_api = ["syscall_dispatch", "validate_user_ptr"]
requires = ["capability_engine", "process_table", "signal_delivery"]
runtime_entry = "sigmaos::kernel::syscall"
boot_dependency = "required"
failure_policy = "return_errno_to_userspace"
```
