# SigmaOS Dispatcher Master Architecture & Support Matrix Plan

## Invariants and scope

The dispatcher bridges the scheduler, interrupt controller, and architecture execution contexts.
Subsystems:
- dispatcher
- scheduler
- interrupt controller
- context save/restore
- run-state transitions
- deferred kernel work dispatch

### Invariants
1. Single scheduling decision path
2. Context integrity
3. Correct state ordering
4. Interrupt and preemption safety
5. Kernel/user return is a security boundary
6. No blocking work in hard-interrupt context
7. No lost wakeups or double execution
8. Feature claims follow test tier

**Prioritize PRs 1–8** first to achieve the initial execution cycle:
boot → idle → timer → scheduler chooses a task → dispatcher switches → task blocks or exits → another task runs

## Proposed PR sequence

| 1 | `docs(dispatch): define scheduler-dispatcher-IRQ boundaries` |
| 2 | `test(dispatch): add context-frame layout and invariant tests` |
| 3 | `refactor(dispatch): define architecture context-switch interface` |
| 4 | `feat(dispatch): connect scheduler selection to x86_64 context switch` |
| 5 | `feat(dispatch): add safe first-task and idle-task handoff` |
| 6 | `feat(dispatch): connect timer interrupt return to reschedule path` |
| 7 | `feat(dispatch): connect syscall and exception return-to-user hooks` |
| 8 | `feat(dispatch): add wakeup and yield handoff semantics` |
| 9 | `feat(dispatch): add run-queue locking and SMP switch protocol` |
| 10 | `feat(dispatch): implement interrupt controller flow interface` |
| 11 | `feat(dispatch): add bounded deferred-work executor` |
| 12 | `feat(dispatch): define deferred-work priority and forward-progress rules` |
| 13 | `feat(dispatch): add dispatcher tracing and counters` |
| 14 | `test(dispatch): add timer, interrupt, and workqueue fault injection` |
| 15 | `test(dispatch): add QEMU end-to-end and architecture matrix` |
| 16 | `docs(dispatch): publish support matrix and recovery guide` |

## Periodic maintenance PR cadence

- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After an incident

## Component-specific rules

- Scheduler interface: Scheduler selects tasks without touching CPU registers directly.
- Context switch: Architecture-specific save and restore must be bounded and race-free.
- Interrupt entry/exit: Top halves must not block; transitions must be atomic.
- Deferred work: Workqueues and bottom-halves must guarantee forward progress.
- Task lifecycle: Transitions must follow legal states (New -> Runnable -> Running -> Blocked/Exited).
- CPU topology/SMP: Per-CPU runqueues and locks must prevent race conditions.
- Observability: Diagnostic counters and trace points must not perturb timing.
- Feature inventory: Capabilities must be verified against runtime test evidence.

## PR template and merge gates

Require one maintainer approval for tests/docs and two qualified reviews for core context-switch and dispatcher changes.

## Completion criteria

- **Prototype:** Unit tests and interface definitions.
- **Integrated:** Runtime path executed under QEMU boot test.
- **Supported:** Validated multi-core execution on physical hardware or virtual matrix.
