# SigmaOS Block Device Layer Policy

## Overview
The block device subsystem governs communication between the kernel filesystem/cache layers and
block-oriented physical or virtual storage controllers (`SsdBlockDevice`, `VirtIoBlockDevice`, `NvmeDevice`, etc.).
It enforces bounded allocation, race-free request dispatching, and exactly-once completion semantics.

## Core Requirements

### 1. Bounded Request Queuing
- All block device managers (`BoundedBlockDeviceManager`) enforce a fixed request queue depth.
- When all request slots are saturated, new submissions must return `BlockError::RequestExhaustion`.
- Unbounded queue growth is strictly prohibited to protect kernel address space.

### 2. Race-Free Enqueue and Dequeue
- Request IDs are generated using monotonic atomic counters (`AtomicU64`).
- Pending requests are maintained in structured FIFO queues with explicit concurrency synchronization.
- Requests transition through distinct states: `Queued -> InFlight -> Completed -> Reaped` (or `Failed`).

### 3. Exactly-Once Request Completion
- A request may only be completed once.
- Any attempt to complete a request that is not in `InFlight` or `Queued` state must return `BlockError::DoubleCompletion`.
- Completed requests remain available for status querying until explicitly reaped by the caller.

### 4. Lost-Completion and Timeout Prevention
- Each in-flight request records a submission timestamp and deadline.
- Block devices execute periodic watchdog sweeps to detect dropped interrupts or stalled controller commands,
  marking unresponsive requests as `BlockError::LostCompletion`.

### 5. Hardware Driver Parity
- Drivers such as `VirtIoBlockDevice` implement standard virtqueue protocols, reading and writing blocks
  with strict buffer length and alignment verification.
