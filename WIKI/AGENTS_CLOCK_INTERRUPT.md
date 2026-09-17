# SigmaOS AI Agent Clock Interrupt Management Directive (`AGENTS_CLOCK_INTERRUPT.md`)

This document defines technical directives, timer interrupt handling protocols, and timekeeping guidelines for AI agents managing clock interrupts in SigmaOS.

---

## 1. Core Principles for Clock Interrupt Management

Clock interrupts (PIT, Local APIC timer, HPET, and ARM Generic Timers) drive kernel timekeeping, process preemption, and high-resolution timer queues in SigmaOS. AI agents modifying timer routines must observe the following rules:

1. **Reentrancy & Interrupt Safety:**
   - Clock interrupt service routines (ISRs) must execute with minimal latency. Avoid blocking lock acquisitions or memory allocations inside timer interrupt handlers.
   - Timer state modifications (`start_time`, `is_running`) must utilize lock-free atomic variables (`AtomicBool`, `AtomicU64`) with `Ordering::SeqCst` memory orderings.

2. **Timer Descriptor Management (`TimerDescriptor`, `TimerCapability`):**
   - Timers managed via `TimerDescriptor` must validate `TimerCapability` and interval bounds before starting or resetting.
   - Periodic and one-shot timer callbacks must execute safely without leaking resources or causing recursive ISR invocation.

3. **Timekeeping Accuracy & Drift Compensation:**
   - Monotonic time calculations must derive from hardware timestamp counters (TSC/Generic Timers) synchronized across CPU cores.
   - Prevent timer drift during process context switches by calculating elapsed interval ticks atomically.

4. **Zero-Dependency `#![no_std]` Compatibility:**
   - Timer subsystems in core kernel layers must maintain zero-dependency `#![no_std]` compliance.

---

## 2. Pre-Commit Clock Interrupt Verification Checklist

Before submitting code modifications, AI agents must verify:
- [ ] Clock ISRs operate without memory allocations or blocking locks.
- [ ] Timer descriptors (`TimerDescriptor`) validate interval bounds before activation.
- [ ] Atomic timer state transitions handle concurrent start/stop requests safely.
- [ ] `./run_sigma_tests.sh` executes with 100% test pass rate.
