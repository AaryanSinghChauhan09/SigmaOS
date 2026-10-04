# Future Development: Kernel, Scheduling, and Memory

**Status:** Proposal. This page describes work to evaluate; it does not claim that the listed Linux or BSD features are implemented in SigmaOS.

## Scope

Coordinate CPU scheduling, memory placement, asynchronous I/O, and kernel interfaces. Track the current implementation in [`scheduler.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/kernel/scheduler.rs), [`memory.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/kernel/memory.rs), [`rcu.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/kernel/rcu.rs), and [`io_uring.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/io/io_uring.rs).

## Linux and BSD ideas to evaluate

| Project | Design idea | SigmaOS question |
|---|---|---|
| Linux | EEVDF scheduling, cgroup resource controls, and io_uring | Can fairness and async completion work under SigmaOS's task and memory model? |
| FreeBSD | ULE scheduling and cpuset affinity | Which affinity and topology policies are useful across supported machines? |
| NetBSD | Rump kernels and modular subsystems | Can selected drivers or services be isolated behind stable interfaces? |
| OpenBSD | Small, auditable kernel interfaces and defensive defaults | Can interfaces reject invalid state before it reaches scheduler or memory code? |

These are reference designs, not promises of compatibility or performance parity.

## Proposed work sequence

1. **Write down invariants.** Define task states, run-queue ownership, wakeup behavior, CPU affinity, memory ownership, and lock ordering.
2. **Make scheduling observable.** Add counters for queue wait, context switches, migrations, and starvation. Keep instrumentation optional and low overhead.
3. **Integrate topology and affinity.** Model discovered CPUs and NUMA nodes from validated hardware data. Define fallback behavior when topology data is absent or malformed.
4. **Harden asynchronous I/O.** Specify buffer lifetime, pinning, cancellation, completion ordering, and error behavior before exposing registered-buffer operations.
5. **Compare policy changes.** Measure fixed workloads on emulators and supported hardware; keep results reproducible and publish the workload and configuration.

## Completion criteria

- Scheduler invariants are documented and checked at subsystem boundaries.
- Affinity and migration reject invalid CPU or node IDs without panics.
- Empty or unavailable topology data has a defined fallback.
- Async I/O registration validates ranges and ownership; it does not report success for metadata-only bookkeeping.
- Performance claims include hardware, workload, build configuration, baseline, and repeated measurements.

## Maintenance

Update this page when the related source interfaces or integration status changes. Mark a proposal implemented only after the path is wired into the running kernel and its behavior is verified; distinguish prototypes from runtime support.
