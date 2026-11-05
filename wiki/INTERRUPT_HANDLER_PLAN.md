# SigmaOS Interrupt Handling Master Architecture & Support Matrix Plan

## Invariants & Principles
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
|---|---|
| 1 | `docs(irq): audit interrupt paths and define support matrix` |
| 2 | `feat(irq): implement core interrupt descriptor table` |
| 3 | `feat(irq): add high-priority handler registration` |
| 4 | `feat(irq): support edge and level triggered interrupts` |
| 5 | `feat(irq): add spurious interrupt accounting` |
| 6 | `feat(irq): integrate lockless interrupt queue` |
| 7 | `feat(irq): implement bottom-half deferred work processing` |
| 8 | `feat(irq): add SMP affinity and IPI routing` |
| 9 | `feat(irq): provide MSI/MSI-X message allocation` |
| 10 | `feat(irq): add IO-APIC routing table support` |
| 11 | `feat(irq): implement ARM GICv2/GICv3 distributor` |
| 12 | `feat(irq): support nested interrupt contexts` |
| 13 | `feat(irq): add hardware watchdog interrupt strobe` |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` |
| 15 | `feat(irq): implement kernel profiling timer ticks` |
| 16 | `test(irq): add cross-architecture QEMU test harness` |
| 17 | `docs(irq): publish driver guide and architecture support matrix` |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: Basic functionality verified.
- Integrated: Cross-module integration passed.
- Supported: Reproducible evidence with high test coverage.
