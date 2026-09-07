# AI Agent Race Condition Management Specification for SigmaOS

This document specifies the operational guidelines for AI agents managing race conditions, concurrency synchronization, and atomic state transitions in **SigmaOS**.

---

## 1. Concurrency Principles & Execution Boundaries

AI agents operating within the SigmaOS kernel or userland must adhere to zero-race concurrency discipline:

1. **Atomic State Transitions**:
   - Shared atomic flags must use explicit memory ordering (`Acquire`/`Release` or `SeqCst`).
   - Use Compare-And-Swap (CAS) loops (`compare_exchange` / `compare_exchange_weak`) for lock-free state updates.

2. **Spinlock & Interrupt Safety**:
   - Critical sections in interrupt contexts must use `IrqSafeSpinlock` (`src/kernel/spinlock.rs`).
   - Spinlocks must never invoke code that allocates memory, blocks, or yields thread execution.

3. **TOCTOU Race Avoidance**:
   - File system modifications must use directory file descriptors (`openat`) and atomic creation flags (`O_EXCL`, `O_NOFOLLOW`).
   - Use atomic file replacement (`renameat2`) for state updates.

4. **Lock Hierarchy & Deadlock Prevention**:
   - Acquire locks in strict lexicographical order.
   - Non-blocking lock requests (`try_lock`) must include backoff strategies.

---

## 2. Verification Protocol

- Modifying concurrent data structures requires running `tests/stress_and_fuzz_tests.rs` and `./run_sigma_tests.sh`.

---

*Maintained by the SigmaOS Core Concurrency Steering Committee.*
