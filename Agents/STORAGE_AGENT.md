# Block Device & Storage Drivers Component Agent

## Role
`SigmaStorage` governs block device drivers, request queuing, I/O scheduling, NVMe, AHCI/SATA, VirtIO-Block, SCSI controllers, and software RAID arrays for SigmaOS — inspired by Linux blk-mq and FreeBSD CAM.

## Linux & BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Block Layer | blk-mq multi-queue | CAM (Common Access Method) | SigmaBlock (lockless multi-queue) |
| I/O Scheduler | mq-deadline / kyber / bfq | CAM queues | SigmaIOSched (request merging) |
| Request Structure | `struct bio` / `request` | `struct bio` | SigmaBio |
| NVMe Driver | `nvme` kernel module | `nvme` driver | SigmaNVMe (PCIe SQ/CQ rings) |
| SATA/AHCI | `ahci` controller driver | `ahci` driver | SigmaAHCI (FIS command tables) |
| Software RAID | mdraid (RAID-0/1/5/10) | GEOM (gmirror, graid) | SigmaRAID |
| Disk Partitioning | GPT / MBR | GPT / MBR | SigmaGPT |
| Block Encryption | dm-crypt / LUKS2 | geli / softraid | SigmaCrypt (AES-XTS hardware) |

## Current SigmaOS Status
VirtIO-Block and AHCI SATA controller drivers implemented in `src/storage/` and `src/system/sigmastorage.rs`. Multi-queue NVMe driver, request merging, and TRIM/discard are active enhancement targets.

## Responsibilities & Architecture
- Block device registration and lifecycle (`/dev/sdX`, `/dev/nvmeXnY`).
- Multi-queue I/O request queuing with lockless submission and completion rings.
- I/O scheduler merging adjacent sector reads and writes to maximize throughput.
- Full NVMe 1.4+ driver with hardware submission/completion rings.
- AHCI/SATA controller driver with NCQ (Native Command Queuing).
- GPT (GUID Partition Table) and legacy MBR partition parsing.
- Automated TRIM/Discard commands for SSD flash wear-leveling.
- Software RAID-0, RAID-1, and RAID-5 redundancy layers.

## Source Files
- `src/storage/nvme.rs`
- `src/storage/ahci.rs`
- `src/storage/virtio_blk.rs`
- `src/storage/raid.rs`
- `src/storage/block_queue.rs`
- `src/system/sigmastorage.rs`

## Testing Strategy
```bash
rustc --test src/storage/ahci.rs --edition 2021 -o /tmp/test_storage && /tmp/test_storage --test-threads=1
```
- Raw sector read/write idempotency verification.
- Synthetic multi-threaded IOPS saturation benchmarks.
- Simulated unexpected controller disconnect and recovery tests.

## Future Development
1. **io_uring storage engine** — Zero-copy async storage submissions.
2. **NVMe over Fabrics (NVMe-oF)** — Disaggregated remote NVMe access over TCP/RDMA.
3. **ZNS (Zoned Namespaces)** — Native zoned SSD support eliminating drive-level garbage collection.
