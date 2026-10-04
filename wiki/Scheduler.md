# CPU Scheduler

**Capability state: Prototype.** The scheduler model is not verified on a booted kernel. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

This page is the canonical scheduler component reference. It tracks the current code, comparison sources, validation, and future work together. A policy implementation in a module does not establish that the booted kernel uses it.

## Current implementation

The EEVDF model is in [`src/scheduler/eevdf.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/scheduler/eevdf.rs) and is exported by [`src/scheduler/mod.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/scheduler/mod.rs). It contains a task/run-queue model, nice-to-weight table, virtual-runtime accounting, deadline selection, and a test-only service supervisor retained for compatibility.

The run queue is a `Vec` sorted by virtual deadline, so enqueue is O(n log n) and selection is O(n) in the current code. The model must not be described as per-CPU, SMP-integrated, O(log n), or production-ready without corresponding implementation and runtime evidence. Confirm the kernel dispatch path before asserting that EEVDF is the active policy.

Eligibility is `Ready && (vruntime - lag) <= min_vruntime`, evaluated with signed arithmetic. Positive lag means service is owed; negative lag means service debt. Regression tests cover negative lag and non-ready tasks.

## Design references

| Project | Design to study | SigmaOS application |
|---|---|---|
| [Linux EEVDF scheduler](https://docs.kernel.org/scheduler/sched-eevdf.html) | Eligibility and virtual deadlines, fair service, preemption | Validate lag accounting and deadline ordering against a small reference model. |
| [FreeBSD ULE](https://man.freebsd.org/cgi/man.cgi?query=sched_ule) | CPU topology and affinity-aware scheduling | Defer migration policy until CPU discovery and per-CPU queue ownership are explicit. |
| [xv6 RISC-V](https://github.com/mit-pdos/xv6-riscv) | Small, auditable process state transitions and locking | Keep transition rules testable and document lock ownership before adding concurrency. |
| [Redox OS](https://doc.redox-os.org/book/) | Rust interfaces and isolation boundaries | Keep scheduler policy separate from architecture-specific context switching. |

## Validation

Run focused scheduler tests with:

```sh
cargo test --lib scheduler::eevdf::tests
```

This validates the library model only. It does not prove boot-time integration, preemption, context switching, multicore safety, or real-time guarantees. Those require kernel-level tests and emulator/hardware traces.

## Future development roadmap

1. **Correctness and invariants:** test signed lag boundaries, equal deadlines, blocked/running tasks, wakeup placement, and arithmetic saturation. Define units and overflow behavior for time and virtual runtime.
2. **Selection structure:** benchmark realistic runnable counts. Replace the vector with a suitable ordered structure only after measurements; preserve deterministic deadline and tie-breaking behavior.
3. **Kernel integration:** document and test the call path from timer interrupt/preemption through policy selection and architecture context switch. Remove mock/synthetic scheduling success paths from any production claim.
4. **Concurrency and SMP:** specify per-CPU ownership, locking, task migration, CPU affinity validation, and starvation bounds. Add race/stress tests before enabling multiple CPUs.
5. **Policy comparison:** compare against Linux EEVDF and xv6 round-robin on the same workloads. Publish hardware/emulator, configuration, workload, baseline, and repeated measurements.
6. **Observability:** add bounded counters for queue wait, dispatches, preemptions, and migration only when the runtime path can report them accurately.

## Completion criteria

- Unit and property tests cover eligibility, state transitions, accounting boundaries, and ordering.
- Kernel integration tests demonstrate timer-driven preemption and context restoration on the documented target.
- SMP claims have multi-CPU stress results and defined lock/ownership rules.
- Performance claims are reproducible and compare the same workload and configuration.
- Status in the README and capability matrix matches verified behavior.
