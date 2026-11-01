# SigmaOS Kernel Integration Contracts

## Purpose
Every kernel subsystem in SigmaOS must declare a formal integration contract specifying its public API, dependencies, entry point, boot criticality, and failure policies. No subsystem may interact with others through undocumented or implicit invariants.

---

## Subsystem Contract Declarations

```toml
[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "protect_range", "unmap_range"]
requires = ["physical_allocator", "page_tables", "interrupt_safety"]
runtime_entry = "kernel::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.scheduler]
public_api = ["enqueue_task", "dequeue_task", "yield_now", "schedule_next", "set_affinity"]
requires = ["timer_interrupt", "context_switch", "run_queue"]
runtime_entry = "kernel::scheduler::core"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.vfs]
public_api = ["mount", "unmount", "open", "read", "write", "close", "stat"]
requires = ["paging", "inode_table", "fd_table"]
runtime_entry = "kernel::fs::vfs"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.serial]
public_api = ["init", "write_char", "write_str", "read_char"]
requires = ["io_port_access"]
runtime_entry = "kernel::drivers::serial"
boot_dependency = "optional"
failure_policy = "fallback_to_memory_buffer"
```

---

## Contract Invariants
1. **Public API Bounds**: Subsystems must only expose functions marked in `public_api`. Unchecked internals must remain private.
2. **Explicit Requirements**: All dependencies in `requires` must be initialized before `runtime_entry` is invoked.
3. **Failure Policy Enforcement**: If a `required` boot dependency encounters an unrecoverable fault, it must issue a structured panic with diagnostics (`panic_with_diagnostic`). Non-essential subsystems must fallback gracefully without halting the kernel.
