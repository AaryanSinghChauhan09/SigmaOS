# Process Management & Scheduling

SigmaOS implements a modern, highly concurrent process management architecture designed for both general-purpose throughput and real-time responsiveness.

## Architecture

The Process Manager (`sigma-procmgr`) is responsible for:
- **PID Allocation**: 4096 maximum processes with O(1) recycling.
- **Process Control Blocks (PCB)**: Tracking state, CPU time, and priority.
- **Process Lifecycle**: `CREATED` → `READY` → `RUNNING` → `BLOCKED` → `TERMINATED`.

## Memory Virtualization

Integrated deeply with the Process Manager is the Virtual Memory Manager (VMM), which provides:
- **Per-Process Address Spaces**: 4-level page table walking (PML4 architecture).
- **Demand Paging**: Lazy allocation of physical memory.
- **Copy-on-Write (CoW)**: Zero-copy fork implementation.

## Task Scheduling

The scheduler utilizes a hybrid architecture:
- **MLFQ (Multi-Level Feedback Queue)**: 8 priority levels for general tasks.
- **EDF (Earliest Deadline First)**: Real-time guarantees for latency-sensitive tasks.
- **Per-CPU Run Queues**: Lockless scheduling on multi-core systems.
- **Starvation Prevention**: Periodic priority boosts.

## IPC (Inter-Process Communication)

- **Message Queues**: Ring-buffer backed, maximum 128 queues, 64 messages per queue.
- **Shared Memory**: Ref-counted, permission-checked shared memory segments.
- **Signals**: Basic signal delivery mechanism mirroring UNIX standard signals.
