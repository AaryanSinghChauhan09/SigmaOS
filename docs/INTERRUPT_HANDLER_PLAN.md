# SigmaOS Interrupt Handler Master Architecture and Support Matrix

This document is the evidence gate for the interrupt subsystem. It follows a
small, explicit pipeline: architecture entry, IRQ flow, controller operations,
deferred work, and user-visible recovery.

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

Architecture entry records the CPU-specific trap or vector frame. The generic IRQ
flow maps that frame to an interrupt identity, checks masking and ownership,
dispatches a bounded handler, sends controller-specific EOI, and schedules
deferred work when necessary. Controller operations must define mask, unmask,
acknowledge, EOI, and teardown behavior for APIC, GICv3, and RISC-V PLIC.

Edge-triggered sources are acknowledged without losing a new edge; level-triggered
sources remain serviced or masked until the device deasserts the line. Spurious
and unhandled interrupts are counted, rate-limited, logged without blocking, and
eventually quarantined according to the controller contract. Handler ownership is
released before driver teardown, and SMP registration, dispatch, and counters use
synchronization appropriate to the shared state.

## Evidence tiers and completion criteria

Prototype: a documented contract, isolated unit tests, and deterministic
simulated dispatch.

Integrated: architecture entry, generic routing, controller operations, and
deferred work execute together in a QEMU or equivalent smoke test.

Supported: repeatable multi-architecture evidence, teardown and fault-path tests,
bounded latency measurements, and a documented recovery path.

Maintenance cadence: Every PR checks invariants and focused tests. Weekly reviews
inspect new interrupt paths and deferred-work ownership. Monthly reviews refresh
the architecture support matrix. Quarterly reviews run QEMU multi-architecture
smoke tests. Each release candidate requires the supported evidence tier. After each incident,
record the trigger, controller state, recovery, and regression
test.

## Seventeen-PR delivery sequence

| PR | Title | Evidence gate |
|---|---|---|
| 1 | `docs(irq): audit interrupt paths and define support matrix` | Inventory and architecture matrix |
| 2 | `docs(irq): define handler context contracts` | Context and bounded-work rules |
| 3 | `feat(irq): add interrupt frame layout checks` | Architecture layout tests |
| 4 | `feat(irq): add generic IRQ descriptors` | Descriptor ownership tests |
| 5 | `feat(irq): define edge and level semantics` | Trigger-mode tests |
| 6 | `feat(irq): add mask unmask and EOI contracts` | Controller operation tests |
| 7 | `feat(irq): add spurious interrupt accounting` | Fault-path evidence |
| 8 | `feat(irq): add deferred work handoff` | No-sleep and queue tests |
| 9 | `feat(irq): add SMP synchronization` | Concurrent dispatch tests |
| 10 | `feat(irq): validate APIC dispatch` | x86 smoke evidence |
| 11 | `feat(irq): validate GICv3 dispatch` | AArch64 smoke evidence |
| 12 | `feat(irq): validate PLIC dispatch` | RISC-V contract evidence |
| 13 | `test(irq): add teardown and ownership coverage` | Driver lifecycle tests |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` | QEMU RISC-V evidence |
| 15 | `test(irq): add fault injection and recovery` | Incident and recovery tests |
| 16 | `perf(irq): measure bounded handler latency` | Repeatable latency measurements |
| 17 | `docs(irq): publish driver guide and architecture support matrix` | Release documentation |

A PR advances only when its evidence gate is reproducible and linked from the
implementation change. A missing hardware or QEMU result remains partial rather
than being represented as supported.

