# Study: Scheduler fairness models

## Reference systems
- **Linux CFS**: Completely Fair Scheduler based on virtual runtime (`vruntime`) tracking with red-black trees. Replaced by EEVDF in newer kernels.
- **FreeBSD ULE**: Modern multiprocessor scheduler utilizing interactive heuristics, dual run queues (current and next), and CPU affinity domains.
- **OpenBSD scheduler**: Multilevel feedback queue (MLFQ) prioritizing interactive responsiveness and simplicity with probabilistic fair sharing.
- **Redox**: Microkernel scheduler utilizing priority round-robin queues and lockless task channels.

## Mechanism
- **Problem**: Balancing low-latency interactivity for GUI/shell threads while preventing starvation of compute-heavy batch tasks.
- **Data structures**: Priority bitmaps, red-black trees for vruntime ordering, dual interactive/batch run queues.
- **Scheduling decision**: `O(1)` or `O(log N)` selection of lowest vruntime or highest priority runnable task.
- **Priority handling**: Nice values map exponentially to weight multipliers in CFS; static priority bands in BSD ULE.
- **Failure modes**: Starvation of low-priority tasks under persistent high load; priority inversion when low-priority tasks hold locks needed by real-time tasks.

## SigmaOS current state
- **Code**: `src/scheduler/core.rs`, `src/scheduler/run_queue.rs`.
- **Tests**: `tests/scheduler/` unit tests and integration tests.
- **Runtime evidence**: QEMU boot logs demonstrate thread switching across 11 boot milestones.
- **Gaps**: Need formal starvation regression test under heavy simulated loads.

## Decision
- **Adopt**: Virtual runtime fairness metrics and priority inversion inheritance.
- **Adapt**: Hybrid priority bands combined with CFS vruntime to ensure responsive shell/Zenith compositor interaction.
- **Reject**: Overly complex multi-socket NUMA load-balancing until multi-socket hardware evidence is established.

## Follow-up PRs
- [ ] feat(scheduler): add fairness metric
- [ ] test(scheduler): add starvation regression test
