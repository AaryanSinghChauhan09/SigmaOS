# AI Agent Concurrency Mechanism Operation Management Specification for SigmaOS

This document specifies operational standards for AI agents managing concurrency mechanisms, locks, atomics, and thread synchronization in **SigmaOS**.

---

## 1. Concurrency Protocol Rules

AI agents managing concurrent code paths must adhere to the following rules:

1. **Spinlock Rules**:
   - Use `IrqSafeSpinlock` for code paths shared with interrupt handlers.
   - Never sleep, allocate memory, or perform blocking I/O while holding a spinlock.

2. **Atomic Ordering**:
   - Use explicit atomic memory ordering (`Acquire`/`Release` or `SeqCst`).
   - Mitigate ABA pointer hazards in CAS loops using generation tagging.

3. **Mutex & Futex Protocol**:
   - Use `SimpleMutex` for sleeping tasks. Ensure Priority Inheritance Protocol (PIP) is active on RT paths.

4. **Deadlock Avoidance**:
   - Acquire locks in strict global hierarchy order.

---

## 2. Verification Protocol

- Verify concurrency mechanism modifications by executing `./run_sigma_tests.sh` and `tests/stress_and_fuzz_tests.rs`.

---

*Maintained by the SigmaOS Core Concurrency Steering Committee.*
