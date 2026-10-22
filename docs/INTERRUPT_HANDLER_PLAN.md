# SigmaOS Interrupt Handler Master Architecture, Support Matrix, and 17-PR Plan

## Executive Summary & Architecture Overview

The SigmaOS interrupt handling infrastructure connects low-level hardware vector entry points to high-level kernel event handlers. This specification distinguishes **exceptions** (CPU-detected synchronous faults/traps), **device interrupts** (asynchronous IRQs delivered via APIC, GICv3, or PLIC), and **deferred work** (bottom-half processing executing in a schedulable context).

The architecture adheres to a 5-stage decoupled interrupt pipeline inspired by Linux's generic IRQ architecture and FreeBSD's filter/threaded-handler model:

1. **Vector Entry Stage:** Architecture-specific assembly stubs preserve CPU registers, save stack frame layout, and transfer control to generic kernel entry.
2. **Generic Dispatch Stage:** Mapped logical IRQs are resolved from hardware vector numbers via domain lookups (`irq_domain`), ensuring hardware details remain isolated from generic subsystem handlers.
3. **Controller Flow Stage:** Controller-specific flow logic (edge, level, fast-EOI, or per-CPU) executes appropriate mask, acknowledge, and end-of-interrupt (EOI) sequencing.
4. **Handler Invocation Stage:** Fast hard-IRQ handlers confirm device assertion, capture minimal state, and trigger optional deferred/threaded handler threads.
5. **Return & Deferred Work Stage:** Preemption and rescheduling checks run at safe architectural return boundaries while deferred work queues are scheduled outside hard-IRQ context.

---

## Subsystem Architecture Support Matrix

| Architecture | Controller / Vector Interface | Code Reference / Files | Status | Test Evidence Level | Known Limitations & Requirements |
|---|---|---|---|---|---|
| **x86_64** | IDT / Local APIC & IO-APIC | `src/arch/x86_64/interrupts.rs`, `src/kernel/interrupt.rs` | Modeled / Structural | Unit / Structural (`rustc --test`) | Requires QEMU boot validation for vector entry and APIC timer dispatch |
| **AArch64** | Exception Vectors / GICv3 | `src/kernel/irq/irq_controller.rs` | Compile-only | Model / Unit | Compile-only; pending QEMU `virt` machine boot validation |
| **RISC-V (rv64)** | Trap Vectors / PLIC | `src/kernel/irq/irq_domain.rs` | Compile-only | Model / Unit | Compile-only; pending QEMU `virt` machine boot validation |
| **Generic IRQ** | Logical Domain & SoftIRQ / Workqueue | `src/kernel/irq/mod.rs`, `src/kernel/irq/softirq.rs` | Integrated | Standalone Unit Tests Pass | SoftIRQ and delayed workqueues tested via unit harness |

---

## Handler Contexts and Execution Contracts

SigmaOS enforces clear execution boundaries across five explicit contexts:

1. **Hard-IRQ Context:** Runs with interrupts disabled or masked on local CPU. Non-blocking, no allocation, bounded execution time, no sleeping or yielding. Top half only.
2. **Threaded / Deferred Context:** Schedulable kernel thread or workqueue context. Allowed to sleep, acquire mutexes, and allocate memory. Performs long I/O processing.
3. **Exception Context:** Synchronous CPU exception state (e.g. Page Fault, General Protection Fault). Resolves recoverable faults or triggers process signals / controlled kernel panic.
4. **NMI / Critical Context:** Non-maskable interrupt or emergency stop. Cannot lock non-reentrant state or invoke non-NMI safe procedures.
5. **Process Context:** Standard user/kernel thread execution context with full preemption and scheduling capability.

---

## Core Rules and Invariants

1. **Separate architecture entry, IRQ flow, and controller operations.** Architecture entry stubs hand off to the generic IRQ layer; flow handlers enforce edge/level/per-CPU policy; controller backends handle mask, unmask, ack, EOI, and affinity.
2. **Keep hard-interrupt work bounded.** Hard-IRQ handlers identify device assertion, capture minimal state, acknowledge the controller/device, and defer blocking work to threaded handlers.
3. **Specify edge vs level semantics.** Maintain strict ordering for mask, acknowledge, service, unmask, and EOI to prevent lost edges or level-triggered interrupt storms.
4. **Make ownership and teardown explicit.** Registration returns a stable handle; unregistration disables the source, waits for in-flight handlers, and drains queued work before freeing handler state.
5. **Define spurious and unhandled behavior.** Unmapped or unhandled IRQs are counted, rate-limited, and contained without panicking the system by default.
6. **Do not sleep in hard-IRQ context.** Debug builds validate preemption and interrupt-disabled state to prevent sleeping or blocking in hard-IRQ handlers.
7. **Treat interrupt numbers as mapped identities.** Separate hardware IRQs from kernel logical IRQs using domain mapping abstractions (`irq_domain`).
8. **SMP requires synchronization.** Shared device state across CPUs must be guarded by spinlocks; IRQ migration and CPU-offlining require device/queue quiescence.
9. **Test by evidence tier.** Validate state machines via unit tests, vector entry and controllers via QEMU booted tests, and physical devices via hardware verification.

---

## 17-PR Sequence Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **1** | `docs(irq): audit interrupt paths and define support matrix` | Audit IDT/APIC, GICv3, PLIC, timer/IPI, and deferred work paths. | Support matrix published with file links, status, test tiers, and limitations. |
| **2** | `docs(irq): define handler context and lifecycle contracts` | Define hard-IRQ, threaded, exception, NMI, and process contexts and API contracts. | Blocking, locking, ownership, and teardown rules documented for all handler APIs. |
| **3** | `test(irq): add vector and interrupt-frame layout checks` | Add compile-time and runtime tests for IDT entries, exception/IRQ frames, and alignment. | Stack frame layout verified for error-code and non-error-code vectors. |
| **4** | `feat(irq): add generic IRQ descriptors and logical mapping` | Implement IRQ descriptors, logical ID allocation, and `irq_domain` translation. | Duplicate mappings fail safely; unregister invalidates tokens; lookup is $O(1)$ or bounded. |
| **5** | `feat(irq): define interrupt-controller chip operations` | Create chip operation interface (mask, unmask, ack, EOI, trigger, affinity). | APIC, GIC, and PLIC backends implement supported operations with explicit error returns for unsupported ones. |
| **6** | `feat(irq): implement generic edge level and fasteoi flows` | Add distinct flow handlers for edge, level, fast-EOI, and per-CPU sources. | Simulated tests confirm exact mask/ack/service/EOI sequences for edge and level interrupts. |
| **7** | `feat(irq): connect x86_64 IDT entry to APIC dispatch` | Wire real IDT assembly entry stubs to APIC vector mapping and generic dispatcher. | QEMU x86_64 boot test receives timer IRQ, dispatches handler, sends EOI, and returns safely. |
| **8** | `feat(irq): add handler registration and synchronized teardown` | Implement register/unregister, nesting depth counters, and in-flight teardown synchronization. | Teardown waits for in-flight dispatches; concurrent unregister and IRQ dispatch does not access freed memory. |
| **9** | `feat(irq): add spurious and unhandled interrupt containment` | Implement unhandled IRQ counters, storm detection, auto-masking, and recovery hooks. | Spurious interrupts do not panic; storm detection bounds unhandled IRQ rates with rate-limited diagnostics. |
| **10** | `feat(irq): add threaded handlers and deferred-work interface` | Connect top-half hard-IRQ handlers to threaded handlers and workqueues. | Deferred work executes in schedulable context; driver teardown flushes and drains pending work. |
| **11** | `feat(irq): add APIC timer and IPI integration tests` | Calibrate APIC timer in QEMU; test timer delivery, reschedule IPIs, and TLB shootdown IPIs. | Monotonic timer ticks in QEMU; IPIs reach target CPUs without storms or lost reschedules. |
| **12** | `feat(irq): add IRQ affinity and CPU-hotplug policy` | Implement affinity masking, CPU migration, and queue quiescence during CPU offline. | IRQs target only online allowed CPUs; queues are quiesced prior to migration. |
| **13** | `feat(irq): validate AArch64 GICv3 dispatch on QEMU` | Connect exception vectors, GIC claim/EOI, and generic dispatch on QEMU `virt`. | QEMU AArch64 boots and passes timer and device IRQ dispatch tests. |
| **14** | `feat(irq): validate RISC-V PLIC dispatch on QEMU` | Connect trap entry, PLIC claim/complete, and generic dispatch on QEMU `virt`. | QEMU RISC-V boots and passes timer and external IRQ tests with safe handling of claim 0. |
| **15** | `test(irq): add fault-injection and interrupt-storm suite` | Add fault injection for missing EOI, storm events, registration failure, and teardown races. | Test suite catches lost IRQs, double completion, use-after-free, and lockup regressions. |
| **16** | `feat(irq): add interrupt statistics and trace interface` | Expose per-CPU/per-vector counters, handler durations, and affinity via versioned sysfs/procfs interface. | Counters are lock-free or bounded, non-allocating in hard-IRQ context, and rate-limited. |
| **17** | `docs(irq): publish driver guide and architecture support matrix` | Publish complete driver integration guide, context rules, and architecture evidence matrix. | All support claims reference verified QEMU/hardware test runs. |

*Initial Implementation Milestone:* Complete **PRs 1–8** and the APIC timer path in **PR 11** before broad driver adoption.

---

## Periodic Maintenance PR Cadence

- **Every PR:** `ci(irq): run architecture entry and flow checks` (Build, frame layout tests, generic flow tests).
- **Weekly:** `maint(irq): triage unhandled vectors and interrupt regressions` (Reproducer, vector source, controller, fix).
- **Weekly:** `test(irq): review storm and deferred-work failures` (Queue depth, duration, dropped work analysis).
- **Monthly:** `audit(irq): review handler registration and teardown safety` (Synchronization, tokens, teardown safety).
- **Monthly:** `test(irq): run timer, device, and spurious-IRQ matrix` (QEMU matrix across timer, device, and spurious IRQs).
- **Monthly:** `security(irq): review privilege and interrupt-storm attack surface` (DoS mitigation, rate limits).
- **Quarterly:** `test(irq): validate SMP affinity and CPU-offline behavior` (Multi-core affinity and hotplug migration).
- **Quarterly:** `docs(irq): review architecture and controller assumptions` (Vector mappings, chip operations).
- **Each Release Candidate:** `release(irq): certify interrupt support claims` (Certify test evidence across platforms).
- **After Each Incident:** `fix(irq): prevent recurrence of <lost IRQ, storm, or teardown race>` (Regression test & root cause analysis).

---

## Component-Specific Rules

- **Architecture entry stubs:** Preserve ABI register contracts, handle CPU error codes, validate stack alignment, and pass context frames to generic dispatch.
- **Interrupt descriptor table / vectors:** Keep exception vectors separate from device IRQs; validate privilege gates and reject duplicate vector bindings.
- **Controller backends:** Keep chip operations minimal; implement explicit mask/unmask/ack/EOI methods with strict MMIO memory barriers.
- **Flow handlers:** Keep edge, level, fast-EOI, and per-CPU dispatch separate; handle retriggering and pending states cleanly.
- **Device handlers:** Top halves capture device state and acknowledge the hardware; threaded bottom halves process data and handle blocking operations.
- **Exceptions:** Separate recoverable page faults and traps from unrecoverable kernel exceptions that trigger controlled diagnostics.
- **Timer & Scheduler:** Timer ISR increments timekeeping structures and signals preemption; context switching takes place at architectural return points.
- **SMP & IPIs:** Target CPU sets, memory barriers, and completion acknowledgments must be explicit; avoid synchronous waiting on target CPUs holding needed locks.
- **Deferred Work:** Use bounded queues with clear saturation behavior; never assume queued work has finished upon enqueue.
- **Diagnostics:** Maintain lock-free, bounded per-vector counters; avoid allocation or formatting in hard-IRQ context.
- **Status Reporting:** Use `Prototype` for structural/unit-tested code, `Integrated` for QEMU-verified paths, and `Supported` for hardware-validated releases.

---

## PR Template and Merge Gates

```text
## Summary
What interrupt path, correctness issue, or device capability does this PR address?

## Scope
Architecture entry | IRQ flow | controller backend | driver handler |
deferred work | affinity | diagnostics

## Interrupt contract
Trigger type, logical/hardware ID mapping, mask/ack/EOI order,
handler context, concurrency, and teardown semantics.

## Runtime evidence
Unit/model | QEMU booted | SMP QEMU | physical hardware
List architecture, controller, device, and exact test configuration.

## Safety and failure behavior
Spurious/unhandled IRQ, storm containment, queue saturation,
handler removal, CPU offline, and recovery behavior.

## Tests
Frame layout, generic flow, timer/device IRQ, EOI, teardown race,
deferred work, affinity, and anything not tested.

## Status and docs
Proposed | Prototype | Integrated | Supported
Link feature inventory, architecture matrix, driver guide, and test evidence.

## Checklist
- [ ] Architecture entry frame and privilege transitions validated
- [ ] Controller flow and ack/EOI ordering documented
- [ ] Hard handler is bounded and non-blocking
- [ ] Teardown synchronizes handlers and deferred work
- [ ] Unknown/storm IRQ behavior is bounded
- [ ] Required CI and QEMU checks pass; review discussions resolved
```

---

## Completion Criteria

- **Prototype:** Interrupt frame structures, controller traits, and generic flow logic are unit-tested in isolation, but no booted IRQ dispatch has been proven on hardware or QEMU.
- **Integrated:** Documented QEMU targets (x86_64, AArch64, or RISC-V) boot successfully, handle timer and device IRQs, execute registered handlers, issue correct EOI sequencing, and return cleanly to thread context.
- **Supported:** Automated CI test suites validate multi-arch boot, edge/level flow correctness, handler registration/teardown synchronization, spurious IRQ containment, deferred work scheduling, and SMP IRQ affinity migration across all advertised targets.
