# AI Agent Message Passing Management in SigmaOS

## Overview
SigmaOS incorporates a zero-trust, high-throughput Message Passing Subsystem supervised by autonomous AI Agents (**Bolt** ⚡, **Sentinel** 🛡️, **Palette** 🎨). This document defines operational directives, queue priority rules, zero-copy buffer sharing protocols, and security validation interfaces for AI agents managing message queues and IPC message passing across SigmaOS.

AI agents interact directly with `src/process/advanced_process_control.rs` (`PosixMessageQueue`, `PosixMessage`, `AdvancedIpcHub`) and `src/process/sovereign_process_engine.rs` (`ZeroCopyIpcChannel`).

---

## 1. Message Passing Architecture & Subsystems

### 1.1 POSIX Message Queue Subsystem (`PosixMessageQueue`)
Implemented in `src/process/advanced_process_control.rs`. Provides kernel-managed POSIX priority message queues (`mq_open`, `mq_send`, `mq_receive`):
```rust
pub struct PosixMessage {
    pub priority: u32,
    pub payload: Vec<u8>,
}

pub struct PosixMessageQueue {
    pub name: String,
    pub max_messages: usize,
    pub max_message_size: usize,
    pub messages: Vec<PosixMessage>,
}
```
* **Priority Enqueueing (`mq_send`)**: Messages are inserted into the queue sorted by integer `priority` (higher numerical priority delivered first).
* **Queue Bounds Enforcement**: Rejects incoming messages if `messages.len() >= max_messages` or `payload.len() > max_message_size`.

### 1.2 Zero-Copy IPC Message Channels (`ZeroCopyIpcChannel`)
Implemented in `src/process/sovereign_process_engine.rs`. Facilitates direct ring-buffer message transfers between producer and consumer processes without intermediate kernel buffer allocations.

### 1.3 Message Routing Hub (`AdvancedIpcHub`)
Implemented in `src/process/advanced_process_control.rs`. Manages named POSIX message queues (`message_queues: BTreeMap<String, PosixMessageQueue>`), handling subscriber registration, channel lifecycles, and teardown error signals (`ProcessControlError::IpcChannelClosed`).

---

## 2. AI Agent Operational Directives & Protocols

### 2.1 Message Priority & Ordering Protocols
1. **Priority Queue Invariants**:
   When **Bolt** ⚡ dispatches system control messages, high-priority real-time messages (`priority >= 100`) bypass standard background messages.
2. **Buffer Capacity Monitoring**:
   AI agents monitor queue message counts (`messages.len() / max_messages`). When a message queue reaches 90% capacity, agents emit flow-control backpressure signals.

### 2.2 Message Payload Security Validation
* **Sentinel 🛡️ Inspection**:
  Before processing foreign message payloads, **Sentinel** 🛡️ validates payload size bounds against `max_message_size` and sanitizes untrusted input bytes.

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Query active POSIX message queues and message counts
sigma-msg status --queue /mq_system_events

# Send a high-priority message to a POSIX message queue
sigma-msg send --queue /mq_system_events --priority 100 --payload "REBOOT_SCHEDULED"

# Benchmark POSIX message queue throughput
sigma-msg bench --max-msgs 1024 --msg-size 256
```
