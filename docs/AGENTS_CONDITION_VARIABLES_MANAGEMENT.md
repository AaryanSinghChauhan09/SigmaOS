# AI Agent Condition Variables Management Specification for SigmaOS

This document specifies the operational guidelines for AI agents managing condition variables, wait queues, and thread notification primitives in **SigmaOS**.

---

## 1. Condition Variable Rules & Synchronization Protocol

AI agents utilizing condition variables (`Condvar` / `SimpleCondVar`) must follow strict synchronization rules:

1. **Predicate Loop Enforcement**:
   - Always wrap `condvar.wait(&mut lock)` inside a `while !predicate_holds() { ... }` loop to prevent spurious wakeups.

2. **Signal vs. Broadcast**:
   - Use `notify_one()` for single-consumer job queue dispatch.
   - Use `notify_all()` for global event notifications, barriers, and shutdown states.

3. **Mutex Association**:
   - Every condition variable must be paired with a `SimpleMutex` guarding the predicate state.
   - Predicate updates must be executed while holding the mutex lock.

4. **Lost Wakeup Prevention**:
   - Ensure predicate state is updated prior to issuing `notify_one()` or `notify_all()`.

---

## 2. Verification Protocol

- Verify condition variable changes by running `tests/stress_and_fuzz_tests.rs` and `./run_sigma_tests.sh`.

---

*Maintained by the SigmaOS Core Concurrency Steering Committee.*
