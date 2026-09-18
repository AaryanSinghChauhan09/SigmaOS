# AI Agent Guidelines: State Management in SigmaOS

## Overview
This document defines operational guidelines and architectural directives for AI agents working on **State Management** in SigmaOS. It specifies NixOS-inspired declarative system state graphs, generation snapshot rollbacks, Model-View-Intent (MVI) reactive state stores, kernel process lifecycle state transitions, and transactional state persistence across `#![no_std]` runtime environments in SigmaOS.

---

## 1. State Management Subsystems & Modules

AI agents interacting with system configuration, kernel process states, or reactive state stores in SigmaOS must interface with the following core subsystems:

| Subsystem / Module | Location | Description |
| :--- | :--- | :--- |
| **Declarative State Graph** | `src/system/state.rs` | Declarative system state graph (`DeclarativeStateGraph`) managing `StateNode` values, dependency resolution, generation snapshots, and atomic rollback (`rollback()`). |
| **MVI Reactive State Store** | `src/klib/store.rs` | Model-View-Intent reactive state store (`StateStore<S, A>`) managing state reducers, action dispatches, and subscriber notifications. |
| **Process Lifecycle State** | `src/kernel/process.rs`, `src/kernel/sched/task.rs` | Kernel process lifecycle states (`ProcessState`) managing transitions (`New`, `Ready`, `Running`, `BlockedWaiting`, `BlockedSuspended`, `Zombie`, `Terminated`). |
| **Package Transaction State** | `src/sigpkg/transaction.rs` | Atomic package transaction state manager managing CAS store links and generation profile swaps. |
| **Zenith Desktop State** | `src/desktop/state.rs` | Compositor layout and theme state engine managing window tile trees and active workspace state. |

---

## 2. Architectural Rules & State Invariants

AI agents must enforce the following 4 core invariants when implementing or auditing state management mechanisms:

```
+-------------------------------------------------------------------------+
|                  SIGMAOS DUAL STATE ARCHITECTURE                        |
+-------------------------------------------------------------------------+
                                     |
         +---------------------------+---------------------------+
         |                                                       |
         v                                                       v
  [Declarative System State Graph]               [MVI Reactive State Store]
  • NixOS-Inspired State Graph                   • Model-View-Intent Pattern
  • Immutable Generation Snapshots                • Unidirectional Action Dispatch
  • Atomic Rollback (rollback())                 • Reducer State Transformations
  • Dependency Graph Validation                  • Subscriber Event Notifications
```

### 1. Immutable Generation Snapshots & Atomic Rollbacks
- **Invariant:** System configuration state mutations in `src/system/state.rs` MUST create an immutable generation snapshot before applying updates.
- **Rule:** If a state update fails or violates dependency assertions, invoking `rollback()` MUST restore the previous historical state graph snapshot in $O(1)$ time with zero configuration drift.

### 2. Dependency Graph Validation
- **Invariant:** Before committing declarative state graph changes, `validate()` MUST be called to verify that all dependency node IDs exist (`StateError::DependencyNotFound`).
- **Rule:** Circular dependencies between state nodes are strictly prohibited and must trigger a validation fault.

### 3. Process Lifecycle State Transition Integrity
- **Invariant:** Kernel process state transitions (`src/kernel/process.rs`, `src/kernel/sched/task.rs`) MUST follow valid lifecycle paths:
  - `New` $\to$ `Ready` $\to$ `Running` $\to$ `BlockedWaiting` / `BlockedSuspended` $\to$ `Zombie` $\to$ `Terminated`.
- **Rule:** Directly jumping from `New` or `Blocked` to `Terminated` without proper resource cleanup is prohibited.

### 4. Zero Ring 0 Panic Rule
- State transitions and MVI dispatches must handle missing or corrupted state keys via explicit `Result<T, StateError>` types rather than unhandled kernel panics.

---

## 3. Verification & Testing Protocols

Every state management modification must be validated via unit tests and integrated test execution:

```bash
# Run standalone unit test for system state graph
rustc --test --edition 2021 src/system/state.rs -o build/test_system_state && ./build/test_system_state

# Run full test suite
./run_sigma_tests.sh
```

---

## 4. AI Agent Self-Assessment Checklist

Before finalizing changes touching system state, process lifecycle, or reactive stores:

- [ ] Does `DeclarativeStateGraph::validate()` pass without missing dependency node errors?
- [ ] Do state updates create immutable generation snapshots supporting $O(1)$ rollback?
- [ ] Are kernel process state transitions compliant with the formal lifecycle state machine?
- [ ] Have all unit tests passed with 0 failures in `./run_sigma_tests.sh`?
