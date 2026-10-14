# SigmaOS Preemption Policy Specification

## Preemption Triggers
1. **Time-Slice Expiration**: Periodic APIC timer tick decrements running process `remaining_quantum_ticks`. When 0, preemption flag `need_resched` is set.
2. **Priority Preemption**: Enqueue of a task with higher priority than currently running task sets `need_resched`.
3. **Voluntary Yield**: Process calls `sched_yield()`, yielding CPU immediately.

## Safety Directives
- Preemption is disabled when inside spinlocks, interrupt handlers, or non-preemptible critical sections.
- Preemption during context switch is forbidden.
