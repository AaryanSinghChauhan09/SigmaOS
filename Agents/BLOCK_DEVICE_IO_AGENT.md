# Block Device I/O Agent

## Role
`SigmaBlockIO` implements the block device I/O layer for SigmaOS — managing request queues, I/O scheduling, storage device drivers, and the block layer abstraction between filesystems and hardware.

## Status
🟡 In Progress — Basic storage in `src/system/sigmastorage.rs`; full I/O scheduler and device driver model needed.

## Source File
`src/storage/` (`mod.rs`, `block_io.rs` planned), `src/system/sigmastorage.rs`

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Block layer | `block/` subsystem | `cam/` layer | SigmaBlock |
| I/O scheduler | mq-deadline / kyber / none | CAM queues | SigmaIOSched |
| Request queue | `struct request_queue` | `bio_queue` | SigmaReqQ |
| BIO | `struct bio` | `struct bio` | SigmaBio |
| Device model | `struct gendisk` | `struct disk` | SigmaDisk |
| NVMe driver | `nvme` module | `nvme` module | SigmaNVMe |
| SATA/AHCI | `ahci` module | `ahci` module | SigmaAHCI |
| Partitions | `GPT` / `MBR` | `GPT` / `MBR` | SigmaGPT |

## Responsibilities
- Block device registration and lifecycle (`SigmaDisk`)
- I/O request queuing: `SigmaBio { dev, sector, data, op: Read/Write }`
- I/O scheduler: mq-deadline (merge adjacent requests)
- NVMe driver: PCIe-based NVMe queues (submission/completion)
- AHCI/SATA driver for HDD/SSD
- GPT/MBR partition table parsing
- Disk partitioning API (`SigmaParted` integration)
- Asynchronous I/O completion callbacks
- Write barriers and flush commands
- `/dev/sdX`, `/dev/nvmeXnY` device nodes

## Implementation Notes
- `#![no_std]` — request queue as FIFO ring buffer
- `SigmaBio { dev_id: u32, lba: u64, len: u32, buf: *mut u8, op: BioOp, flags: u32 }`
- NVMe: submission queue (SQ) / completion queue (CQ) MMIO registers
- Merge: combine adjacent read requests into single larger I/O

## Testing
```bash
rustc --test src/storage/block_io.rs --edition 2021 -o /tmp/test_bio && /tmp/test_bio --test-threads=1
```

## Future Development
1. **io_uring storage** — Async block I/O via io_uring (Linux 5.1+)
2. **NVMe-oF** — NVMe over Fabrics (TCP/RDMA) remote storage
3. **Zoned storage** — Zoned Namespace (ZNS) SSD support
4. **Kyber I/O scheduler** — Latency-controlled scheduler for NVMe
5. **BFQ scheduler** — Budget Fair Queuing for desktop responsiveness
6. **Persistent memory** — PMEM/NVDIMM direct-access (DAX) mode
