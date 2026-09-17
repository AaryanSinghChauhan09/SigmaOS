# SigmaOS AI Agent Coarse Parallelism & Threading Directive (`AGENTS_THREADING_PARALLELISM.md`)

This document defines technical directives, thread synchronization protocols, and coarse-grained parallelism rules for AI agents managing multi-threaded operations in SigmaOS.

---

## 1. Core Principles for Coarse Parallelism & Threading

Coarse-grained task parallelism and thread execution in SigmaOS utilize Linux- and BSD-inspired scheduling models, Read-Copy-Update (RCU) synchronization, and adaptive quantum scaling. AI agents modifying threading or parallel dispatch routines must observe the following rules:

1. **RCU Synchronization Generations (`rcu_epoch`):**
   - Thread state descriptors track RCU synchronization epochs (`rcu_epoch`) to allow lockless concurrent reads across multi-core systems.
   - Updates to shared thread-local data or process descriptors must enter RCU read-side critical sections safely and defer reclamation until quiescent states are observed across all CPU cores.

2. **Adaptive Thread Quantum Scaling (`adaptive_thread_quantum_multiplier`):**
   - Dynamic quantum multipliers (`adaptive_thread_quantum_multiplier`) scale thread execution time slices based on workload characteristics (e.g., interactive UI vs compute-bound background threads).
   - High-priority interactive threads require shortened quanta to maintain responsiveness, whereas batch compute threads use expanded time slices to minimize context-switch overhead.

3. **Stack Guard Safety & Thread Isolation (`has_guard_page`):**
   - Every kernel and user thread stack must allocate guard pages (`has_guard_page = true`) to prevent thread stack-clash exploits and overflow into adjacent thread contexts.

4. **NUMA Affinity & Multi-Core Distribution:**
   - Coarse-grained parallel tasks must align execution with NUMA node boundaries to minimize cross-socket interconnect latency.
   - Use atomic thread-safe primitives (`AtomicU64`, `AtomicUsize`) for inter-thread state signaling.

---

## 2. Pre-Commit Threading Verification Checklist

Before submitting code modifications, AI agents must verify:
- [ ] Thread creation routines allocate stack guard zones (`has_guard_page = true`).
- [ ] RCU read-side critical sections update `rcu_epoch` tracking correctly without deadlocks.
- [ ] Adaptive quantum adjustments enforce upper and lower bounds on thread time slices.
- [ ] `./run_sigma_tests.sh` executes with 100% test pass rate.
