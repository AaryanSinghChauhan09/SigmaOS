# AI Agent Guidelines for SigmaOS Coroutines & Fibril Operations Management

This document provides guidelines, architectural specifications, and verification protocols for AI agents developing, inspecting, or extending **SigmaOS Coroutines, Fibril-based Userland Cooperative Multitasking, and Async IPC Operations**.

---

## 1. System Architecture & Coroutine/Fibril Overview

SigmaOS implements microkernel-inspired coroutines and lightweight pseudo-threads (fibrils) in `src/ipc/helenos_async.rs` (inspired by HelenOS async IPC architecture):

| Subsystem Component | Primary Source File | Primary Structures | Functional Role |
| :--- | :--- | :--- | :--- |
| **Fibril Engine** | `src/ipc/helenos_async.rs` | `Fibril`, `FibrilManager`, `FibrilType`, `FibrilState` | Lightweight userland coroutine/pseudo-thread context management and cooperative scheduling |
| **Async IPC Manager** | `src/ipc/helenos_async.rs` | `HelenIpcManager`, `Answerbox`, `Phone`, `HelenMessage` | Async message routing queues, call-id tracking, and non-blocking message passing |
| **Top-Half IRQ Handlers** | `src/ipc/helenos_async.rs` | `TopHalfHandler`, `SimpleTopHalfHandler`, `IrqRegistration` | Fast top-half hardware interrupt handlers routing async notifications to userland answerboxes |
| **Combined System System** | `src/ipc/helenos_async.rs` | `HelenAsyncSystem` | Unified coordinator binding task answerboxes, phones, and fibril managers |

---

## 2. Core Coroutine & Fibril Mechanics

AI agents modifying coroutine execution or async message passing must adhere to these patterns:

### 1. Fibrils (Cooperative Userland Pseudo-Threads)
Fibrils run cooperatively without preemptive kernel context switches:
- **Fibril Types (`FibrilType`):** `Manager` (main dispatch loop) and `Worker` (task processing coroutine).
- **Fibril States (`FibrilState`):** `Uninitialized`, `Ready`, `Running`, `Blocked`, `Terminated`.
- **Cooperative Switching:** `FibrilManager::create_worker_fibril` creates coroutines, while `schedule_next` yields control to the next ready worker or manager fibril.

```rust
use sigma::ipc::helenos_async::{FibrilManager, FibrilType, FibrilState};

let mut manager = FibrilManager::new();
// Create worker fibril / coroutine
let fibril_id = manager.create_worker_fibril("data_processor".to_string(), 4096);

// Schedule next ready coroutine
if let Some(next_id) = manager.schedule_next() {
    assert_ne!(next_id, 0);
}
```

### 2. Async Answerbox & Phone Messaging
Communication between coroutines and tasks uses HelenOS-style phones and answerboxes:
- **Phone:** One-way client handle that targets an `Answerbox`.
- **Answerbox:** Task inbox containing message queues for incoming async calls (`HelenMessage` with 4 arguments).

```rust
use sigma::ipc::helenos_async::{HelenIpcManager, HelenMessage};

let mut ipc = HelenIpcManager::new();
let answerbox_id = ipc.create_answerbox(10); // Task ID 10
let phone_id = ipc.create_phone(20, answerbox_id); // Task ID 20 -> Answerbox

// Send async message from task 20 to task 10 without blocking caller
let msg = HelenMessage::new(1, 100, 200, 300, 400);
let call_id = ipc.send_message(phone_id, msg).unwrap();
```

---

## 3. Testing & Verification Protocol for AI Agents

When modifying coroutine scheduling, fibril managers, or async IPC queues, AI agents must execute the following validation steps:

### 1. Standalone Module Test Execution
Run standalone rustc test suite for async IPC and fibrils:

```bash
rustc --test --edition=2021 src/ipc/helenos_async.rs -o build/test_helenos && ./build/test_helenos
```

### 2. Full System Integration & Inspection Suite
Run the master test script to validate all C++ test runners, inspection test binaries, Python test suites, and core coroutine subsystems:

```bash
./run_sigma_tests.sh
```

---

## 4. Coding Standards & Safety Directives

- **Cooperative Yield Invariants:** Fibril coroutines must explicitly yield or complete to avoid blocking the single-threaded manager dispatch loop.
- **Queue Bounds:** Ensure answerbox message queues enforce capacity limits to prevent memory bloat under high message volume.
- **Verification Rule:** Always confirm file creation/edits with `read_file` before completing steps.
