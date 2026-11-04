# Interrupt Handler Plan

## Architecture Invariants
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
| 2 | `feat(irq): define handler context contracts` |
| 3 | `feat(irq): interrupt-frame layout checks` |
| 4 | `feat(irq): generic IRQ descriptors` |
| 5 | `feat(irq): controller interface` |
| 6 | `feat(irq): APIC routing` |
| 7 | `feat(irq): GICv3 routing` |
| 8 | `feat(irq): PLIC routing` |
| 9 | `feat(irq): spurious interrupt tracking` |
| 10 | `feat(irq): top-half dispatch` |
| 11 | `feat(irq): deferred work queue` |
| 12 | `feat(irq): interrupt affinities` |
| 13 | `feat(irq): validate x86 APIC dispatch on QEMU` |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` |
| 15 | `feat(irq): validate ARM GICv3 dispatch on QEMU` |
| 16 | `feat(irq): stress and preemption tests` |
| 17 | `docs(irq): publish driver guide and architecture support matrix` |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: Basic vector dispatch functional.
- Integrated: Driver registration and top/bottom-half split operational.
- Supported: Production validation on all supported targets.
