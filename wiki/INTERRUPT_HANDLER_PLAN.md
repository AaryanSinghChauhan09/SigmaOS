# SigmaOS Interrupt Handler Master Architecture & Support Matrix Plan

## Invariants
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
| PR | Title |
| 1 | `docs(irq): audit interrupt paths and define support matrix` |
| 2 | `feat(irq): define handler context contracts` |
| 3 | `feat(irq): interrupt-frame layout checks` |
| 4 | `feat(irq): generic IRQ descriptors` |
| 5 | `feat(irq): APIC flow controller` |
| 6 | `feat(irq): IOAPIC redirection table` |
| 7 | `feat(irq): MSI/MSI-X allocator` |
| 8 | `feat(irq): top-half dispatch bounded work` |
| 9 | `feat(irq): bottom-half softirq and tasklets` |
| 10 | `feat(irq): threaded interrupt handlers` |
| 11 | `feat(irq): spurious interrupt rate limiter` |
| 12 | `feat(irq): SMP affinity and routing` |
| 13 | `feat(irq): GICv3 ITS message signaled interrupts` |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` |
| 15 | `feat(irq): telemetry and stats counter matrix` |
| 16 | `feat(irq): stress test interrupt storm resilience` |
| 17 | `docs(irq): publish driver guide and architecture support matrix` |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: Unit tests and basic dispatch
- Integrated: Runtime path across multiple controllers
- Supported: Reproducible multi-arch testing and validation
