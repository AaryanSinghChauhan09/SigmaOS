# SigmaOS Interrupt Handling Master Architecture & Support Matrix Plan

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
| 2 | Foundation |
| 3 | Core |
| 4 | Architecture |
| 5 | Traps |
| 6 | Flow |
| 7 | Controller |
| 8 | Drivers |
| 9 | Multi-queue |
| 10 | Timers |
| 11 | VirtIO |
| 12 | APIC |
| 13 | ARM GIC |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` |
| 15 | Testing |
| 16 | Diagnostics |
| 17 | `docs(irq): publish driver guide and architecture support matrix` |

## Maintenance Cadence

- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria

- Prototype: Architecture scaffolded and tested under mock harness.
- Integrated: Driver subsystem and APIC/PLIC dispatch verified in QEMU.
- Supported: Production verified with zero regression under stress testing.
