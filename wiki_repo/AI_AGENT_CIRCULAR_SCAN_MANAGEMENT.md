# 🔄 AI Agent Circular Scan (C-SCAN) Management in SigmaOS

## Executive Summary
The Circular Scan (C-SCAN) scheduling algorithm in SigmaOS regulates block I/O request queues for rotational hard disk drives (HDD) and multi-queue solid-state storage. Unlike standard SCAN (elevator algorithm) which sweeps back and forth, C-SCAN services pending read/write requests in one direction only (inner to outer LBA track cylinder). Upon reaching the maximum LBA boundary, the drive head immediately wraps back to LBA `0` without servicing requests during the return sweep, offering uniform wait-time variance across all LBA addresses. Autonomous AI Agents (**Bolt ⚡**, **Palette 🎨**, and **Sentinel 🛡️**) managing storage queues and disk driver schedulers must adhere to C-SCAN policies to prevent I/O starvation and bound tail latency.

---

## 1. C-SCAN Scheduling Algorithm Architecture

SigmaOS block device layer (`src/kernel/block_dev.rs`) integrates C-SCAN request sorting via binary insertion queues:

```
[ Head Traversal Direction ──► Outer Cylinder Track (Max LBA) ]
  LBA 0 ───────────────► LBA 2000 ───────────────► LBA 5000 (Max)
   ▲                                                  │
   └────────────────────── Instant Return ────────────┘
                  (No Request Servicing)
```

### Core Algorithmic Phases
1. **Unidirectional Sweep**: Pending requests with LBA $\ge$ current head position are serviced in ascending numerical order.
2. **Boundary Wrap-Around**: When no pending requests remain ahead of the head position, the drive head wraps around to the lowest pending LBA.
3. **Uniform Service Distribution**: Provides uniform maximum delay guarantees for requests arriving near cylinder boundaries compared to traditional bi-directional SCAN.

---

## 2. Queue Ordering & Starvation Prevention

To ensure bounded latency and fairness:
- **Batching Snapshots**: Newly arriving I/O requests are placed into a secondary staging queue during an active sweep cycle to prevent new arrivals from indefinitely extending the sweep.
- **Max Age Threshold**: Requests whose wait time exceeds `CSCAN_MAX_LATENCY_MS` trigger an immediate forced wrap or priority dispatch.

---

## 3. AI Agent Operational Guidelines

1. **Bolt ⚡ (Performance Optimization)**:
   - Apply C-SCAN to mechanical rotational media and sequential streaming workloads where seek time dominates latency.
   - For NVMe flash storage, dynamically fall back to Kyber/BFQ multi-queue fair queueing schedulers to avoid artificial seek overheads.

2. **Palette 🎨 (UX & Responsiveness)**:
   - Ensure interactive user desktop applications (file manager, media playback) receive priority LBA queue placement during C-SCAN sweeps.

3. **Sentinel 🛡️ (Security & Hardening)**:
   - Validate that block request LBA offsets never exceed device boundary limits (`LBA < max_lba`) before inserting into C-SCAN queues.
   - Protect storage metadata (superblock, inode tables) from queue starvation during heavy C-SCAN sequential sweeps.
