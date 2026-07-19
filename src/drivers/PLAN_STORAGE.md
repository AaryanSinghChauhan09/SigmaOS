# 💾 SigmaOS OOP Storage Subsystem Development Plan

This document presents the development plan for the **SigmaOS Block Storage Subsystem**. Inspired by the multi-queue scheduling and zero-copy block layers in distributions like **Red Hat Enterprise Linux (RHEL)** and **Ubuntu**, this plan outlines how SigmaOS supports older storage technologies (e.g. Floppy and IDE) alongside high-performance solid-state drives (e.g. PCIe Gen5/6 NVMe) with minimal memory footprint.

---

## 🏗️ 1. Block Subsystem Architecture

Every storage device in SigmaOS is represented as a polymorphic block device, exposing uniform APIs for block reading and writing while encapsulating sector-to-byte translations.

```
          +-------------------------------------------+
          |         Virtual Filesystem (VFS)          |
          +-------------------------------------------+
                                |
             +------------------+------------------+
             |                                     |
             v                                     v
+------------------------+             +------------------------+
|      BlockDevice       |             |   MultiQueueStorage    | (OOP Traits)
+------------------------+             +------------------------+
| - Floppy Controller    |             | - SATA3/AHCI SSD       |
| - IDE Hard Drive       |             | - PCIe Gen5/Gen6 NVMe  |
+------------------------+             +------------------------+
```

### 1.1 The Core Trait (`BlockDevice`)
Every storage driver must implement this abstract interface:

```rust
pub trait BlockDevice: PeripheralDevice {
    /// Returns the sector size (usually 512 or 4096 bytes)
    fn block_size(&self) -> u32;

    /// Returns total sectors available on the drive
    fn sector_count(&self) -> u64;

    /// Reads sequential sectors from the device into the raw buffer
    fn read_sectors(&mut self, lba: u64, sectors: u32, buffer: &mut [u8]) -> Result<usize, &'static str>;

    /// Writes sequential sectors to the device from the raw buffer
    fn write_sectors(&mut self, lba: u64, sectors: u32, data: &[u8]) -> Result<usize, &'static str>;
}
```

### 1.2 The Multi-Queue Trait (`MultiQueueStorage`)
Allows high-throughput NVMe controllers to utilize multiple parallel hardware command channels:

```rust
pub trait MultiQueueStorage {
    /// Creates a dedicated hardware queue pair linked to a specific CPU core
    fn setup_queue_pair(&mut self, core_id: usize) -> Result<(), &'static str>;
}
```

---

## 🔌 2. Supported Generations & Compatibility

SigmaOS implements a range of storage drivers to support various CPU architectures and virtualization host controllers:

### 2.1 Ancient: FloppyDiskDriver & IdeControllerDriver
- **Floppy Driver**: Communicates over legacy port `0x3F0` using direct PIO commands and ISA DMA Channel 2. Restricted to 1.44MB floppy formats.
- **IDE Driver**: Interfaces with primary/secondary channels (ports `0x1F0` / `0x170`). Uses LBA28/LBA48 addressing modes via traditional Port I/O register writes.

### 2.2 Modern: Sata3Controller & PCIe Gen5/Gen6 NVMe
- **SATA3 Controller**: Operates on AHCI standard with memory-mapped Command List Headers and FIS structures.
- **NVMe Driver**: Fully modern PCIe Gen5/Gen6 design. Leverages door-bell registers, MSI-X interrupt steering, and hardware-separated Submission and Completion Queue pairs (SQ/CQ) matching CPU topologies.

---

## ⚡ 3. UDF Block Virtualization Snips

To provide support for complex RAID arrays, custom sector caching, or encrypted file systems (LUKS equivalents):
- Users register short **UDF storage transforms** that intercept `read_sectors` or `write_sectors` requests.
- These transforms run in a sandboxed, zero-allocation context to perform AES-256 block encryption or mirror blocks onto secondary disks instantly.

---

## 📈 4. Roadmap and Milestones

1. **Phase 1: Subsystem Abstraction**
   - Implement `BlockDevice` trait and common sector boundary verification helper in `src/drivers/storage/mod.rs`.
2. **Phase 2: IDE Master/Slave PIO Driver**
   - Write standard LBA28 controller polling commands, and integrate into the main block pool.
3. **Phase 3: AHCI / SATA3 Controller**
   - Setup AHCI HBA registers, initialize Ports, map command lists, and execute standard DMA reads.
4. **Phase 4: PCIe NVMe Host Controller**
   - Parse PCIe BAR registers, map controller register block, initialize Admin queue, and dynamically allocate custom I/O queue pairs per CPU thread.
5. **Phase 5: Cache Layer & Filesystem Mounting**
   - Add a zero-allocation block cache and mount the unified `VirtualFilesystem` (VFS) to the root block storage partition.
