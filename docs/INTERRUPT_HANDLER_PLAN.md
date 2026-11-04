# Interrupt Handling Master Architecture & Support Matrix Plan

## Purpose
Define the architecture, invariants, multi-arch support matrix, and development roadmap for SigmaOS interrupt handling.

## Architectural Invariants and Rules

1. Separate architecture entry, IRQ flow, and controller operations.
2. Keep hard-interrupt work bounded.
3. Specify edge vs level semantics.
4. Make ownership and teardown explicit.
5. Define spurious and unhandled behavior.
6. Do not sleep in hard-IRQ context.
7. Treat interrupt numbers as mapped identities.
8. SMP requires synchronization.
9. Test by evidence tier.

## Pull Request Sequence

| PR | Title | Status | Deliverables |
|---|---|---|---|
| 1 | `docs(irq): audit interrupt paths and define support matrix` | Done | Architecture audit & matrix |
| 2 | `feat(irq): define handler context contracts` | Planned | Trait contracts |
| 3 | `feat(irq): interrupt-frame layout checks` | Planned | Frame verification |
| 4 | `feat(irq): generic IRQ descriptors` | Planned | Descriptors & tables |
| 5 | `feat(irq): edge and level trigger abstractions` | Planned | Flow control |
| 6 | `feat(irq): spurious interrupt counter` | Planned | Spurious handling |
| 7 | `feat(irq): top-half and bottom-half split` | Planned | Deferred dispatch |
| 8 | `feat(irq): softirq tasklet queue` | Planned | Tasklets & workqueues |
| 9 | `feat(irq): x86_64 IOAPIC and LAPIC routing` | Planned | x86_64 APIC routing |
| 10 | `feat(irq): x86_64 MSI/MSI-X allocation` | Planned | PCI MSI vectors |
| 11 | `feat(irq): aarch64 GICv2/v3 driver` | Planned | ARM GIC routing |
| 12 | `feat(irq): aarch64 PPI/SPI distribution` | Planned | ARM core interrupts |
| 13 | `feat(irq): riscv64 PLIC controller` | Planned | RISC-V PLIC driver |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` | Planned | QEMU PLIC runtime tests |
| 15 | `test(irq): stress test concurrent interrupts` | Planned | Race condition stress test |
| 16 | `security(irq): privilege and capability enforcement` | Planned | Capability gating |
| 17 | `docs(irq): publish driver guide and architecture support matrix` | Planned | Driver documentation |

## Maintenance Cadence

- Every PR: Static checks, unit tests, and regression test suite.
- Weekly: Component health and test matrix verification.
- Monthly: Revalidate full cross-architecture interrupt handling on QEMU.
- Quarterly: Architecture review, performance analysis, and security audit.
- Each release candidate: Full stress tests and multi-core interrupt distribution validation.
- After each incident: Root cause analysis and new regression test cases.

## Completion Criteria

- Prototype: Architecture specification and unit test coverage.
- Integrated: Runtime path validated with QEMU boot test.
- Supported: Reproducible multi-arch hardware/virtual test logs.
