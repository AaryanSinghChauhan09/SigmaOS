# AI Agent Guidelines: Disk Operation Management in SigmaOS

## 📌 1. Architectural Distinction & Overview

In **SigmaOS**, disk operation management provides high-throughput, low-latency block I/O abstractions across NVMe SSDs, AHCI SATA drives, VirtIO-Block micro-VMs, and USB Mass Storage devices.

As an AI agent developing block drivers, I/O schedulers, or file system backends, you must enforce **multi-queue submission/completion rings, $4\text{ KB}$ Advanced Format sector alignment, hardware DMA buffer pinning, and TRIM/Deallocate wear-leveling**.

---

## ⚙️ 2. Disk I/O Schedulers & Multi-Queue Architecture

SigmaOS implements a multi-queue block layer (`blk-mq`) with specialized I/O scheduling algorithms:

```
+-----------------------------------------------------------------------------------+
|                           SIGMAOS DISK I/O PIPELINE                               |
+-----------------------------------------------------------------------------------+
|  📥 Userland / VFS: Async System Calls (`io_uring` / Read / Write)                |
|  🤹 Block I/O Schedulers: Kyber (NVMe SSD) | BFQ (Interactive) | mq-deadline (SATA) |
|  🔄 Multi-Queue Engine: Per-CPU Hardware Submission / Completion Queue Pairs      |
|  💾 Hardware Controllers: NVMe 2.0 Fabrics/ZNS | AHCI SATA | VirtIO-Block           |
+-----------------------------------------------------------------------------------+
```

### 2.1 I/O Scheduler Selection Matrix:
* **Kyber (High-Speed NVMe SSDs):** Monitors read/write target latencies ($L_{\text{read}} \le 2\text{ ms}$, $L_{\text{write}} \le 10\text{ ms}$) and adjusts submission queue depths dynamically.
* **BFQ - Budget Fair Queueing (Desktop & Interactive Workstations):** Guarantees low latency and fair bandwidth budgets for interactive desktop applications.
* **mq-deadline (Rotational HDDs & Legacy SATA SSDs):** Enforces strict expiry deadlines ($500\text{ ms}$ for reads, $5\text{ s}$ for writes) and sorts requests by physical LBA sector order to minimize disk head seek times.

---

## 💾 3. Hardware Controller Operations & Multi-Queue Rings

### 3.1 NVMe 2.0 Submission & Completion Rings
* **Module Location:** `src/drivers/modern_nvme.rs`, `src/kernel/block_dev.rs`
* **Invariants:**
  * Uses hardware Submission Queue (SQ) and Completion Queue (CQ) ring buffers allocated in Non-Paged physical memory.
  * Doorbell register writes (`sq_tail_db`) notify NVMe controllers without interrupt overhead.
  * Zoned Namespaces (ZNS) support sequential-write-only flash zones to eliminate garbage collection write amplification.

### 3.2 AHCI SATA Command List & FIS Structure
* **Module Location:** `src/driver/ahci_sata_controller.rs`
* **Invariants:**
  * Uses Command Header tables and Frame Information Structure (FIS) packets mapped into $1\text{ KB}$ aligned physical DMA memory.
  * Command Table Physical Region Descriptor Tables (PRDT) specify physical memory page ranges for scatter-gather DMA transfers.

---

## 🛡️ 4. Sector Alignment, Bad Blocks & Discard/TRIM

1. **4 KB Advanced Format Sector Alignment:**
   * All disk partitions and VFS block allocations MUST align to $4096$-byte ($4\text{ KB}$) sector boundaries ($LBA \bmod 8 == 0$ for $512$-byte emulation drives).
2. **TRIM / Dataset Management Deallocate:**
   * File deletions issue asynchronous NVMe Dataset Management (Deallocate) or SATA TRIM commands to inform SSD controllers of unallocated flash blocks, preserving write endurance.
3. **Bad Block Mapping & SMART Telemetry:**
   * NVMe SMART health logs and AHCI bad block tables automatically remap failing physical LBAs to spare reserved sectors before reporting device errors.

---

## 🚫 5. AI Agent Rules & Code Patterns

1. **DMA Buffer Alignment:**
   * Physical memory buffers passed to disk DMA controllers MUST be aligned to $4096$ bytes (`align(4096)`) and pinned in memory (`NonPagedPool`).
2. **Lock-Free Hardware Queue Access:**
   * Per-CPU NVMe SQ/CQ pairs MUST be accessed using per-core lockless ring operations to eliminate inter-core lock contention.
3. **Atomic Asynchronous Completion:**
   * Disk I/O completions MUST execute via bottom-half tasklets or softirqs (`SoftIrqType::BlockIo`) rather than blocking hardware IRQ handlers.

---

## 🧪 6. Standalone Testing Procedures

AI agents can verify NVMe block device drivers, I/O schedulers, and AHCI SATA controllers via standalone unit compilation:

```bash
# Test NVMe 2.0 block device driver & submission ring
rustc --test --edition=2021 src/drivers/modern_nvme.rs -o build/nvme_tests && ./build/nvme_tests && rm build/nvme_tests

# Test kernel io_uring submission/completion queues
rustc --test --edition=2021 src/kernel/io_uring.rs -o build/io_uring_tests && ./build/io_uring_tests && rm build/io_uring_tests
```
