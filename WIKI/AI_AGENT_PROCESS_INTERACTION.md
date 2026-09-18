# AI Agent Process Interaction Management in SigmaOS

## Overview
SigmaOS incorporates a high-performance Inter-Process Communication (IPC) and Process Interaction Subsystem governed by autonomous AI Agents (**Bolt** ⚡, **Sentinel** 🛡️, **Palette** 🎨). This document defines operational directives, zero-copy buffer sharing protocols, IPC hub routing rules, and event notification mechanisms for AI agents supervising interactions between userland processes, microVMs, and system daemons.

AI agents interact directly with `src/process/sovereign_process_engine.rs` (`ZeroCopyIpcChannel`) and `src/process/advanced_process_control.rs` (`AdvancedIpcHub`, `EventFd`).

---

## 1. Process Interaction Architecture & Subsystems

### 1.1 Zero-Copy IPC Channels (`ZeroCopyIpcChannel`)
Implemented in `src/process/sovereign_process_engine.rs`. Provides ring-buffer backed inter-process payload sharing without kernel-to-userland memory copies:
```rust
pub struct ZeroCopyIpcChannel {
    pub channel_id: usize,
    pub sender_pid: usize,
    pub receiver_pid: usize,
    pub ring_buffer: Vec<u8>,
    pub capacity_bytes: usize,
}
```
* **Payload Enqueueing (`send_payload`)**: Appends bytes directly to the shared ring buffer, returning `Err("IPC channel capacity exceeded")` if `ring_buffer.len() + payload.len() > capacity_bytes`.
* **Payload Dequeueing (`receive_payload`)**: Reads pending bytes and flushes the ring buffer in a single atomic pass.

### 1.2 Advanced IPC Hub (`AdvancedIpcHub`)
Implemented in `src/process/advanced_process_control.rs`. Manages centralized message queues, IPC channel lifecycles, subscriber routing tables, and channel closure errors (`ProcessControlError::IpcChannelClosed`).

### 1.3 Event Notification Descriptors (`EventFd`)
Implemented in `src/process/advanced_process_control.rs`. Provides lightweight, 64-bit event counter notification handles for thread signaling and asynchronous process wait-wake loops.

---

## 2. AI Agent Operational Rules & Protocols

### 2.1 IPC Capacity & Backpressure Rules
1. **Backpressure Flow Control**:
   **Bolt** ⚡ monitors channel fill levels (`ring_buffer.len() / capacity_bytes`). When channel fill exceeds 85%, agents throttle producer thread scheduling or expand channel capacity.
2. **Channel Teardown Safety**:
   When a process terminates, `AdvancedIpcHub` marks associated channels as closed. Agents verify that pending receiver tasks receive `ProcessControlError::IpcChannelClosed` rather than blocking indefinitely.

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Query active zero-copy IPC channels and buffer usage
sigma-ipc channel-status --channel-id 42

# Inspect IPC message routing table in AdvancedIpcHub
sigma-ipc route-table

# Benchmark IPC throughput and latency between processes
sigma-ipc bench-throughput --sender-pid 101 --receiver-pid 102
```
