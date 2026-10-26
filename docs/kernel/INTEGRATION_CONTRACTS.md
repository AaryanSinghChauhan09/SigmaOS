# SigmaOS Subsystem Integration Contracts

This specification defines public APIs, dependency requirements, runtime entry points, and failure policies for core kernel subsystems.

## Core Subsystem Contracts

### 1. Physical Memory Allocator (`physical_allocator`)
- **Public API:** `alloc_frame`, `free_frame`, `total_frames`, `allocated_frames`
- **Requires:** `boot_info_map`, `reserved_memory_ranges`
- **Runtime Entry:** `src/memory/physical.rs`
- **Boot Dependency:** Required
- **Failure Policy:** `panic_with_diagnostic`

### 2. Page Table & Virtual Memory Manager (`paging`)
- **Public API:** `alloc_pages`, `free_pages`, `map_range`, `unmap_range`, `switch_page_table`
- **Requires:** `physical_allocator`, `interrupt_safety`
- **Runtime Entry:** `src/mm/paging.rs`, `src/mm/virtual.rs`
- **Boot Dependency:** Required
- **Failure Policy:** `panic_with_diagnostic`

### 3. Swap Manager (`swap`)
- **Public API:** `allocate_swap_slot`, `free_swap_slot`, `read_swap_page`, `write_swap_page`
- **Requires:** `paging`, `block_device_manager`
- **Runtime Entry:** `src/swap/core.rs`
- **Boot Dependency:** Optional / Deferred
- **Failure Policy:** `return_oom_error`

### 4. Scheduler Core & Run Queue (`scheduler`)
- **Public API:** `enqueue_process`, `schedule_next`, `yield_current`, `set_priority`
- **Requires:** `paging`, `timer_subsystem`, `interrupt_controller`
- **Runtime Entry:** `src/scheduler/core.rs`, `src/scheduler/run_queue.rs`
- **Boot Dependency:** Required
- **Failure Policy:** `panic_with_diagnostic`

### 5. Virtual Filesystem & File Locks (`vfs`)
- **Public API:** `vfs_open`, `vfs_read`, `vfs_write`, `vfs_mount`, `vfs_flock`
- **Requires:** `paging`, `block_cache_manager`, `permission_engine`
- **Runtime Entry:** `src/fs/fd_table.rs`, `src/filesystem/core.rs`
- **Boot Dependency:** Required
- **Failure Policy:** `return_vfs_errno`

### 6. Syscall Dispatcher (`syscalls`)
- **Public API:** `dispatch_syscall`, `register_syscall_handler`, `validate_user_pointer`
- **Requires:** `scheduler`, `vfs`, `security_capability_policy`
- **Runtime Entry:** `src/kernel/syscalls.rs`
- **Boot Dependency:** Required
- **Failure Policy:** `return_syscall_errno`
