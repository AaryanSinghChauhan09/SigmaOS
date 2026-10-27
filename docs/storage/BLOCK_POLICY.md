# Block Device Manager Policy Specification

## 1. Scope & Purpose

This document specifies the operational policies and invariants enforced by the SigmaOS Block Device Manager (`src/storage/block.rs` and `src/storage/virtio.rs`).

---

## 2. Block Layer Contracts & Invariants

### 2.1 Request Enqueue and Dequeue Race Freedom
- Block requests are managed by `BoundedBlockDeviceManager`.
- Request submission (`submit_request`) assigns a monotonically increasing 64-bit request ID (`next_req_id`) using `AtomicU64`.
- Queue operations on `active_requests` and `pending_queue` execute within a synchronized locking discipline, guaranteeing race-free enqueue and dequeue ordering.

### 2.2 Request Reuse & Allocation Policy
- Completed or failed requests must be explicitly reaped via `reap_completed(req_id)` before their slot in `active_requests` is released.
- Active request count is capped at `max_requests`.
- If `active_requests.len() >= max_requests`, submission fails immediately with `Err(BlockError::RequestExhaustion)`.

### 2.3 Completion Semantics
- **Exactly-Once Completion**: Transitioning a request to `Completed` or `Failed` via `complete_request` must happen exactly once.
- **Double-Completion Prevention**: Attempting `complete_request` on an already completed/failed request returns `Err(BlockError::DoubleCompletion)`.
- **Lost-Completion Prevention**: Dispatched requests exceeding `timeout_ms` are detected by `detect_lost_completions` and transitioned to `Failed`. Attempting completion on a non-existent request ID returns `Err(BlockError::LostCompletion)`.

---

## 3. Device Support & Classes

1. **`NvmeBlockDevice`**: Multi-queue NVMe 2.0 device implementation supporting submission/completion queue pairs.
2. **`VirtIoBlockDevice`**: VirtIO-blk PCI driver implementation managing virtqueue descriptors and async I/O.
3. **`SsdBlockDevice`**: Solid-state block device simulation tracking FTL erase cycles per block and write-blocking flags.
4. **`ScsiSenseData`**: Structured SCSI sense key and ASC/ASCQ descriptor logging (`ScsiSenseKey`, `ScsiAscQualifier`).

---

## 4. Verification

Enforced via unit test suite `tests/storage/block.rs`:
- `test_bounded_block_manager_request_exhaustion`
- `test_exact_once_completion_and_double_completion_prevention`
- `test_virtio_block_device_lifecycle`
