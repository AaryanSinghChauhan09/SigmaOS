# SigmaOS Dispatcher Master Architecture & Support Matrix Plan

## Invariants and scope
The dispatcher is responsible for scheduler execution, interrupt controller interface, context save/restore, run-state transitions, and deferred kernel work dispatch.

- Single scheduling decision path
- Context integrity
- Correct state ordering
- Interrupt and preemption safety
- Kernel/user return is a security boundary
- No blocking work in hard-interrupt context
- No lost wakeups or double execution
- Feature claims follow test tier

**Prioritize PRs 1–8** first for the primary milestone: boot → idle → timer → scheduler chooses a task → dispatcher switches → task blocks or exits → another task runs.

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
- Scheduler interface: explicit handoffs.
- Context switch: register integrity.
- Interrupt entry/exit: boundary check.
- Deferred work: bounded queue.
- Task lifecycle: state machine integrity.
- CPU topology/SMP: affinity enforcement.
- Observability: non-intrusive counters.
- Feature inventory: status alignment.

## PR template and merge gates
Require one maintainer approval for tests/docs and two qualified reviews for kernel changes.

## Completion criteria
- **Prototype:** Basic context switch.
- **Integrated:** Kernel-wide dispatch.
- **Supported:** Multi-arch validation.
