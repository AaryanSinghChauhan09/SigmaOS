> Imported repository document from [`Agents/STORAGE_AGENTS.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/Agents/STORAGE_AGENTS.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

# SigmaOS Storage & Block Device Component Agents

## Component Overview

The storage subsystem manages block devices (SATA, NVMe, SCSI), partitioning (GPT, MBR), RAID, LVM, and advanced storage features (TRIM, NCQ, ZBC). This component is currently **PARTIALLY IMPLEMENTED** (basic block device layer exists) but lacks modern NVMe and software RAID support.

**Status**: 🟡 **PARTIAL** - Basic block I/O exists, missing NVMe, RAID, multipath

## Linux & BSD Inspiration Sources

### Primary References
- **Linux Block Layer** (`block/blk-core.c`): Generic block I/O subsystem
- **Linux NVMe Driver** (`drivers/nvme/host/`): NVMe 1.4+ with multipath
- **FreeBSD GEOM** (`sys/geom/`): Modular storage transformation layer
- **OpenBSD Softraid** (`sys/dev/softraid.c`): Software RAID 0/1/5/6/C
- **Linux md** (`drivers/md/`): MD RAID, device mapper, LVM
- **ZFS** (FreeBSD/Linux): Advanced filesystem with snapshots, RAID-Z

### Key Capabilities to Absorb
1. **NVMe Support** (Linux NVMe driver, I/O queues, namespaces)
2. **Software RAID** (OpenBSD softraid, Linux md)
3. **Multipath I/O** (Linux dm-multipath, FreeBSD gmultipath)
4. **TRIM/UNMAP** (SSD optimization)
5. **Zoned Storage** (ZBC/ZAC for SMR drives, ZNS SSDs)

## Agent Role: 💾 Storage

### Core Mission
Implement high-performance block I/O subsystem with NVMe support, software RAID, TRIM optimization, and partition management for SigmaOS.

### Operational Boundaries

**Always Do**:
- Use NVMe I/O queues (one per CPU core)
- Support GPT partitioning (BIOS MBR for legacy)
- Implement write-through caching for safety
- Validate all partition table entries (prevent overlap)
- Test with real hardware (NVMe, SATA SSD, USB flash)

**Ask First**:
- Adding hardware RAID controller support
- Implementing ZFS/Btrfs kernel modules
- Supporting exotic interfaces (Fibre Channel, iSCSI)

**Never Do**:
- Trust partition tables without validation (malicious GPT)
- Allow unaligned I/O (performance penalty)
- Skip barrier support (data corruption risk)
- Disable write caching without fsync guarantee

### Philosophy
Storage is truth: data loss is unacceptable. Performance matters: NVMe can saturate PCIe 4.0 (8 GB/s). Reliability: RAID saves data. Simplicity: block device abstraction hides complexity.

### Required Components

#### 1. **Block Device Core** (`src/drivers/block/core.rs`)
```rust
#![no_std]
// Generic block device interface
pub trait BlockDevice {
    fn read(&self, sector: u64, buffer: &mut [u8]) -> Result<usize, IoError>;
    fn write(&self, sector: u64, buffer: &[u8]) -> Result<usize, IoError>;
    fn sector_size(&self) -> u16; // Usually 512 or 4096
    fn total_sectors(&self) -> u64;
    fn flush(&self) -> Result<(), IoError>; // Force write-through
}

// Block I/O request queue (elevator algorithm)
// - FIFO, DEADLINE, CFQ (Complete Fairness Queueing)
// - I/O scheduler selection per-device
```

#### 2. **NVMe Driver** (`src/drivers/block/nvme.rs`)
```rust
#![no_std]
// NVMe 1.4 driver
// - Admin queue (identify controller, create I/O queues)
// - I/O submission/completion queues (per-CPU)
// - Namespace management (multiple logical drives per device)
// - MSI-X interrupt delivery (per-queue interrupts)
// - TRIM support (Dataset Management command)
// - Write Zeroes (fast zero-fill)
```

#### 3. **AHCI (SATA) Driver** (`src/drivers/block/ahci.rs`)
```rust
#![no_std]
// Advanced Host Controller Interface (SATA)
// - FIS (Frame Information Structure) handling
// - NCQ (Native Command Queuing) for concurrent I/O
// - Hot-plug support (eSATA)
// - TRIM support (DATA SET MANAGEMENT)
// - Power management (ALPM: Aggressive Link Power Management)
```

#### 4. **Partition Table Parser** (`src/drivers/block/partition.rs`)
```rust
#![no_std]
// GPT (GUID Partition Table) and MBR support
// - Parse GPT header (LBA 1) and partition entries
// - Validate CRC32 checksums
// - Support protective MBR (for BIOS compatibility)
// - Expose partitions as /dev/nvme0n1p1, /dev/sda1, etc.
```

#### 5. **Software RAID** (`src/drivers/block/raid.rs`)
```rust
#![no_std]
// MD RAID implementation
// - RAID 0: Striping (performance)
// - RAID 1: Mirroring (redundancy)
// - RAID 5: Single parity (N-1 capacity)
// - RAID 6: Double parity (N-2 capacity)
// - RAID 10: Striped mirrors (1+0)
// - Online resizing, hot spare management
```

#### 6. **Device Mapper** (`src/drivers/block/dm.rs`)
```rust
#![no_std]
// Logical volume management
// - Linear mapping (concatenate devices)
// - Striped mapping (RAID 0 equivalent)
// - Snapshot support (LVM snapshots)
// - Thin provisioning
// - dm-crypt integration (LUKS)
```

### Verification Protocol

```bash
# List block devices
lsblk
# Should show nvme0n1, sda, etc.

# Check NVMe info
nvme list
nvme id-ctrl /dev/nvme0
# Verify queue count, namespace count

# Partition a disk
parted /dev/sdb
  (parted) mklabel gpt
  (parted) mkpart primary 1MiB 100%
  (parted) quit
partprobe /dev/sdb

# Test RAID 1 (mirror)
mdadm --create /dev/md0 --level=1 --raid-devices=2 /dev/sdb1 /dev/sdc1
mdadm --detail /dev/md0
# Verify both disks active

# Benchmark NVMe performance
fio --filename=/dev/nvme0n1 --direct=1 --rw=randread --bs=4k --ioengine=libaio --iodepth=256 --numjobs=4 --name=nvme-bench
# Should achieve > 1M IOPS on modern NVMe

# Test TRIM support
hdparm -I /dev/sda | grep TRIM
fstrim -v /
# Verify TRIM executed
```

### Security Hardening Rules

1. **Validate Partition Tables**: Check GPT CRC32, reject overlapping partitions
2. **DMA Protection**: All block I/O through IOMMU
3. **Secure Erase**: Support ATA Secure Erase, NVMe Format NVM
4. **Read-Only Mode**: Support write-protect flag (forensics)
5. **Rate Limiting**: Prevent I/O storms (max 10K IOPS from unprivileged)

### Integration Points

**Dependencies**:
- `src/drivers/pci.rs` - NVMe/AHCI controller enumeration
- `src/kernel/memory.rs` - DMA buffer allocation (physically contiguous)
- `src/fs/` - Filesystem layer sits on block devices
- `src/crypto/` - dm-crypt for disk encryption
- `src/drivers/usb/storage.rs` - USB mass storage integration

**Exports to Userland**:
- `/dev/nvme0n1` - NVMe namespace block device
- `/dev/sda` - SATA disk
- `/dev/md0` - RAID array
- `/dev/mapper/vg0-lv0` - LVM logical volume
- `smartctl`, `hdparm`, `nvme-cli` - Diagnostic tools

### Zero-Dependency Philosophy

**No libblkid/libudev in Kernel**: Partition detection and device naming happen in kernel. Userland tools query via sysfs.

**Direct Hardware Access**:
```rust
// Example: Submit NVMe I/O command
unsafe fn nvme_submit_io(queue: &mut NvmeQueue, lba: u64, buffer: &[u8]) {
    let cmd = NvmeCommand {
        opcode: 0x01, // Write
        nsid: 1,
        prp1: buffer.as_ptr() as u64, // Physical address
        slba: lba,
        nlb: (buffer.len() / 512 - 1) as u16,
        ..Default::default()
    };
    queue.submit(&cmd);
    queue.ring_doorbell();
}
```

### Component Milestones

1. **Phase 1**: Generic block device interface + AHCI driver
2. **Phase 2**: GPT partition table parsing
3. **Phase 3**: NVMe driver (basic I/O)
4. **Phase 4**: NVMe multiqueue (per-CPU queues)
5. **Phase 5**: Software RAID 1 (mirroring)
6. **Phase 6**: RAID 5/6 (parity)
7. **Phase 7**: Device mapper + LVM

### Testing Requirements

- Unit tests: GPT parsing, RAID parity calculation
- Integration tests: Create filesystem on RAID, verify integrity
- Stress tests: 1M IOPS sustained (NVMe)
- Reliability: Pull disk from RAID 1, verify failover
- Performance: NVMe > 3 GB/s sequential read

### Performance Targets

- **NVMe IOPS**: > 1 million (4K random read, QD=256)
- **NVMe bandwidth**: > 3 GB/s sequential (PCIe 3.0 x4)
- **SATA bandwidth**: > 500 MB/s (SATA 3.0)
- **RAID 1 overhead**: < 10% (vs single disk)
- **Latency**: < 100 microseconds (NVMe read, QD=1)

### Error Handling

All storage errors MUST:
1. Log to kernel ring buffer (device, LBA, error code)
2. Mark bad sectors (do not retry indefinitely)
3. Trigger RAID rebuild on device failure
4. Notify userland via udev event (disk failure)
5. Never panic kernel (storage errors are recoverable)

### Block Device Naming

Follow Linux convention:
- `/dev/nvme0n1` - NVMe controller 0, namespace 1
- `/dev/nvme0n1p1` - Partition 1 on nvme0n1
- `/dev/sda` - First SATA disk
- `/dev/sda1` - First partition on sda
- `/dev/md0` - RAID array 0
- `/dev/mapper/vg0-lv0` - LVM logical volume

### NVMe Features

Implement these NVMe 1.4 features:
- **Multiple I/O queues**: One per CPU core
- **Namespace management**: Multiple logical drives per device
- **Dataset Management (TRIM)**: SSD wear leveling
- **Write Zeroes**: Fast zero initialization
- **Directives**: Stream hints for ZNS SSDs
- **Reservation**: Shared storage locking
- **Sanitize**: Secure data erasure

### RAID Algorithms

Implement these RAID levels:
- **RAID 0 (Striping)**: Capacity = N × disk, no redundancy
- **RAID 1 (Mirroring)**: Capacity = disk, survives N-1 failures
- **RAID 5 (Single Parity)**: Capacity = (N-1) × disk, survives 1 failure
- **RAID 6 (Double Parity)**: Capacity = (N-2) × disk, survives 2 failures
- **RAID 10 (1+0)**: Capacity = N/2 × disk, best performance + redundancy

### SMART Monitoring

Expose SMART (Self-Monitoring, Analysis and Reporting Technology):
- **SATA**: ATA SMART via AHCI pass-through
- **NVMe**: SMART / Health Information (Log Page 2)
- Attributes: Temperature, wear leveling count, uncorrectable errors
- Thresholds: Predictive failure warnings

### Documentation Requirements

- Block I/O architecture diagram (elevator, I/O scheduler)
- NVMe setup guide (multiqueue tuning)
- RAID creation guide (mdadm commands)
- LVM quickstart (create VG, LV, snapshots)
- Performance tuning (I/O scheduler selection, queue depth)
- Troubleshooting (disk not detected, slow performance)

---

## Journaling Rules (`.jules/storage.md`)

Record critical insights:
- NVMe quirks (specific controller bugs)
- RAID rebuild performance
- I/O scheduler regressions
- Filesystem corruption root causes

**Journal Entry Template**:
```
## [Date] - [Storage Issue Summary]
**Problem**: [Disk not detected / slow I/O / corruption]
**Root Cause**: [Driver bug / firmware issue / etc.]
**Solution**: [Quirk added / firmware updated / etc.]
**Hardware**: [NVMe/SATA model]
```

---

*This agent file defines the PARTIALLY IMPLEMENTED storage subsystem. NVMe and RAID support are CRITICAL for production deployment.*
