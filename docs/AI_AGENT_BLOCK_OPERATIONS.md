# AI Agent Block Operations in SigmaOS

## Overview
SigmaOS features an AI Agent-driven Block I/O Subsystem supervised by autonomous AI agents (**Bolt** ⚡, **Sentinel** 🛡️, **Palette** 🎨). This document defines operational directives, hardware submission protocols, and accounting interfaces for AI agents managing NVMe PCIe storage controllers, AHCI SATA host adapters, Btrfs Copy-on-Write (CoW) block allocation trees, and block accounting metrics.

AI agents interact directly with `src/driver/nvme_storage.rs` (`NvmeStorageController`), `src/driver/ahci_sata_controller.rs` (`AhciSataController`), `src/filesystem/btrfs_inspired.rs` (`BtrfsInspiredEngine`), and `src/resource/accounting.rs` (`ResourceAccountingManager`).

---

## 1. Block Subsystems & Architecture

### 1.1 NVMe PCIe Storage Subsystem (`NvmeStorageController`)
Implemented in `src/driver/nvme_storage.rs`. Manages PCIe NVMe SSD controllers using hardware queue pairs:
* **Submission Queue (SQ)**: 64-byte command entries posted to PCIe MMIO ring buffers.
* **Completion Queue (CQ)**: 16-byte status response entries processed upon doorbell interrupt notifications.
* **Direct DMA Transfers**: Physical block transfers execute via DMA, bypassing CPU byte loops.

### 1.2 AHCI SATA Host Controller (`AhciSataController`)
Implemented in `src/driver/ahci_sata_controller.rs`. Manages legacy and enterprise SATA drives via FIS (Frame Information Structure) command tables and PRDT (Physical Region Descriptor Table) scatter-gather entries.

### 1.3 Btrfs Copy-on-Write Block Tree Allocation
Implemented in `src/filesystem/btrfs_inspired.rs`. Manages storage blocks using CoW B-trees, extent allocation, block group chunks, and automatic block checksum verification.

### 1.4 Block I/O Accounting (`ResourceAccountingManager`)
Implemented in `src/resource/accounting.rs`. Tracks process-level block reads (`block_input_ops`), block writes (`block_output_ops`), and block I/O completion latency (`block_io_delay_ns`).

---

## 2. AI Agent Operational Rules & Directives

### 2.1 Queue Pointer & Memory Safety Rules
1. **MMIO Doorbell Verification**:
   AI agents must verify submission queue head/tail pointers prior to ringing the doorbell register to prevent hardware queue overrun deadlocks.
2. **Buffer Alignment**:
   DMA data buffers passed to NVMe/AHCI block operations must align to 4096-byte page boundaries (`PAGE_SIZE`).

### 2.2 Dynamic Block I/O Optimization
* **Bolt ⚡ Optimization**:
  During heavy package installations or database transactions, **Bolt** ⚡ batches block write commands into vectorized scatter-gather PRDT lists, reducing PCIe MMIO doorbell writes by up to 40%.

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Inspect NVMe controller health and active submission queues
sigma-block nvme-status --dev /dev/nvme0n1

# Query block I/O accounting and latency stats for process
sigma-block accounting --pid 1024

# Trigger Btrfs Copy-on-Write block tree defragmentation
sigma-block cow-defrag --mount /data
```
