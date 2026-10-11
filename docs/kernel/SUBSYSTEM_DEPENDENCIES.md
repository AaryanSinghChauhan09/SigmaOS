# TOML Declaration of Kernel Subsystem Integration Contracts

[subsystem.physical_allocator]
public_api = ["alloc_frame", "free_frame", "available_frames"]
requires = ["boot_memory_map"]
runtime_entry = "kernel::memory::physical"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range"]
requires = ["physical_allocator", "page_tables", "interrupt_safety"]
runtime_entry = "kernel::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"

[subsystem.syscall_dispatcher]
public_api = ["dispatch_syscall", "register_handler"]
requires = ["paging", "process_manager", "capability_checker"]
runtime_entry = "kernel::syscalls::posix450"
boot_dependency = "required"
failure_policy = "return_errno"

[subsystem.storage_block]
public_api = ["read_blocks", "write_blocks", "get_geometry"]
requires = ["virtio_driver", "pci_bus", "dma_allocator"]
runtime_entry = "kernel::storage::block"
boot_dependency = "required_for_rootfs"
failure_policy = "retry_then_degrade"

[subsystem.vfs]
public_api = ["vfs_open", "vfs_read", "vfs_write", "vfs_close"]
requires = ["storage_block", "fd_table", "permission_engine"]
runtime_entry = "kernel::fs::vfs"
boot_dependency = "required"
failure_policy = "return_errno"
