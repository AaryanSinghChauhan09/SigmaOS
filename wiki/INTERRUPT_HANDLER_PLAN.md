# SigmaOS Interrupt Handling Master Architecture & Support Matrix Plan

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
| 2 | Core IRQ dispatch table abstraction |
| 3 | Hard-IRQ vs Soft-IRQ worker model |
| 4 | Edge and level interrupt lifecycle manager |
| 5 | Dynamic device IRQ reservation engine |
| 6 | Spurious interrupt mitigation filter |
| 7 | Non-blocking spinlock synchronization guards |
| 8 | Virtual to physical IRQ mapping matrix |
| 9 | SMP multi-core affinity router |
| 10 | Tier 1 test matrix and evidence suite |
| 11 | x86_64 IOAPIC & LAPIC driver parity |
| 12 | ARM64 GICv2/GICv3 interrupt distributor |
| 13 | RISC-V PLIC controller abstraction |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` |
| 15 | MSI / MSI-X message signaled interrupts |
| 16 | Latency benchmark and jitter profiling |
| 17 | `docs(irq): publish driver guide and architecture support matrix` |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: Basic dispatch on emulator
- Integrated: Cross-architecture driver support
- Supported: Full production tier evidence and verification
