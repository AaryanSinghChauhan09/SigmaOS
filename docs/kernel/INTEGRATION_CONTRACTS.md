# SigmaOS Kernel Subsystem Integration Contracts

## Architectural Design
SigmaOS enforces explicit integration contracts across all kernel execution subsystems. Code modules must adhere to declared public APIs, dependency orders, and failure handling policies.

## Subsystem Declarations

```toml
[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "unmap_range"]
requires = ["physical_allocator", "page_tables", "interrupt_safety"]
exposes_tests = ["tests/mm/paging.rs", "tests/kernel/missing_linux_kernel_components.rs"]
runtime_entry = "kernel::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.swap]
public_api = ["swap_out_page", "swap_in_page", "allocate_swap_slot", "free_swap_slot"]
requires = ["paging", "storage_block", "virtual_memory"]
exposes_tests = ["tests/swap/core.rs"]
runtime_entry = "kernel::swap"
boot_dependency = "optional"
failure_policy = "oom_kill_process"

[subsystem.scheduler]
public_api = ["schedule", "enqueue_task", "dequeue_task", "yield_now", "set_priority"]
requires = ["interrupt_controller", "context_switch", "timer"]
exposes_tests = ["tests/scheduler/core.rs", "tests/scheduler/fairness/fairness_test.rs"]
runtime_entry = "kernel::scheduler"
boot_dependency = "required"
failure_policy = "kernel_panic"

[subsystem.signals]
public_api = ["send_signal", "sigprocmask", "sigreturn", "deliver_pending_signals"]
requires = ["process_table", "user_context_switch"]
exposes_tests = ["tests/signal/signal_test.rs"]
runtime_entry = "kernel::signal"
boot_dependency = "required"
failure_policy = "terminate_target_process"

[subsystem.ipc]
public_api = ["pipe_create", "socket_pair", "shm_open", "msg_send", "msg_receive"]
requires = ["vfs", "capability_engine", "process_table"]
exposes_tests = ["tests/ipc_namespace_integration.rs", "tests/kernel/tier2_usability_stack.rs"]
runtime_entry = "kernel::ipc"
boot_dependency = "required"
failure_policy = "return_posix_errno"

[subsystem.vfs]
public_api = ["vfs_open", "vfs_read", "vfs_write", "vfs_close", "vfs_mount", "vfs_unmount"]
requires = ["fd_table", "inode_manager", "permission_engine"]
exposes_tests = ["tests/fs/fd_table.rs", "tests/filesystem/inode.rs"]
runtime_entry = "kernel::vfs"
boot_dependency = "required"
failure_policy = "return_posix_errno"

[subsystem.namespaces]
public_api = ["clone_namespaces", "unshare", "setns", "validate_isolation"]
requires = ["process_table", "capability_policy"]
exposes_tests = ["tests/namespace_integration_full.rs", "tests/user_namespace_tests.rs"]
runtime_entry = "kernel::namespaces"
boot_dependency = "required"
failure_policy = "return_posix_errno"
```
