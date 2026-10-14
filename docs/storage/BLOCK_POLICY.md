# SigmaOS Block Layer Specification & Policy

## Overview

The block layer is responsible for translating file system and Virtual File System (VFS) I/O requests into physical block transactions executed by underlying disk controllers (VirtIO-Blk, NVMe, AHCI/SATA, USB Storage).

## Block Request Lifecycle

Block requests follow an atomic state transition model to prevent memory races, request corruption, or double completions:

```text
[Free Slot] ---> Enqueue ---> [Enqueued] ---> Dispatch ---> [InFlight] ---> Complete ---> [Completed] ---> Reclaim ---> [Free Slot]
```

### Request Queue Boundaries and Limits

1. **Queue Capacity**: Each block device manager maintains a fixed, bounded queue capacity (default 256 requests per queue).
2. **Exhaustion Behavior**: When all request slots are occupied, allocation attempts must fail immediately with `BlockError::QueueFull` (or block safely on a bounded wait queue).
3. **Allocation Safety**: Request allocation uses lock-free or mutex-guaranteed atomic slot selection. Request recycling must clear all payload buffers and reset status flags before returning the slot to the free pool.

## Exact-Once Completion & Double Completion Prevention

- **Single Completion Invariant**: A request in `InFlight` state transition to `Completed` state exactly once.
- **Atomic State Guard**: The transition is performed using atomic swap or under lock. If a completion event arrives for a request already marked `Completed` or `Free`, the system logs a critical diagnostic warning and ignores the redundant signal without altering state.
- **Lost Completion Prevention**: Every in-flight request tracks a submission timestamp or tick counter. Timer-based queue audits scan for requests exceeding the maximum latency threshold (e.g., 5000ms), triggering controller resets or timing out requests with `BlockError::Timeout`.

## Device Class Requirements

### VirtIO Block (`virtio-blk`)
- Uses VirtQueue ring descriptors (`avail` and `used` rings).
- Request headers, status bytes, and scatter-gather buffer descriptors are pinned in physical memory.
- Interrupt and polling completion handlers process used descriptors deterministically.

### Solid State Drives & FTL Wear Leveling
- Supports wear leveling monitoring, TRIM/Discard commands, and bad block remapping.
- Sector alignment enforced (4096-byte physical sectors).

## Error Policy & Diagnostics

All block operations must return strongly typed `BlockError` variants:
- `BlockError::QueueFull`: Request capacity exceeded.
- `BlockError::DeviceDetached`: Hardware device unplugged during I/O.
- `BlockError::InvalidSector`: Request bounds check failed.
- `BlockError::Timeout`: Controller failed to respond within time window.
- `BlockError::IoError`: Physical media read/write failure.
