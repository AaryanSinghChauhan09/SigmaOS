# SigmaOS Interrupt Handling Master Architecture & Support Matrix Plan

## Overview
Comprehensive plan for interrupt architecture, IRQ flow, and controller operations.

## Core Invariants
- Separate architecture entry, IRQ flow, and controller operations.
- Keep hard-interrupt work bounded.
- Specify edge vs level semantics.
- Make ownership and teardown explicit.
- Define spurious and unhandled behavior.
- Do not sleep in hard-IRQ context.
- Treat interrupt numbers as mapped identities.
- SMP requires synchronization.
- Test by evidence tier.

## PR Sequence Matrix
| PR | Title |
|---|---|
| 1 | `docs(irq): audit interrupt paths and define support matrix` |
| 2 | `feat(irq): implement core IRQ abstraction and registration` |
| 3 | `feat(irq): implement top-half interrupt dispatch loop` |
| 4 | `feat(irq): bounded handler execution time limits` |
| 5 | `feat(irq): interrupt thread / bottom-half scheduler` |
| 6 | `feat(irq): x86_64 IOAPIC and local APIC driver` |
| 7 | `feat(irq): MSI and MSI-X vector allocation framework` |
| 8 | `feat(irq): spurious interrupt detector and counter` |
| 9 | `feat(irq): per-CPU interrupt statistics and monitoring` |
| 10 | `feat(irq): interrupt affinity and CPU routing` |
| 11 | `feat(irq): ARM64 GICv2/GICv3 interrupt controller support` |
| 12 | `feat(irq): device driver interrupt teardown and cleanup` |
| 13 | `feat(irq): interrupt handler denial-of-service throttling` |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` |
| 15 | `feat(irq): dynamic interrupt rebalancing` |
| 16 | `test(irq): comprehensive interrupt stress and fuzzing suite` |
| 17 | `docs(irq): publish driver guide and architecture support matrix` |

## Maintenance Cadence
- Every PR: run fast unit tests
- Weekly: review pending PRs and metrics
- Monthly: sync architecture roadmap
- Quarterly: full security audit
- Each release candidate: verify all tiers
- After each incident: conduct postmortem and update rules

## Completion Criteria
- Prototype: unit tests and initial flow
- Integrated: runtime path and QEMU evidence
- Supported: reproducible production verification
