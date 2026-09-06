# AI Agent Time Sharing System Management Guide

## Overview
This wiki guide details Time Sharing System Management protocols for AI coding agents operating on SigmaOS. It covers quantum time slicing, POSIX `SCHED_RR` quanta calculation, EEVDF virtual deadline tracking (`vruntime_us`), Multi-Level Feedback Queue (MLFQ) priority decay, and preemption timer accounting.

## Key Principles
1. **Quantum Preemption**: CPU time is divided into time slices (quanta). When a task exhausts its slice, preemption occurs and the scheduler context-switches to the next ready process.
2. **Priority Quantum Scaling**: Higher-priority processes receive larger quanta to reduce context-switch overhead, while interactive tasks receive shorter quanta for low latency.
3. **MLFQ Exponential Quanta**: Queue level $i$ allocates $2^i\text{ ms}$ quanta to penalize CPU-bound tasks and favor interactive I/O-bound tasks.

## Time Slice Accounting (`src/kernel/roundrobin.rs`)
```rust
if task.time_slice_remaining_ms > 0 {
    task.time_slice_remaining_ms -= 1;
}
if task.time_slice_remaining_ms == 0 {
    task.rr_time_slice_ms = Self::calculate_sched_rr_quantum(task.priority);
    task.time_slice_remaining_ms = task.rr_time_slice_ms;
    need_resched = true;
}
```

## Related Documents
- `docs/AI_AGENT_TIME_SHARING_SYSTEM_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_TIME_SHARING_SYSTEM_MANAGEMENT_GUIDELINES.md`
- `wiki/AI_AGENTS_TIME_MANAGEMENT_GUIDE.md`
