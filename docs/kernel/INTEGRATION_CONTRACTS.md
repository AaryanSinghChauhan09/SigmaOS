# Kernel Subsystem Integration Contracts

This document defines the formal integration contracts for critical SigmaOS kernel subsystems.

---

## 1. Subsystem Definitions & Contracts

### Paging & Memory Management (`kernel::mm::paging`)
```toml
[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "unmap_range", "protect_range"]
requires = ["physical_allocator", "page_tables", "interrupt_safety"]
runtime_entry = "kernel::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"
```
*Invariants:* Page tables must enforce W^X (Write XOR Execute) isolation between user and kernel space. Address ranges must reject non-canonical addresses.

### Scheduler Core (`kernel::sched::core`)
```toml
[subsystem.scheduler]
public_api = ["schedule", "yield_now", "wake_up", "sleep_on", "set_priority"]
requires = ["timer_subsystem", "interrupt_controller", "process_table"]
runtime_entry = "kernel::sched::core"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"
```
*Invariants:* Scheduler run-queues must enforce starvation protection and Lost-Wakeup prevention.

### Virtual File System (`kernel::vfs`)
```toml
[subsystem.vfs]
public_api = ["sys_open", "sys_read", "sys_write", "sys_close", "mount_fs"]
requires = ["inode_manager", "block_cache", "permission_engine"]
runtime_entry = "kernel::vfs"
boot_dependency = "required"
failure_policy = "graceful_error_return"
```
*Invariants:* Path validation must enforce canonical directory traversal, permissions evaluation, and bounded file descriptor tables.

### Syscall Dispatcher (`kernel::syscall`)
```toml
[subsystem.syscall]
public_api = ["dispatch_syscall", "register_syscall_handler", "validate_user_ptr"]
requires = ["capability_engine", "process_context", "signal_delivery"]
runtime_entry = "kernel::syscall"
boot_dependency = "required"
failure_policy = "return_errno_to_userspace"
```
*Invariants:* Pointers received from userspace must be checked for boundary violations and validity before copy-in/copy-out.

### Driver Framework (`kernel::drivers`)
```toml
[subsystem.drivers]
public_api = ["probe_device", "remove_device", "dispatch_irq", "register_driver"]
requires = ["pci_bus", "io_manager", "dma_allocator"]
runtime_entry = "kernel::drivers"
boot_dependency = "optional"
failure_policy = "isolate_and_log_error"
```
*Invariants:* Driver probing and removal must be leak-free and handle hardware disconnects cleanly without kernel panic.
