# ADR-004: Fixed-Priority Preemptive Scheduler

## Status
Accepted

## Upstream study
xv6 `scheduler()` context switch and CachyOS BORE / Linux CFS run queues.

## SigmaOS decision
Implement a round-robin preemptive scheduler driven by the HPET/APIC timer interrupt with thread context saving/restoring (`rsp`, `rip`, registers).

## Scope
- Thread control blocks (`TCB`) and state transitions (`Ready`, `Running`, `Blocked`, `Zombie`)
- Per-CPU run queues
- Context switch routine

## Non-goals
- Multi-CPU NUMA affinity balancing
- Real-time `SCHED_FIFO` / `SCHED_RR` priority classes
- Energy-aware scheduling

## Validation
- Context switch unit tests
- Two-process alternation integration test in QEMU
