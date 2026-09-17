# AI Agent Clock Interrupt Management Architecture (`docs/AGENTS_CLOCK_INTERRUPT.md`)

This guide details the technical architecture, timer descriptor interfaces, and AI agent monitoring protocols for clock interrupt management in SigmaOS.

---

## 1. Subsystem Architecture

SigmaOS provides low-latency timer interrupt handling and high-resolution timekeeping:

### A. Timer Abstractions & Descriptors
- Located in `src/timer/timer.rs`.
- Defines `TimerDescriptor`, `TimerCapability`, and `TimerInfo` for managing periodic and one-shot hardware timers.
- Uses atomic integers (`AtomicU64`, `AtomicBool`) to track timer start times, intervals, and active states without lock contention inside ISRs.

### B. Hardware Timer Ticks & Preemption
- APIC and HPET hardware timers generate periodic interrupts that trigger the kernel scheduler (`src/kernel/scheduler.rs`) to calculate process virtual deadlines and enforce EEVDF/ULE preemption.

### C. Timer Callback Execution
- Dispatches registered callback functions (`fn(TimerID)`) upon timer expiration, automatically resetting periodic timers to the next interval tick.

---

## 2. AI Agent Operational Directives

1. **ISR Non-Blocking Audit:** Ensure clock interrupt handlers avoid heap allocations or lock contention.
2. **Atomic Synchronization:** Verify atomic load/store operations on `is_running` and `start_time` fields prevent race conditions between interrupt context and process threads.
3. **Automated Verification:** Execute `./run_sigma_tests.sh` to confirm timer unit tests pass.
