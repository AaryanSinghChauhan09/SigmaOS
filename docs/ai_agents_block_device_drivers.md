# Block Device Driver Management Guidelines for AI Agents (`docs/ai_agents_block_device_drivers.md`)

This document provides AI agents with directives, architectural standards, Rust types, and safety rules for managing **Block Device Drivers** across the SigmaOS driver stack.

---

## 1. Overview of Block Device Drivers in SigmaOS

In SigmaOS, block devices represent fixed-size addressable storage hardware (e.g., NVMe SSDs, SATA/AHCI drives, VirtIO virtual disks, USB Mass Storage, and SCSI/Storport devices). Block device drivers expose fixed sector/block read and write interfaces to the virtual filesystem (VFS), swap system, and transactional database filesystem (`FilesystemAsDatabaseEngine`).

Key characteristics:
* **Fixed-Size Sector Addressability:** Reads and writes operate on block/sector indexes (`u64`) with standardized block sizes (e.g., 512 bytes or 4096 bytes).
* **Scatter-Gather DMA Support:** Drivers construct DMA descriptors and physical region page lists (PRPs/SGLs) for zero-copy bulk data transfers.
* **Forensic Write-Blocking:** Security and forensics filter drivers enforce hardware and software write-protect policies.
* **Cross-OS Compat Shims:** Supports native Rust drivers, Linux VirtIO shims, and Windows Storport SCSI Miniport drivers.

---

## 2. Core Block Device Interfaces & Rust Traits

### 2.1 `BlockDevice` Trait (`src/driver/device.rs`)
The primary object-oriented abstraction for block devices in userland and kernel shims:

```rust
pub trait BlockDevice {
    fn read_block(&mut self, block: u64, buffer: &mut [u8]) -> Result<(), DeviceError>;
    fn write_block(&mut self, block: u64, buffer: &[u8]) -> Result<(), DeviceError>;
    fn block_size(&self) -> usize;
    fn total_blocks(&self) -> u64;
}
```

* **`SimpleBlockDevice` (`src/driver/device.rs`):** In-memory or virtual block device implementation used for initial ramdisks, swap backing, and testing.

### 2.2 `BlockDeviceDriver` Trait (`src/driver/framework.rs`)
Low-level driver framework interface for multi-block batch I/O and DMA page transfers:

```rust
pub trait BlockDeviceDriver: Driver {
    fn read_blocks(&mut self, block_idx: u64, buf: &mut [u8]) -> Result<usize, DriverError>;
    fn write_blocks(&mut self, block_idx: u64, buf: &[u8]) -> Result<usize, DriverError>;
}
```

### 2.3 `VirtioBlockShim` (`src/driver/shims.rs`)
VirtIO paravirtualized block storage driver supporting queue requests and asynchronous ring buffers:

```rust
pub struct VirtioBlockShim { ... }

impl VirtioBlockShim {
    pub fn push_block_request(&mut self, sector: u64, op: VirtioBlockOp, data: &[u8]) -> Result<u64, DriverError>;
}
```

### 2.4 `StorportDriver` (`src/driver/windows_compat.rs`)
Windows Storport SCSI miniport driver adapter converting SCSI Command Descriptor Blocks (CDB `READ_10` / `WRITE_10`) into standard block reads and writes.

---

## 3. Forensic Write-Blocking Filters (`UsbStorageFilterDriver`)

SigmaOS includes forensic write-blocking capabilities (`src/driver/framework.rs`) to prevent unauthorized modification of storage media:

```rust
pub struct UsbStorageFilterDriver {
    pub write_protected: bool,
}
```

* When `write_protected == true`, calls to `write_blocks()` or write IOCTLs return `DriverError::UnloadFailed` or `DeviceError::WriteProtected` immediately without issuing physical write commands to hardware.

---

## 4. CLI Commands for AI Agents

AI agents can manage and inspect block devices using standardized command-line tool outputs:

```bash
# List all registered block devices in JSON format
sigma-driver storage list --json

# Query device capacity and block size
sigma-driver storage info /dev/nvme0n1 --json

# Trigger a diagnostic block read self-test
sigma-driver storage test /dev/sda --block 0

# Set forensic write-blocking on a USB block device
sigma-driver storage write-protect /dev/sdb --enable
```

---

## 5. Directives & Safety Rules for AI Agents

When implementing or modifying block device drivers:

1. **Verify Sector Alignment:**
   Buffers passed to `read_block` and `write_block` MUST be a multiple of `block_size()`. Out-of-bounds or misaligned slices must return `DeviceError::BufferTooSmall` or `DeviceError::InvalidParameter`.
2. **Bounds Checking:**
   Always validate that `block_index < total_blocks()`. Return `DeviceError::InvalidBlock` for out-of-range requests.
3. **Respect Write Protection:**
   Check write-protection flags prior to issuing write commands to hardware or allocating DMA transfer descriptors.
4. **DMA Memory Pinning:**
   Buffers used for hardware DMA transfers MUST be pinned in physical memory (`mdl_pinning`) to prevent physical page translation changes during active transfers.

---

## 6. Verification & Testing Procedure

When modifying block storage drivers:

1. **Run Block Device Unit Tests:**
   ```bash
   cargo test --lib driver::device::tests::test_simple_block_device
   cargo test --lib driver::shims::tests::test_virtio_block_driver
   cargo test --lib driver::windows_compat::tests
   ```

2. **Run Full Kernel Test Suite:**
   ```bash
   ./run_sigma_tests.sh
   ```

---
*Maintained by the SigmaOS Core Driver & Storage Team.*
