# SigmaOS Kernel Subsystem Integration Contracts

## Integration Contract Schema

Subsystem integration boundaries are specified using formal contract declarations:

```toml
[subsystem.paging]
public_api = ["alloc_pages", "free_pages", "map_range", "unmap_range"]
requires = ["physical_allocator", "page_tables", "interrupt_safety"]
runtime_entry = "kernel::mm::paging"
boot_dependency = "required"
failure_policy = "panic_with_diagnostic"
```

## Defined Contracts

### 1. Physical Memory Allocator
- **ID**: `PhysicalAllocator`
- **Requires**: Firmware memory map / E820 / EFI memory map.
- **Boot Order**: 1
- **Failure Policy**: Panic with diagnostic report.

### 2. Virtual Memory & Paging
- **ID**: `Paging`
- **Requires**: `PhysicalAllocator`.
- **Boot Order**: 2
- **Failure Policy**: Panic with diagnostic report.

### 3. Process Manager & Scheduler
- **ID**: `ProcessManager`
- **Requires**: `Paging`, `PhysicalAllocator`.
- **Boot Order**: 3
- **Failure Policy**: Panic with diagnostic report.

### 4. Syscall Dispatcher
- **ID**: `SyscallDispatcher`
- **Requires**: `Paging`, `ProcessManager`.
- **Boot Order**: 4
- **Failure Policy**: Panic with diagnostic report.

### 5. Storage Block Layer
- **ID**: `StorageBlock`
- **Requires**: `Paging`.
- **Boot Order**: 5
- **Failure Policy**: Return typed block error / fail-closed.

### 6. VFS & Filesystem Core
- **ID**: `Vfs`
- **Requires**: `StorageBlock`, `SyscallDispatcher`.
- **Boot Order**: 6
- **Failure Policy**: Return typed VFS error / panic on root mount failure.
