# 📦 AI Agents Block Storage Management Specification (`docs/AI_AGENTS_BLOCK_STORAGE_MANAGEMENT.md`)

This specification defines block storage device abstractions, NVMe/AHCI driver protocols, I/O queue scheduling, and storage encryption policies for autonomous AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) in SigmaOS.

---

## 1. Block Device Subsystem Architecture (`src/kernel/block_dev.rs`)

AI agents manage storage abstractions:
- **Logical Block Addressing (LBA)**: Sector size normalization (512B / 4096B Advanced Format).
- **Request Queue Management**: Asynchronous block request queuing, request merging, and batch execution.
- **Partition Table Scanning**: MBR, GPT, and FreeBSD disklabel partition discovery.

---

## 2. NVMe PCIe & AHCI Storage Drivers (`src/driver/nvme_storage.rs`, `src/driver/ahci_sata_controller.rs`)

- **NVMe Controller Driver**:
  - 64-byte Submission Queue Entries (SQE) and 16-byte Completion Queue Entries (CQE).
  - PCIe MMIO doorbell register updates and admin/io queue ring buffer processing.
- **AHCI SATA Controller Driver**:
  - Frame Information Structure (FIS) command lists and Received FIS structures.
  - Native Command Queuing (NCQ) processing for parallel SATA drive commands.

---

## 3. I/O Scheduling, TRIM & Storage Encryption

- **I/O Schedulers**: BFQ (Budget Fair Queueing) and Kyber low-latency I/O scheduling queues.
- **TRIM / UNMAP Wear Leveling**: Garbage collection and block discard commands for NVMe and SATA SSDs.
- **Transparent Disk Encryption**: LUKS2 (AES-XTS-256) and FreeBSD GELI disk encryption layers.

---

## 4. AI Agent Block Storage Responsibilities

- **⚡ Bolt**: Profiles storage read/write throughput, measures IOPS latency, and tunes NVMe queue depths.
- **🎨 Palette**: Visualizes disk partition layout, storage utilization bar graphs, and SMART health metrics.
- **🛡️ Sentinel**: Enforces block encryption policies, audits sector write integrity, and validates LUKS2/GELI keys.
