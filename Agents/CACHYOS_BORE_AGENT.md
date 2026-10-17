# CachyOS BORE Scheduler Enhancer Component Agent

## Role
The **CachyOS BORE Scheduler Agent** (`SigmaBoreAgent`) optimizes multi-tasking desktop responsiveness and interactive latency inspired by CachyOS's Burst-Oriented Response Enhancer (BORE) scheduler.

## Linux Inspiration
- **CachyOS**: BORE scheduler tracking CPU burst times vs sleep cycles, assigning dynamic burst scores (0..39), and shielding interactive desktop tasks (Wayland compositors, audio mixers, gaming engines) from batch compute starvation.

## Current SigmaOS Status
BORE engine implemented in `src/scheduler/bore_engine.rs` with runtime quantum tracking, burst score updates, and effective vruntime penalty adjustments.

## Key Files
- `src/scheduler/bore_engine.rs`
- `src/scheduler/mod.rs`
- `src/kernel/scheduler.rs`

## Public API & Invariant Contract
```rust
pub struct BoreTask {
    pub pid: u64,
    pub name: &'static str,
    pub task_type: TaskType,
    pub burst_time_ns: u64,
    pub sleep_time_ns: u64,
    pub vruntime: u64,
    pub burst_score: u32,
}

pub struct BoreSchedulerEngine {
    pub fn add_task(&mut self, task: BoreTask);
    pub fn pick_next(&self) -> Option<u64>;
    pub fn step_task(&mut self, pid: u64, run_quantum_ns: u64);
}
```

## Testing Strategy
```bash
rustc --test src/scheduler/bore_engine.rs --edition 2021 -o /tmp/test_bore && /tmp/test_bore --test-threads=1
```
- Verify interactive audio/GUI tasks maintain low burst scores and preemption priority over heavy compute loops.
- Validate that quiescent sleep intervals decay accumulated burst penalty.

## Future Enhancements
- Integration with Ananicy-Cpp rule heuristics for dynamic auto-cgroup classification.
- eBPF sched-ext pluggable BORE scheduler module.
