# AI Agent Circular SCAN (C-SCAN) Policy Management Architecture

## Executive Overview

Circular SCAN (C-SCAN) is a deterministic elevator block I/O scheduling policy implemented in the SigmaOS kernel block layer (`src/kernel/block_dev.rs`). Unlike standard SCAN or LOOK elevators that sweep back and forth across sectors, C-SCAN services block I/O requests exclusively in one direction (ascending sector Logical Block Addresses - LBA). Once the disk head reaches the maximum requested sector address, it wraps back to sector 0 without servicing requests during the return movement, establishing a uniform maximum wait time guarantee across all disk sector ranges.

This document serves as the architectural reference for AI coding agents inspecting, optimizing, or extending block I/O scheduling, queue management, and elevator policies in SigmaOS.

---

## Subsystem Integration & Architecture

```
                                +-----------------------------------+
                                |    Subsystem / Application Bio    |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |      BlockDeviceManager           |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |    DeadlineScheduler (C-SCAN)     |
                                +-----------------------------------+
                                 /                                 \
                                v                                   v
                    +-----------------------+           +-----------------------+
                    | read_queue: BTreeMap  |           | write_queue: BTreeMap |
                    |  (sector -> Bio)      |           |  (sector -> Bio)      |
                    +-----------------------+           +-----------------------+
                                 \                                 /
                                  +---------------+---------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |       Ascending Sector Sweep      |
                                |  head_pos.. -> Sector Wrap-Around |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |       BlockDevice (RamDisk/NVMe)  |
                                +-----------------------------------+
```

### Core Components (`src/kernel/block_dev.rs`)

1. **`Bio` Request Abstraction**:
   - `id`: Unique atomic identifier (`u64`).
   - `sector`: Starting Logical Block Address (`u64`).
   - `count`: Sector length (`u32`).
   - `op`: I/O operation (`BioOp::Read`, `BioOp::Write`, `BioOp::Flush`, `BioOp::Discard`).
   - `priority`: Task priority level (`Idle`, `Normal`, `High`, `Sync`).

2. **`DeadlineScheduler` Queue Structure**:
   - `read_queue: BTreeMap<u64, Bio>`: Sorted red-black tree map indexing pending read requests by starting sector.
   - `write_queue: BTreeMap<u64, Bio>`: Sorted red-black tree map indexing pending write requests by starting sector.
   - `dispatch: VecDeque<Bio>`: Out-of-order priority FIFO for synchronous and flush operations.
   - `head_pos: u64`: Tracked current sector position of the disk head.

3. **C-SCAN Dispatch Rules**:
   - **Read Priority**: Read queues are serviced before write queues to minimize read latency for process thread execution.
   - **Ascending Sweep**: The scheduler invokes `range(head_pos..).next()` on the `BTreeMap` to select the lowest requested sector address greater than or equal to `head_pos`.
   - **C-SCAN Wrap-Around**: If no sector exists in `range(head_pos..)`, the scheduler wraps around by calling `iter().next()` to retrieve the smallest sector address in the entire tree.
   - **Head Advance**: Upon selecting a `Bio`, `head_pos` advances to `bio.end_sector()`.

---

## Mathematical Uniformity & Wait Bounds

In traditional LOOK/SCAN scheduling, sectors located near the center of the disk platters experience a lower expected wait time ($W_{mid} \approx T_{sweep} / 2$) compared to edge sectors ($W_{edge} \approx T_{sweep}$).

C-SCAN normalizes wait times across all sectors:

$$W_{max}(sector) = T_{sweep} + T_{return}$$

where $T_{sweep}$ is the time taken to sweep from sector $0$ to sector $S_{max}$, and $T_{return}$ is the wrap-around seek cost.

---

## Zero-Allocation Guardrails

AI agents maintaining or extending `src/kernel/block_dev.rs` must adhere to the following zero-allocation rules:
- `BTreeMap` lookups (`range()`, `iter()`) use borrow-based iterators and yield `&u64` keys without heap allocation.
- Removal via `remove(&sector)` executes in $O(\log N)$ time and reclaims node memory within the existing slab pool.
- Hot-path sector advancement does not allocate string names or formatting wrappers.

---

## Related Architectural References
- `src/kernel/block_dev.rs` - Master block device and scheduler implementation.
- `docs/AI_AGENT_IO_MANAGEMENT_ARCHITECTURE.md` - Sovereign I/O ring and VFS tiering architecture.
- `docs/AI_AGENT_CARRY_FLAG_MANAGEMENT_ARCHITECTURE.md` - ALU and register flag specifications.
