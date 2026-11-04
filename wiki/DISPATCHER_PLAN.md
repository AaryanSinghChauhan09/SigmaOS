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
Prioritize PRs 1–8 before expanded capabilities.

| PR | Title | Status |
|---|---|---|
| 1 | `docs(dispatch): define scheduler-dispatcher-IRQ boundaries` | Merged |
| 2 | `feat(dispatch): context frame setup` | Merged |
| 3 | `feat(dispatch): task switch logic` | Merged |
| 4 | `feat(dispatch): connect scheduler selection to x86_64 context switch` | Merged |
| 5 | `feat(dispatch): deferred work queue` | Merged |
| 6 | `feat(dispatch): bottom-half scheduling` | Merged |
| 7 | `feat(dispatch): user return trampoline` | Merged |
| 8 | `feat(dispatch): kernel thread switch` | Merged |
| 9 | `feat(dispatch): add run-queue locking and SMP switch protocol` | Merged |
| 10 | `feat(dispatch): tlb invalidation` | Merged |
| 11 | `feat(dispatch): fpu lazy restore` | Merged |
| 12 | `feat(dispatch): arm64 context switch` | Merged |
| 13 | `feat(dispatch): riscv context switch` | Merged |
| 14 | `feat(dispatch): latency metrics` | Merged |
| 15 | `test(dispatch): stress validation` | Merged |
| 16 | `docs(dispatch): publish support matrix and recovery guide` | Merged |

## Periodic maintenance PR cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After an incident

## Component-specific rules
Component rules.

## PR template and merge gates
Template and gates.

## Completion criteria
- Prototype:
- Integrated:
- Supported:
