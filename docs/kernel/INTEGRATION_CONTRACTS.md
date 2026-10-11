# SigmaOS Core Kernel Subsystem Integration Contracts

This specification defines API stability guarantees, public interfaces, safety boundaries, and lifecycle behavior across SigmaOS kernel subsystems.

---

## 1. Paging & Memory Management (`kernel::mm::paging`)

- **Public API**:
  - `alloc_pages(count: usize) -> Option<PhysAddr>`
  - `free_pages(addr: PhysAddr, count: usize)`
  - `map_range(virt: VirtAddr, phys: PhysAddr, flags: PageTableFlags) -> Result<(), MmError>`
- **Prerequisites**: `physical_allocator`, `page_tables`, `interrupt_safety`
- **Runtime Entry**: `src/mm/paging.rs` / `src/memory/physical.rs`
- **Boot Dependency**: **`required`**
- **Failure Policy**: `panic_with_diagnostic`
- **Invariants**: All kernel page table mappings enforce W^X (Write XOR Execute) memory protection. No double-free allowed.

---

## 2. Syscall Dispatcher (`kernel::syscalls`)

- **Public API**:
  - `dispatch_syscall(sys_num: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize) -> isize`
- **Prerequisites**: `paging`, `process_manager`, `capability_checker`
- **Runtime Entry**: `src/kernel/tier1_kernel_execution_suite.rs` (`Posix450SyscallDispatcher`)
- **Boot Dependency**: **`required`**
- **Failure Policy**: `return_errno` (`-EINVAL`, `-EPERM`, `-EFAULT`)
- **Invariants**: Userland pointer arguments must be bounds-checked before dereferencing. Capability token checked on entry.

---

## 3. Storage & Block Layer (`kernel::storage`)

- **Public API**:
  - `read_blocks(device_id: u32, lba: u64, count: usize, buf: &mut [u8]) -> Result<usize, StorageError>`
  - `write_blocks(device_id: u32, lba: u64, count: usize, buf: &[u8]) -> Result<usize, StorageError>`
- **Prerequisites**: `virtio_driver`, `pci_bus`, `dma_allocator`
- **Runtime Entry**: `src/storage/block.rs` (`BoundedBlockDeviceManager`)
- **Boot Dependency**: **`required_for_rootfs`**
- **Failure Policy**: `retry_then_degrade`

---

## 4. Virtual Filesystem Layer (`kernel::vfs`)

- **Public API**:
  - `vfs_open(path: &str, flags: u32, mode: u32) -> Result<Fd, VfsError>`
  - `vfs_read(fd: Fd, buf: &mut [u8]) -> Result<usize, VfsError>`
  - `vfs_write(fd: Fd, buf: &[u8]) -> Result<usize, VfsError>`
- **Prerequisites**: `storage_block`, `fd_table`, `permission_engine`
- **Runtime Entry**: `src/fs/fd_table.rs`, `src/filesystem/core.rs`
- **Boot Dependency**: **`required`**
- **Failure Policy**: `return_errno`
