# SigmaOS Dispatcher Master Architecture & Support Matrix Plan

## Invariants and scope
- Single scheduling decision path
- Context integrity
- Correct state ordering
- Interrupt and preemption safety
- Kernel/user return is a security boundary
- No blocking work in hard-interrupt context
- No lost wakeups or double execution
- Feature claims follow test tier

## Proposed PR sequence
Prioritize PRs 1–8.

| 1 | `docs(dispatch): define scheduler-dispatcher-IRQ boundaries` |
| 2 | PR 2 |
| 3 | PR 3 |
| 4 | `feat(dispatch): connect scheduler selection to x86_64 context switch` |
| 5 | PR 5 |
| 6 | PR 6 |
| 7 | PR 7 |
| 8 | PR 8 |
| 9 | `feat(dispatch): add run-queue locking and SMP switch protocol` |
| 10 | PR 10 |
| 11 | PR 11 |
| 12 | PR 12 |
| 13 | PR 13 |
| 14 | PR 14 |
| 15 | PR 15 |
| 16 | `docs(dispatch): publish support matrix and recovery guide` |

## Periodic maintenance PR cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After an incident

## Component-specific rules
Specific rules for dispatcher mechanisms.

## PR template and merge gates
Authoritative merge gates.

## Completion criteria
- Prototype: Basic dispatcher model.
- Integrated: Kernel context switch connected.
- Supported: Multi-core SMP verified without lost wakeup.
