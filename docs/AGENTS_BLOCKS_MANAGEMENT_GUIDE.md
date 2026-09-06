# SigmaOS AI Agent Block Device & Storage Management Guide

This guide defines block layer architectures, NVMe / VirtIO block drivers, I/O schedulers, Copy-on-Write (CoW) transactional logging, and block deduplication standards for AI coding agents developing on SigmaOS.

---

## 1. Block Device Drivers & Abstractions

SigmaOS block device drivers interface with bare-metal hardware and microvm hypervisor layers:

* **NVMe Controllers:** PCIe-attached High-Performance NVMe queue pairs (Submission & Completion Queues).
* **AHCI SATA:** Legacy Serial ATA controller register programming.
* **VirtIO Block (`virtio-blk`):** MicroVM zero-copy virtqueue ring buffer block transfer interface (`src/unimplemented_tools.rs`).

```rust
pub trait BlockDevice {
    fn read_block(&self, lba: u64, buf: &mut [u8; 512]) -> Result<(), &'static str>;
    fn write_block(&mut self, lba: u64, buf: &[u8; 512]) -> Result<(), &'static str>;
    fn block_count(&self) -> u64;
}
```

---

## 2. Block Layer I/O Schedulers

SigmaOS includes Linux & BSD inspired block I/O scheduling algorithms in `src/scheduler/distro_schedulers.rs`:

* **Kyber I/O Scheduler:** Latency-targeted request throttling balancing read latency vs write throughput.
* **BFQ (Budget Fair Queueing):** High-fairness proportional bandwidth allocation per process.
* **POSIX Batch / Idle Scheduler:** Low-priority background block I/O deferral.

---

## 3. CoW Transactional Block Logging & Deduplication

### 3.1 JBD2-Style Merkle Transactional Ledger
Filesystem block commits are protected against power loss using JBD2-style Merkle root transaction logs (`src/unimplemented_features.rs` - `Jbd2TransactionLedger`):
* Incremental CRC32c checksum validation for block payloads.
* Atomic rollback capability restoring previous Merkle root hashes on crash recovery.

### 3.2 DragonFly HAMMER2 Block Deduplication
Block-level inline deduplication computes 64-bit Merkle hashes (`DragonFlyHammer2DeduplicationEngine`):
* Duplicate block writes increment ref-counts without allocating new physical blocks.
* Deduplication savings ratio is monitored in realtime (`get_dedup_ratio()`).

---

## 4. Block Buffer Cache Directives for AI Agents

1. **Alignment Verification:** Ensure block LBA offsets and physical buffer pointers are aligned to 512-byte or 4096-byte page boundaries.
2. **Zero-Copy Memory Mapping:** Prefer DMA ring buffers (`memory::resource_allocator::DmaRingBufferAllocator`) for block transfers to eliminate kernel-to-userspace buffer copying.
3. **CoW Snapshot Safety:** Never overwrite active CoW subvolume blocks directly; always duplicate physical frames on write (`ZfsBtrfsHybridSelfHealingCoW`).
