# SigmaOS Kernel Internals

Deep dive into the memory architecture, process management, and scheduling models.

## Memory Management

*   **SLAB Allocator**: High-performance, contiguous block allocation designed under `#![no_std]`.
*   **NUMA Isolation**: CPU-socket aware memory zones to minimize cross-talk.
*   **THP (Transparent Huge Pages)**: Optimizes page table depth for high-throughput database operations.

## Process Scheduling

*   **Topological Sorting**: Boot and background daemons are scheduled based on dependency graphs.
*   **Real-Time Heartbeat**: Scheduler prioritizes execution paths with strict latency guarantees.\n
