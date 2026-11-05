# SigmaOS Interrupt Handler Master Architecture & Support Matrix Plan

## Overview
Master Architecture, Support Matrix, and 17-PR Plan for SigmaOS interrupt handling: 5-stage decoupled pipeline, multi-arch vector contracts, and deferred work dispatch.

## Invariants and Mandatory Rules
1. Separate architecture entry, IRQ flow, and controller operations.
2. Keep hard-interrupt work bounded.
3. Specify edge vs level semantics.
4. Make ownership and teardown explicit.
5. Define spurious and unhandled behavior.
6. Do not sleep in hard-IRQ context.
7. Treat interrupt numbers as mapped identities.
8. SMP requires synchronization.
9. Test by evidence tier.

## PR Sequence Roadmap
| PR | Title | Scope |
|---|---|---|
| 1 | `docs(irq): audit interrupt paths and define support matrix` | Baseline audit and documentation |
| 2 | `feat(irq): define handler context contracts` | Hard-IRQ vs soft-IRQ handler boundaries |
| 3 | `feat(irq): interrupt-frame layout checks` | Architecture register saving verification |
| 4 | `feat(irq): generic IRQ descriptors` | Bounded vector table and handler registry |
| 5 | `feat(irq): x86_64 IOAPIC and LAPIC routing` | Hardware interrupt controller abstraction |
| 6 | `feat(irq): AArch64 GICv3 distributor and redistributor` | ARM interrupt routing |
| 7 | `feat(irq): RISC-V PLIC support` | RISC-V platform level interrupt controller |
| 8 | `feat(irq): MSI and MSI-X allocation` | PCIe message signaled interrupts |
| 9 | `feat(irq): spurious interrupt suppression` | Filtering unhandled and spurious vectors |
| 10 | `feat(irq): deferred work queue` | Bottom-half tasklets and softirqs |
| 11 | `feat(irq): nested interrupt handling` | Reentrant safe exception stacks |
| 12 | `feat(irq): CPU affinity and balance` | Multi-core IRQ steering |
| 13 | `feat(irq): validate x86_64 and AArch64 QEMU` | Virtualized target validation |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` | RISC-V QEMU automated testing |
| 15 | `feat(irq): crash recovery and watchdog` | Watchdog timer and NMI watchdog |
| 16 | `feat(irq): telemetry and profiling` | Latency tracing and event counters |
| 17 | `docs(irq): publish driver guide and architecture support matrix` | Final driver guidelines |

## Maintenance Cadences
- Every PR: Automated build, unit tests, and invariant checks.
- Weekly: Hardware regression test pass on supported platforms.
- Monthly: Latency and interrupt overhead benchmark suite.
- Quarterly: Architecture audit and vector table verification.
- Each release candidate: Full end-to-end stress test across SMP configurations.
- After each incident: Root cause analysis and new regression test addition.

## Completion Criteria
- Prototype: Core vector tables and mock dispatch functional.
- Integrated: Driver integration, IOAPIC/GIC routing, and deferred work operational.
- Supported: Production-tested, hardened, validated with zero panics in hard-IRQ context.
