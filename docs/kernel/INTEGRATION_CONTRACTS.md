# SigmaOS Kernel Subsystem Integration Contracts

## Subsystem Contract Specification

Every kernel subsystem must define explicit initialization contracts, public APIs, runtime entrypoints, and failure policies.

```toml
[subsystem.physical_allocator]
public_api = ["alloc_frame", "free_frame", "get_free_memory_bytes"]
requires = ["serial_logging"]
runtime_entry = "kernel::memory::physical"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "unmap_range"]
requires = ["physical_allocator", "interrupt_safety"]
runtime_entry = "kernel::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.process_manager]
public_api = ["spawn_process", "terminate_process", "switch_context"]
requires = ["physical_allocator", "paging", "scheduler"]
runtime_entry = "kernel::process::table"
boot_dependency = "required"
failure_policy = "log_and_restart_init"

[subsystem.syscall_dispatcher]
public_api = ["dispatch_syscall", "register_syscall_handler"]
requires = ["process_manager", "paging", "security_capabilities"]
runtime_entry = "kernel::syscall::dispatcher"
boot_dependency = "required"
failure_policy = "return_posix_errno"

[subsystem.storage_block]
public_api = ["read_block", "write_block", "flush_cache"]
requires = ["physical_allocator", "pci_bus"]
runtime_entry = "kernel::storage::block"
boot_dependency = "required"
failure_policy = "return_io_error"

[subsystem.vfs]
public_api = ["mount", "unmount", "open", "read", "write", "close"]
requires = ["storage_block", "process_manager"]
runtime_entry = "kernel::vfs::core"
boot_dependency = "required"
failure_policy = "return_vfs_error"
```

---

## Contract Verification

Contract initialization order and dependency rules are tested via:

```bash
rustc --test tests/kernel/integration/subsystem_contracts_test.rs --edition=2021 -o build/test_contracts
./build/test_contracts
```
