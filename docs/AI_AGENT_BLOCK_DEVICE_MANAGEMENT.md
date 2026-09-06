# AI Agent Block-Oriented Device System Management Architecture in SigmaOS

This document specifies block-oriented device drivers, block operation dispatch engines, record blocking strategies, and storage management guidelines for AI agents working on storage, block drivers, and file systems in SigmaOS (`src/storage/block.rs`).

---

## 💾 1. Block-Oriented Devices & Drivers Subsystem

SigmaOS implements a unified block abstraction interface via the `BlockOrientedDevice` trait:

```
+---------------------------------------------------------------------------------+
| Trait: `BlockOrientedDevice` (`src/storage/block.rs`)                            |
| Defines `device_id()`, `device_class()`, `block_size()`, `total_blocks()`,       |
| `is_write_blocked()`, `read_block()`, `write_block()`, and `flush()`.           |
+---------------------------------------------------------------------------------+
          |                                        |
          v                                        v
+-----------------------------------+   +------------------------------------+
| `SsdBlockDevice`                  |   | `NvmeBlockDevice`                  |
| SSD device with Flash Translation |   | NVMe 2.0 multi-queue hardware block|
| Layer (FTL) wear-level tracking.  |   | device abstraction.                |
+-----------------------------------+   +------------------------------------+
```

### Supported Device Classes (`DeviceClass`)
- `Hdd` (Spinning Hard Disk Drive)
- `Ssd` (Solid State Drive with wear-level tracking)
- `Nvme` (High-performance NVMe Multi-Queue storage)
- `RamDisk` (Volatile in-memory block device)
- `VirtIoBlock` (VirtIO paravirtualized storage)
- `TapeDevice` (Sequential access stream storage)
- `LoopDevice` (File-backed loopback block device)

---

## ⚡ 2. Block Operation Engine & Request Dispatch

All block requests are processed through `BlockOperationEngine::execute_op`:

- **OpCodes (`BlockOpCode`):** `Read`, `Write`, `Flush`, `DiscardTrim`, `WriteSame`, `SecureErase`, `Barrier`, `DirectIoRead`, `DirectIoWrite`, `AsyncIoSubmit`.
- **Write-Blocking Enforcement:** All destructive operations (`Write`, `DiscardTrim`, `WriteSame`, `SecureErase`, `DirectIoWrite`) check `dev.is_write_blocked()` before execution and fail immediately with `BlockError::WriteBlocked` if write protection is enabled.

---

## 📐 3. Record Blocking Strategies & Classification

SigmaOS supports flexible record blocking via `RecordBlockingEngine` and `BlockingStrategy`:
1. `FixedLengthUnspanned`: Records fit cleanly inside single blocks without spanning boundaries.
2. `FixedLengthSpanned`: Fixed records can span across block boundaries.
3. `VariableLengthUnspanned`: Variable record lengths with 4-byte header descriptors.
4. `VariableLengthSpanned`: Variable record lengths spanning block boundaries.
5. `PermanentContiguous`: Uninterrupted contiguous allocation block layout.

Block classification (`SovereignBlockClassifier` & `BlockKind`):
- `BootBlock`: Stage1/Stage2 bootloader header
- `DataBlock`: File system payload data
- `DefinedBlock`: Pre-defined system descriptors
- `DispatchedBlock`: In-flight DMA transfer queue
- `FunctionOfBlock`: Inode/Superblock control metadata
- `ProcessControlBlock`: PCB register state storage
- `ScheduledBlock`: Elevator queue scheduled block

---

## 🛡️ 4. Rules & Directives for AI Agents

1. **Write-Blocking & Safety Checks**
   - Always verify `dev.is_write_blocked()` before submitting write, trim, or erase operations.
2. **Buffer Alignment & Block Boundaries**
   - Buffers provided to `read_block` and `write_block` must match `dev.block_size()` (e.g. 512, 4096 bytes).
   - Ensure `block_num < dev.total_blocks()` to prevent `BlockError::OutOfBounds`.
3. **Cache Invalidation**
   - When issuing `WriteSame`, `DiscardTrim`, or `SecureErase`, invalidate matching blocks in `SimpleBlockCache` using `invalidate(device_id, block_num)`.

---

## ⚙️ 5. Verification Commands for Storage Agents

- **Block Subsystem Unit Tests:**
  `cargo test --lib -- storage::block::tests`
- **Full SigmaOS Test Pipeline:**
  `./run_sigma_tests.sh`
