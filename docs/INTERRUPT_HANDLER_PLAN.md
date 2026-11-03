# Interrupt Handler Master Architecture & Support Matrix Plan

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
| 1 | `docs(irq): audit interrupt paths and define support matrix` |
| 2 | Core IRQ routing |
| 3 | APIC controller support |
| 4 | IOAPIC routing |
| 5 | MSI / MSI-X support |
| 6 | Nested interrupt prevention |
| 7 | Bottom-half tasklet scheduling |
| 8 | Threaded interrupt handlers |
| 9 | Spurious interrupt counters |
| 10 | Real-time low latency handlers |
| 11 | ARM GICv2/v3 support |
| 12 | ARM GICv4 virtual interrupts |
| 13 | RISC-V CLINT timer/IPI |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` |
| 15 | Interrupt affinity balancing |
| 16 | Hardware interrupt profiling |
| 17 | `docs(irq): publish driver guide and architecture support matrix` |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: Initial implementation and interface definition.
- Integrated: Wired into kernel and unit tested.
- Supported: Production ready with hardware validation.
