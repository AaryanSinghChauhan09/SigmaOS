# SigmaOS Block Layer Subsystem Policy

## Overview

The Block Layer Subsystem (`src/storage/block.rs` and `src/storage/virtio.rs`) manages raw storage hardware (NVMe, SSD, HDD, VirtIO-Block) and provides bounded, race-free I/O request queueing for upper filesystem and cache layers.

---

## Core Requirements & Specifications

### 1. Request Allocation & Enqueueing
- **Bounded Queue**: Every `BoundedBlockDeviceManager` instance enforces a strict `max_requests` capacity.
- **Exhaustion Handling**: When the queue reaches capacity, `submit_request` immediately returns `Err(BlockError::RequestExhaustion)`. Unbounded growth or blocking without timeout is strictly prohibited.
- **Thread Safety**: Request sequence IDs (`req_id`) are assigned atomically via `AtomicU64`.

### 2. Request Lifecycle & Exact-Once Completion
Request state transitions strictly follow:
```text
Unallocated -> Pending -> Dispatched -> Completed / Failed -> Freed
```

- **Exact-Once Completion**: A request in `Dispatched` or `Pending` state transitions to `Completed` or `Failed` exactly once upon completion call.
- **Double-Completion Prevention**: Invoking `complete_request` on an already finished request returns `Err(BlockError::DoubleCompletion)`.
- **Lost-Completion Prevention**: `detect_lost_completions(current_time, timeout)` scans dispatched requests exceeding timeout and marks them `Failed` with `BlockError::LostCompletion`.

### 3. SCSI Sense & Hardware Error Mapping
Hardware errors are translated into SCSI Sense Key and ASC/ASCQ codes (`ScsiSenseData`):
- `BlockError::OutOfBounds` -> `ScsiSenseKey::IllegalRequest`, `ScsiAscQualifier::LbaOutOfRange`
- `BlockError::WriteBlocked` -> `ScsiSenseKey::DataProtect`, `ScsiAscQualifier::WriteProtected`
- `BlockError::ReadFailed` -> `ScsiSenseKey::MediumError`, `ScsiAscQualifier::UnrecoveredReadError`

---

## Verification & Testing

Compliance is verified via `tests/storage/block.rs` and `tests/storage_block_tests.rs`:
- Bounded request allocation and exhaustion check.
- Exact-once completion and double-completion error check.
- VirtIO ring queue submission and completion.
- FTL wear leveling on SSD devices.
