# SigmaOS Kernel Subsystem Integration Contracts

## Overview

SigmaOS kernel subsystems must adhere to explicit initialization order contracts, public API boundaries, dependency requirements, and panic/error handling policies.

## Subsystem Contract Declarations

### 1. Physical Memory Allocator (`PhysicalAllocator`)
- **Public API**: `alloc_frame`, `free_frame`, `available_frames`
- **Prerequisites**: Framebuffer/bootloader memory map handoff
- **Failure Policy**: Panic with diagnostic panic report (`KernelPanicDiagnosticReport`)
- **Boot Order Position**: 1

### 2. Paging & Virtual Memory (`Paging`)
- **Public API**: `map_page`, `unmap_page`, `translate_address`
- **Prerequisites**: `PhysicalAllocator`
- **Failure Policy**: Double fault exception / kernel panic
- **Boot Order Position**: 2

### 3. Process Manager (`ProcessManager`)
- **Public API**: `create_process`, `terminate_process`, `schedule`
- **Prerequisites**: `Paging`
- **Failure Policy**: Process exit code / kernel panic if `init` (PID 1) fails
- **Boot Order Position**: 3

### 4. Syscall Dispatcher (`SyscallDispatcher`)
- **Public API**: `dispatch_syscall`, `register_handler`
- **Prerequisites**: `Paging`, `ProcessManager`
- **Failure Policy**: Return negative POSIX error code (e.g. `-EINVAL`, `-EPERM`)
- **Boot Order Position**: 4

### 5. Storage Block Layer (`StorageBlock`)
- **Public API**: `read_blocks`, `write_blocks`, `register_device`
- **Prerequisites**: `Paging`
- **Failure Policy**: Return `BlockIoError`
- **Boot Order Position**: 5

### 6. Virtual File System (`Vfs`)
- **Public API**: `vfs_open`, `vfs_read`, `vfs_write`, `vfs_mount`
- **Prerequisites**: `StorageBlock`, `SyscallDispatcher`
- **Failure Policy**: Return POSIX file I/O error codes
- **Boot Order Position**: 6

## Verification

Initialization order contracts are verified by integration test:
- `tests/kernel/integration/subsystem_contracts_test.rs`
