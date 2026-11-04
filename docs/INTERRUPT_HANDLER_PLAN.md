# Interrupt Handling Master Architecture & Support Matrix Plan

## Invariants and Rules
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
| --- | --- |
| 1 | `docs(irq): audit interrupt paths and define support matrix` |
| 2 | `feat(irq): separate arch entry and irq flow` |
| 3 | `feat(irq): bounded hard irq handlers` |
| 4 | `feat(irq): edge vs level triggered semantics` |
| 5 | `feat(irq): explicit irq ownership and teardown` |
| 6 | `feat(irq): spurious and unhandled irq counters` |
| 7 | `feat(irq): non-sleeping hard irq context verification` |
| 8 | `feat(irq): irq number mapping identities` |
| 9 | `feat(irq): smp interrupt synchronization locks` |
| 10 | `test(irq): evidence tier test matrix execution` |
| 11 | `feat(irq): x86_64 IOAPIC and LAPIC dispatch` |
| 12 | `feat(irq): arm gic v2 v3 interrupt controller` |
| 13 | `feat(irq): msi and msix message signaled interrupts` |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` |
| 15 | `feat(irq): bottom half softirq tasklet scheduler` |
| 16 | `feat(irq): telemetry counters and diagnostics` |
| 17 | `docs(irq): publish driver guide and architecture support matrix` |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: initial scaffold and mock unit tests
- Integrated: kernel runtime wiring and functional checks
- Supported: stress test, SMP safety, and reproducible boot evidence
