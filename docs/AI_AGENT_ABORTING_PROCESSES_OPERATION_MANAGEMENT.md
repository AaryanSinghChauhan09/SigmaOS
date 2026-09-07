# AI Agent Guidelines for SigmaOS Aborting Processes Operation Management

This document provides guidelines, architectural specifications, and verification protocols for AI agents developing, inspecting, or extending **SigmaOS Process Abortion, Signal-Driven Termination, Core Dump Generation, and OOM Reclamation**.

---

## 1. System Architecture & Process Abortion Subsystem Layout

SigmaOS implements production-oriented process abortion and lifecycle control across `src/process/advanced_process_control.rs` and `src/process/activity_manager.rs`:

| Subsystem Component | Primary Source File | Core Data Structures | Functional Role |
| :--- | :--- | :--- | :--- |
| **Job Control Lifecycle Engine** | `src/process/advanced_process_control.rs` | `JobControlLifecycleEngine`, `CoreDumpMetadata`, `ProcessJobEntry` | Job control, core dump isolation upon fatal signals (`SIGABRT`, `SIGSEGV`), and process group aborts |
| **Cancellation & Termination Manager** | `src/process/advanced_process_control.rs` | `ProcessCancellationAndTerminationManager`, `ProcessCancelState`, `CancellationType` | Deferred vs Asynchronous thread/process cancellation and exit code recording |
| **Process Lifecycle Controller** | `src/process/advanced_process_control.rs` | `SovereignProcessLifecycleController`, `ProcessLifecycleRecord`, `ProcessLifecycleState` | Unified lifecycle manager handling `terminate_process`, `abort_process`, and status reaping |
| **Activity Manager & OOM Killer** | `src/process/activity_manager.rs` | `ActivityManager`, `ProcessActivityRecord`, `PsiMetrics` | Pressure Stall Information (PSI) monitoring and OOM process reclamation |

---

## 2. Process Abortion Mechanics & Code Patterns

AI agents modifying process termination or signal-driven abortion must adhere to these core patterns:

### 1. Process Abortion vs Normal Termination
- **Normal Termination (`terminate_process(pid, exit_code)`):** Cleans up process resources, records the exit code, and moves state to `Terminated` or `Zombie`.
- **Signal Abortion (`abort_process(pid, signal, fault_addr)`):** Triggered by fatal signals (such as `SIGABRT` = 6 or `SIGSEGV` = 11). Generates a `CoreDumpMetadata` snapshot containing instruction pointers (`fault_rip`), stack pointers (`fault_rsp`), and register state before terminating the process.

```rust
use sigma::process::advanced_process_control::JobControlLifecycleEngine;

let mut job_engine = JobControlLifecycleEngine::new();
job_engine.register_process(10, 100); // PID 10, PGID 100

// Abort process PID 10 due to SIGABRT (Signal 6)
let core_dump = job_engine.abort_process(10, 6, 0x7FFF0000).unwrap();
assert_eq!(core_dump.pid, 10);
assert_eq!(core_dump.signal, 6);
```

### 2. Process Cancellation Types (`CancellationType`)
Manages thread/process cancellation requests:
- **`Deferred`:** Cancellation is pended until the target process reaches an explicit cancellation point (e.g. `read`, `write`, `sleep`).
- **`Asynchronous`:** Process execution is aborted immediately upon receiving the cancellation request.

```rust
use sigma::process::advanced_process_control::{ProcessCancellationAndTerminationManager, CancellationType};

let mut cancel_mgr = ProcessCancellationAndTerminationManager::new();
cancel_mgr.set_cancellation_type(10, CancellationType::Deferred);

// Request deferred cancellation
cancel_mgr.request_cancellation(10);
```

### 3. OOM Memory Reclamation (`ActivityManager`)
Under severe OOM pressure, `ActivityManager` inspects process activity states and reclaims idle or low-priority background applications:

```rust
use sigma::process::activity_manager::ActivityManager;

let mut activity_mgr = ActivityManager::new();
// Reclaim background processes to free memory under OOM pressure
let terminated_pids = activity_mgr.reclaim_idle_processes_for_oom();
```

---

## 3. Testing & Verification Protocol for AI Agents

When modifying process abortion, signal handling, or core dump generation, AI agents must execute the following validation steps:

### 1. Standalone Module Test Execution
Run standalone rustc test suites for advanced process control and activity manager:

```bash
rustc --test --edition=2021 src/process/advanced_process_control.rs -o build/test_proc_control && ./build/test_proc_control
rustc --test --edition=2021 src/process/activity_manager.rs -o build/test_act_mgr && ./build/test_act_mgr
```

### 2. Full System Integration & Inspection Suite
Run the master test script to validate all C++ test runners, inspection test binaries, Python test suites, and core process management subsystems:

```bash
./run_sigma_tests.sh
```

---

## 4. Coding Standards & Safety Directives

- **Zombie Reaping:** Terminated or aborted child processes must retain their exit code/signal metadata until reaped by parent via `waitpid` (`WaitStatus`).
- **Core Dump Isolation:** Core dump metadata generation must be isolated to prevent cascade failures in parent process groups.
- **Verification Rule:** Always confirm file creation/edits with `read_file` before completing steps.
