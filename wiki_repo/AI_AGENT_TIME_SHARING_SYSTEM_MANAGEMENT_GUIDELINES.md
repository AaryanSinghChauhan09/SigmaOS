# AI Agent Time Sharing System Management Guidelines

## Purpose
These guidelines define operational rules, implementation patterns, and safety guardrails for AI coding agents configuring or optimizing Time Sharing System Management in SigmaOS.

---

## Directives for AI Agents

1. **Preemption Timer Compliance**:
   - Ensure `time_slice_remaining_ms` is updated accurately during timer interrupts.
   - When a process exhausts its quantum (`time_slice_remaining_ms == 0`), re-arm its quantum via `calculate_sched_rr_quantum()` and trigger a voluntary yield.

2. **Latency-Sensitive Task Handling**:
   - Tasks with low `latency_nice` or interactive BORE scores should be assigned smaller, high-frequency time slices to minimize response jitter.

3. **Code Pattern: Quantum Accounting and Preemption**:
```rust
// Preemption tick check
if task.time_slice_remaining_ms > 0 {
    task.time_slice_remaining_ms -= 1;
}
if task.time_slice_remaining_ms == 0 {
    // Reset quantum for next cycle
    task.rr_time_slice_ms = Self::calculate_sched_rr_quantum(task.priority);
    task.time_slice_remaining_ms = task.rr_time_slice_ms;
    // Signal reschedule request
    need_resched = true;
}
```

4. **Testing and Verification**:
   - Execute `./run_sigma_tests.sh` to confirm time slice and round-robin scheduler unit tests.

---

## Related Files
- `src/kernel/roundrobin.rs`
- `src/process/scheduler.rs`
- `docs/AI_AGENT_TIME_SHARING_SYSTEM_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_TIME_SHARING_SYSTEM_MANAGEMENT.md`
