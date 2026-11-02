# Interrupt Handler Master Architecture & Support Matrix Plan

## Architectural Invariants
- Separate architecture entry, IRQ flow, and controller operations.
- Keep hard-interrupt work bounded.
- Specify edge vs level semantics.
- Make ownership and teardown explicit.
- Define spurious and unhandled behavior.
- Do not sleep in hard-IRQ context.
- Treat interrupt numbers as mapped identities.
- SMP requires synchronization.
- Test by evidence tier.

## PR Sequence
| 1 | `docs(irq): audit interrupt paths and define support matrix` |
| 2 | PR 2 |
| 3 | PR 3 |
| 4 | PR 4 |
| 5 | PR 5 |
| 6 | PR 6 |
| 7 | PR 7 |
| 8 | PR 8 |
| 9 | PR 9 |
| 10 | PR 10 |
| 11 | PR 11 |
| 12 | PR 12 |
| 13 | PR 13 |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` |
| 15 | PR 15 |
| 16 | PR 16 |
| 17 | `docs(irq): publish driver guide and architecture support matrix` |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: Defined
- Integrated: Tested
- Supported: Production
