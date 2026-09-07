# AI Agent Coarse Parallelism & Threading Architecture (`docs/AGENTS_THREADING_PARALLELISM.md`)

This guide details the technical architecture, thread scheduling interfaces, and AI agent monitoring protocols for coarse-grained parallelism and thread management in SigmaOS.

---

## 1. Subsystem Architecture

SigmaOS implements scalable coarse parallelism and thread management across kernel and architectural subsystems:

### A. Task & Thread Descriptor State
- Located in `src/arch/comprehensive.rs` and `src/arch/hal.rs`.
- Tracks thread context (`current_thread_id`), RCU synchronization epochs (`rcu_epoch`), and HAL execution levels (`PassiveLevel`, `DispatchLevel`).

### B. Adaptive Quantum & Scheduler Integration
- Located in `src/ai/next_gen.rs` and `src/kernel/scheduler.rs`.
- Dynamically calculates `adaptive_thread_quantum_multiplier` to adjust time slice allocations based on thread priority and execution history.

### C. Multi-Core & NUMA Parallel Dispatch
- Balances thread workloads across CPU cores while respecting NUMA node memory locality and thread stack guard zones (`has_guard_page`).

---

## 2. AI Agent Operational Directives

1. **Lockless RCU Audit:** Ensure thread state modifications within RCU read-side critical sections do not invoke blocking calls.
2. **Stack Protection Verification:** Confirm all thread allocation paths enforce `has_guard_page = true`.
3. **Automated Testing:** Execute `./run_sigma_tests.sh` to confirm threading and scheduler unit tests pass.
