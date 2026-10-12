# SigmaOS Thread Manager Architecture & Engineering Cadence Contract

## 1. Subsystem Overview

The Thread Manager is a core kernel subsystem responsible for thread lifecycle management, state transitions, context tracking, wait-channel wait/wakeup synchronization, and CPU scheduler interfacing.

The primary design objective is to provide a single, canonical `Thread` abstraction across kernel, user, process, and detached threads while strictly preventing parallel or divergent implementations.

---

## 2. Thread Lifecycle States & Transition Matrix

Every thread in SigmaOS exists in exactly one of seven canonical `ThreadState` variants:

1. **`New`**: Created and allocated; context initialized; not yet placed in a scheduler ready queue.
2. **`Ready`**: Enqueued in a scheduler run queue, waiting to be scheduled on a CPU core.
3. **`Running`**: Currently executing instructions on a CPU core.
4. **`Blocked`**: Suspended waiting on an explicit event, I/O, timer, signal, or wait-channel resource.
5. **`Suspended`**: Paused by administrative or process control (e.g. SIGSTOP / `suspend()`); ineligible for scheduling until resumed.
6. **`Zombie`**: Terminated execution; exit status and resource summary retained until reaped by owner/parent.
7. **`Terminated`**: Reaped and detached; kernel stack and metadata ready for final deallocation.

### Transition Rules

- `New -> Ready`: Activated by creation or `start()`.
- `Ready -> Running`: Selected by the CPU scheduler.
- `Running -> Ready`: Yielded voluntarily or preempted by timer/interrupt.
- `Running -> Blocked`: Waiting on wait channel, I/O, or sleeping.
- `Blocked -> Ready`: Woken up by wait channel notification, signal delivery, or timer expiration.
- `Running -> Suspended` / `Ready -> Suspended` / `Blocked -> Suspended`: Suspended by signal/control.
- `Suspended -> Ready`: Resumed by explicit `resume()` / SIGCONT.
- `Running -> Zombie`: Thread exited or killed; awaiting join/reap.
- `Zombie -> Terminated`: Reaped via `join()`, parent wait, or detach reaper.

Invalid state transitions (e.g. `Zombie -> Running` or `Terminated -> Ready`) must fail deterministically with `ThreadError::InvalidStateTransition`.

---

## 3. Ownership & Capabilities

Thread ownership is categorized into four distinct models:
- **Kernel Thread**: Executes solely in kernel space; no user space page table or user stack; parent PID is 0.
- **User Thread**: Belongs to a user space process; possesses a user stack, TLS area, and system call interface.
- **Process Thread**: Main thread or worker thread sharing the process address space, file descriptor table, and signal disposition.
- **Detached Thread**: Disowned thread whose resources are automatically reclaimed upon termination without requiring `join()`.

---

## 4. Guarantees & Safety Invariants

1. **Wait-Channel Wakeup Safety**: Wait channel lock scope wraps state checking and wakeup registration simultaneously, preventing lost wakeup races when timeouts, signals, and wakeups occur concurrently.
2. **Stack Guards & Bounds Check**: Every kernel and user stack must feature guard pages and bounds validation (`stack_base .. stack_base + stack_size`).
3. **Lock-Order Enforcement**: When lock-order tracking is enabled, thread-held locks are tracked to detect potential cyclic deadlocks at acquisition time.
4. **Preemption Safety**: Timer-driven and interrupt preemption must preserve saved CPU context (`CpuContext`) without corrupting caller-saved registers or interrupt masks.

---

## 5. Engineering Cadence Program

| Period | Thread Manager & Scheduler Work | OS-Wide Maintenance | Required PR Output |
|---|---|---|---|
| **Daily** | Triage scheduler bugs, flaky tests, lock-order warnings, and regression reports | Triage CI failures, build breakage, security advisories, and documentation drift | Bug reports or small fix PRs |
| **Weekly** | Implement one scheduler behavior: run queue, preemption, priority, affinity, or wait queue | Dependency/toolchain updates, lint cleanup, test-suite repair | Scoped PR per change |
| **Biweekly** | Integration test: threads with signals, FDs, IPC, timers, and process exit | Component status audit: update `FEATURE_STATUS.toml` and docs with evidence | Status + test-evidence PR |
| **Monthly** | Scheduler benchmark, contention test, starvation audit, deadlock/fuzz review | Release-candidate build, ISO smoke test, installer/update recovery test | Release-readiness PR |
| **Quarterly** | Scheduler architecture review against Linux/BSD patterns; remove dead code and duplicated plans | Roadmap consolidation, security review, API stability review, contributor survey | Architecture RFC + cleanup PRs |

---

## 6. Verification Commands

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```
