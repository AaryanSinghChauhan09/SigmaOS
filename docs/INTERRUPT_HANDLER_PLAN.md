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
| PR | Title | Status |
|---|---|---|
| 1 | `docs(irq): audit interrupt paths and define support matrix` | Merged |
| 2 | `feat(irq): implement core irq chip interface` | Merged |
| 3 | `feat(irq): add apic controller support` | Merged |
| 4 | `feat(irq): implement ioapic routing` | Merged |
| 5 | `feat(irq): add msi-x allocation` | Merged |
| 6 | `feat(irq): add bounded hard-irq queue` | Merged |
| 7 | `feat(irq): implement bottom-half worker threads` | Merged |
| 8 | `feat(irq): spurious irq accounting and mitigation` | Merged |
| 9 | `feat(irq): irq affinity and smp balancing` | Merged |
| 10 | `feat(irq): nested interrupt guard` | Merged |
| 11 | `feat(irq): arm gicv2 controller driver` | Merged |
| 12 | `feat(irq): arm gicv3 its virtualization` | Merged |
| 13 | `feat(irq): riscv plic controller driver` | Merged |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` | Merged |
| 15 | `feat(irq): hardware trace integration` | Merged |
| 16 | `feat(irq): lockdep interrupt context validation` | Merged |
| 17 | `docs(irq): publish driver guide and architecture support matrix` | Merged |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: Basic implementation validated.
- Integrated: Integrated across all supported architectures.
- Supported: Production-ready and verified.
