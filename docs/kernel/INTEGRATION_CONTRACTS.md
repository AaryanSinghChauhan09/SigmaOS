# SigmaOS Kernel Integration Contracts

## Overview

Integration contracts define formal boundaries, public APIs, dependencies, runtime entry points, and failure policies across all kernel subsystems.

## Subsystem Contract Specifications

```toml
[subsystem.physical_allocator]
public_api = ["allocate_frame", "deallocate_frame", "available_memory"]
requires = ["bootloader_memory_map"]
runtime_entry = "crate::memory::physical"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "unmap_range"]
requires = ["physical_allocator", "page_tables", "interrupt_safety"]
runtime_entry = "crate::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.interrupts]
public_api = ["register_handler", "enable_interrupts", "disable_interrupts"]
requires = ["arch_idt_setup"]
runtime_entry = "crate::interrupt"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.scheduler]
public_api = ["schedule", "add_process", "block_process", "yield_cpu"]
requires = ["interrupts", "paging", "timer"]
runtime_entry = "crate::scheduler::core"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.vfs]
public_api = ["open", "read", "write", "close", "mount", "unmount"]
requires = ["scheduler", "paging"]
runtime_entry = "crate::fs"
boot_dependency = "required"
failure_policy = "log_and_fallback"
```
