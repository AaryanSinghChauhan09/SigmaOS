# AI Agent IPC Subsystem Maintenance & Development Guidelines

This document establishes standard operating procedures and architectural guidelines for AI agents maintaining and developing Inter-Process Communication (IPC) subsystems within SigmaOS.

---

## 1. Subsystem Overview & Core Architecture

SigmaOS provides a unified, zero-copy, safe Rust IPC infrastructure that bridges POSIX Unix domain sockets, D-Bus object buses, shared memory ring buffers, and fast message-passing queues. AI agents modifying or maintaining IPC components must adhere to strict `#![no_std]` allocations and memory-ordering constraints.

### Core Modules:
- `src/ipc/dbus.rs` / `src/ipc/`: Sovereign D-Bus implementation (signal broadcasting, method call dispatching, match filtering, and object paths).
- `src/ipc/zero_copy.rs` / `src/performance/zero_copy_ipc.rs`: Lock-free ring buffer channels for bulk data transfer across task/process boundaries.
- `src/ipc/unix_socket.rs`: Unix domain stream and datagram sockets (`AF_UNIX`).
- `src/ipc/message_queue.rs`: Sovereign System V / POSIX message queues with priority-based message dequeuing.

---

## 2. Maintenance & Development Guidelines for AI Agents

### 2.1 Safe Memory Ownership & Zero-Allocation Pipelines
1. **Zero-Copy Invariant:** Always prefer shared memory ring buffers (`ZeroCopyIpcChannel`) for high-throughput payload transfers exceeding 4 KB. Avoid allocations during message enqueuing.
2. **Handle Lifetime:** File descriptors and socket handles passed across processes must be tracked via reference-counted atomic handles (`Arc<AtomicU32>`) or capability tokens.
3. **Buffer Alignment:** Ensure ring-buffer memory slices are cacheline-aligned (64 bytes) to eliminate false sharing between reader and writer cores.

### 2.2 D-Bus Parity & Routing Rules
1. **Match Filter Evaluation:** Match rules must evaluate `interface`, `member`, `path`, and `sender` attributes in $O(1)$ or $O(\log N)$ time using pre-computed path hash maps.
2. **Method Dispatching:** Asynchronous method calls must support timeouts (`TimeoutMs`) and return structured error codes rather than panicking.
3. **Signal Broadcasts:** Signals must be dispatched concurrently to all matching subscriber queues without blocking sender threads.

### 2.3 Diagnostic Protocol & Self-Verification
AI agents must verify changes to IPC subsystems using the following routine:
```bash
# 1. Run unit test runner for D-Bus IPC
cargo test --lib ipc::dbus

# 2. Run full sovereign D-Bus test suite
./run_sigma_tests.sh
```

---

## 3. Safe Rust Code Blueprints

### 3.1 D-Bus Method Call Dispatcher Blueprint
```rust
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DBusMessageHeader {
    pub serial: u64,
    pub path: String,
    pub interface: String,
    pub member: String,
}

pub struct DBusDispatcher {
    next_serial: AtomicU64,
}

impl DBusDispatcher {
    pub const fn new() -> Self {
        Self {
            next_serial: AtomicU64::new(1),
        }
    }

    pub fn dispatch(&self, header: &DBusMessageHeader, payload: &[u8]) -> Result<Vec<u8>, &'static str> {
        if header.path.is_empty() || header.member.is_empty() {
            return Err("Invalid D-Bus message header");
        }
        let _serial = self.next_serial.fetch_add(1, Ordering::SeqCst);
        Ok(payload.to_vec())
    }
}
```

---

*Verified & Enforced for SigmaOS AI Agents.*
