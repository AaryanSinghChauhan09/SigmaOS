# SigmaOS Interrupt Handling Master Architecture & Support Matrix Plan

## Overview
This document defines the 5-stage decoupled interrupt pipeline, architecture contracts, maintenance rules, and the 17-PR sequence for interrupt handling across x86_64, aarch64, and riscv64.

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

## 17-PR Delivery Sequence
| PR | Title | Category |
|---|---|---|
| 1 | `docs(irq): audit interrupt paths and define support matrix` | Documentation |
| 2 | `feat(irq): define handler context contracts` | Architecture |
| 3 | `feat(irq): interrupt-frame layout checks` | Core |
| 4 | `feat(irq): generic IRQ descriptors` | Core |
| 5 | `feat(irq): mask unmask and eoi flow control` | Controllers |
| 6 | `feat(irq): spurious interrupt tracking` | Core |
| 7 | `feat(irq): lockless bottom-half deferred queue` | Deferred |
| 8 | `feat(irq): smp affinity and target routing` | Multi-core |
| 9 | `feat(irq): edge-triggered vs level-triggered handling` | Semantics |
| 10 | `feat(irq): per-cpu interrupt telemetry and metrics` | Observability |
| 11 | `feat(irq): validate x86_64 IOAPIC on QEMU` | Validation |
| 12 | `feat(irq): validate x86_64 Local APIC timer and IPI` | Validation |
| 13 | `feat(irq): validate aarch64 GICv3 on QEMU` | Validation |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` | Validation |
| 15 | `feat(irq): fault recovery and teardown guarantees` | Resilience |
| 16 | `feat(irq): hardware fuzzing and stress injection` | Hardening |
| 17 | `docs(irq): publish driver guide and architecture support matrix` | Documentation |

## Maintenance Cadence
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Completion Criteria
- Prototype: Basic vector routing and unit tests.
- Integrated: Driver dispatch and QEMU integration tests.
- Supported: Multi-architecture boot, stress testing, and documentation verified.
