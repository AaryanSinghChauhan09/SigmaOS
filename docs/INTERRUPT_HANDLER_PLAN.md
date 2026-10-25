# SigmaOS Interrupt Handling Master Architecture, Support Matrix, and Development Plan

SigmaOS lists x86_64 GDT/IDT and interrupt stubs, an APIC with EOI and timer/IPI support, plus AArch64 GICv3 and RISC-V PLIC components. The repository's inventory rates x86_64 interrupt work as structural and needing QEMU boot validation; AArch64 and RISC-V are compile-only pending QEMU boots. The primary objective is to connect **vector entry → generic dispatch → controller flow → registered handler → safe return**, then prove it with booted tests before claiming hardware support.

This master plan distinguishes **exceptions** (CPU-detected faults/traps), **device interrupts** (APIC/GIC/PLIC-delivered IRQs), and **deferred work** (work run later in a schedulable context). Linux's generic IRQ design separates driver registration, interrupt-flow handling, and controller-specific operations; FreeBSD's filter/threaded-handler model provides a complementary BSD reference.

---

## 5-Stage Decoupled Interrupt Pipeline Architecture

SigmaOS enforces a strict 5-stage decoupled pipeline to guarantee architectural portability, safe concurrency, and bounded context handling:

```
+-----------------------------------------------------------------------------------+
| Stage 1: Vector Entry (Architecture Assembly / Exception Frame)                   |
| - Save CPU register state into stack interrupt frame                              |
| - Distinguish hardware vector / exception error code / privilege transition       |
+-----------------------------------------------------------------------------------+
                                        |
                                        v
+-----------------------------------------------------------------------------------+
| Stage 2: Generic Dispatch (Hardware-to-Logical Mapping & Domain Lookup)           |
| - Map raw hardware vector / IRQ to logical IRQ descriptor                         |
| - Verify domain validity, check spurious / unhandled counters                     |
+-----------------------------------------------------------------------------------+
                                        |
                                        v
+-----------------------------------------------------------------------------------+
| Stage 3: Controller Flow (Edge / Level / Fast-EOI / Per-CPU Policy)               |
| - Invoke interrupt-controller chip operations (Mask, Ack, EOI)                   |
| - Enforce retrigger, pending state transitions, and storm containment             |
+-----------------------------------------------------------------------------------+
                                        |
                                        v
+-----------------------------------------------------------------------------------+
| Stage 4: Handler Invocation (Bounded Top-Half Hard-IRQ / Shared Handlers)          |
| - Execute registered ISRs in hard-IRQ context                                     |
| - Capture minimal device state, acknowledge device interrupt source, queue bottom |
+-----------------------------------------------------------------------------------+
                                        |
                                        v
+-----------------------------------------------------------------------------------+
| Stage 5: Return & Deferred Work Dispatch (SoftIRQ / Workqueue / Taskqueue / EOI)  |
| - Process softirqs and wake up threaded interrupt handlers / workqueues           |
| - Restore CPU register context, execute EOI, and return to interrupted thread      |
+-----------------------------------------------------------------------------------+
```

---

## Multi-Architecture Subsystem Audit Matrix

| Architecture | Controller / Subsystem | Implemented Paths | Test Evidence Level | Known Limitations & Next Milestone |
|--------------|------------------------|-------------------|----------------------|--------------------------------------|
| **x86_64**   | IDT / LAPIC / IOAPIC  | IDT vector stubs, LAPIC EOI/Timer, IPI stubs | Structural / Unit-Tested (`src/kernel/interrupt.rs`) | Needs QEMU boot validation for IDT entry and APIC timer dispatch |
| **AArch64**   | Exception Vectors / GICv3 | Vector table, GICC/GICD registers, distributor stubs | Compile-only (`src/arch/aarch64/`) | Lacks QEMU `virt` machine boot proof and GIC claim/EOI test |
| **RISC-V**   | Traps / PLIC / SBI    | Direct trap entry, PLIC claim/complete, SBI timer | Compile-only (`src/arch/riscv64/`) | Lacks QEMU `virt` machine boot proof and PLIC external IRQ test |

---

## Context Contracts and Rules

SigmaOS explicitly defines execution contexts, locking rules, and allocation constraints across the interrupt subsystem:

| Execution Context | Allowed Operations | Forbidden Operations | Stack / Lock Rules |
|-------------------|--------------------+----------------------+--------------------+
| **Hard-IRQ Context** | Device register read/write, ACK/EOI, enqueuing deferred work, lockless atomic counters | Blocking/sleeping, heap allocation, mutex lock, long loops, yield/reschedule | Uses dedicated per-CPU interrupt stack; spinlocks must disable local IRQs |
| **Threaded / Deferred Context** | Mutex acquisition, heap allocation, blocking I/O, device queue draining | Accessing un-synchronized hard-IRQ structures without locks | Schedulable kernel thread context; preemption enabled |
| **Exception Context** | Fault translation, process signal queuing, MMU page-fault resolution, controlled panic | Unresolved kernel page fault recovery without unwind handler | Saves full CPU frame; execution on interrupt/fault stack |
| **NMI / Critical Context** | Hardware diagnostic logging, emergency reset, memory error capture | Calling non-reentrant kernel routines, acquiring any spinlock | Execution on dedicated NMI stack; irrecoverable fault aborts |
| **Process / User Context** | Standard system call execution, blocking, allocation, mutex acquisition | Direct MMIO controller manipulation or direct interrupt ACK | Kernel stack; fully preemptible and schedulable |

---

## Rules and Invariants

1. **Separate architecture entry, IRQ flow, and controller operations.** Architecture code enters the generic layer; flow policy handles edge/level/per-CPU behavior; controller code supplies mask, acknowledge, EOI, and affinity operations. Do not combine these in one monolithic handler.
2. **Keep hard-interrupt work bounded.** A hard-IRQ handler should identify and acknowledge the source, capture minimal state, and defer work that can block or take significant time. Linux and FreeBSD both distinguish quick interrupt-context work from threaded/deferred handling.
3. **Specify edge vs level semantics.** Define when to mask, acknowledge, service, unmask, and EOI. An incorrect order can lose edge-triggered events or cause repeated level-triggered interrupts. Linux provides separate generic flow handlers for edge, level, fast-EOI, and per-CPU IRQs.
4. **Make ownership and teardown explicit.** Registration returns a stable handler token; teardown disables/masks the source, waits for in-flight handlers and deferred work, then releases state. Never free handler data while an IRQ can still reference it.
5. **Define spurious and unhandled behavior.** Unmapped, unexpected, or repeatedly unhandled IRQs must be counted and contained; they must not panic the system by default.
6. **Do not sleep in hard-IRQ context.** Enforce context rules in debug builds; use threaded handlers or work queues for blocking I/O, allocation, and long processing.
7. **Treat interrupt numbers as mapped identities.** Keep hardware IRQ numbers distinct from kernel logical IRQs; validate controller mappings and reject duplicates.
8. **SMP requires synchronization.** Specify whether a handler may run concurrently on multiple CPUs, how shared device state is protected, and what affinity means. Devices and queues must be quiesced when interrupts are disabled or moved.
9. **Test by evidence tier.** Unit/model tests validate state machines; QEMU tests validate real vector entry/controller behavior; hardware tests validate actual devices. Module tests do not establish bootability or hardware support.

---

## Proposed PR Sequence Roadmap

| PR | Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **1** | `docs(irq): audit interrupt paths and define support matrix` | Audit x86_64 IDT/APIC, AArch64 exception vectors/GICv3, RISC-V traps/PLIC, timer/IPI code, device handlers, and deferred work. | Each architecture and IRQ source has a status, source links, controller type, test level, and known limitations; compile-only paths labeled. |
| **2** | `docs(irq): define handler context and lifecycle contracts` | Define hard-IRQ, threaded/deferred, exception, NMI/critical, and process contexts; registration, masking, synchronization, removal, and error semantics. | Every API states blocking/locking restrictions, ownership, ordering, and teardown requirements; unsupported contexts rejected in debug builds. |
| **3** | `test(irq): add vector and interrupt-frame layout checks` | Add compile-time/runtime checks for IDT entries, exception/IRQ frame layouts, stack alignment, error-code handling, and register saves. | Tests verify frame construction for error-code and no-error-code vectors; invalid vector mappings rejected; tests marked structural until QEMU entry proven. |
| **4** | `feat(irq): add generic IRQ descriptors and logical mapping` | Introduce IRQ descriptors, stable logical IDs, hardware-ID mappings/domains, handler registrations, and per-IRQ state. | Duplicate mappings and invalid IDs fail safely; unregister invalidates a handler token; lookup is bounded and tested. |
| **5** | `feat(irq): define interrupt-controller chip operations` | Create controller interface for enable/disable, mask/unmask, acknowledge, EOI, trigger type, affinity, and pending-state query. | APIC/GIC/PLIC backends implement only supported operations; unsupported operations return explicit errors; register-access ordering documented. |
| **6** | `feat(irq): implement generic edge level and fasteoi flows` | Add separate flow handlers and state transitions for edge, level, fast-EOI, simple, and per-CPU sources as supported. | Simulated tests verify exact mask/ack/service/EOI sequence; pending edges not lost; level IRQs do not storm while unhandled. |
| **7** | `feat(irq): connect x86_64 IDT entry to APIC dispatch` | Wire real IDT entry stubs and APIC vector mapping to generic IRQ dispatcher; distinguish exceptions, external IRQs, and IPIs. | QEMU x86_64 test receives a timer IRQ, invokes registered handler, sends EOI exactly once, and returns with registers/stack intact. |
| **8** | `feat(irq): add handler registration and synchronized teardown` | Implement register/unregister, enable/disable nesting semantics, shared handler cookies, and in-flight synchronization. | Concurrent IRQ and teardown stress does not call freed data; unregister waits or reports in-flight work according to contract. |
| **9** | `feat(irq): add spurious and unhandled interrupt containment` | Add unhandled counters, rate-limited diagnostics, storm detection, masking/quarantine policy, and recovery/reset hooks. | Unknown or unhandled IRQs do not panic; repeated storms bounded; diagnostics identify vector/controller without unbounded logging. |
| **10** | `feat(irq): add threaded handlers and deferred-work interface` | Connect minimal worker/taskqueue support; allow short top half to acknowledge and queue a threaded handler. | Deferred work runs outside hard-IRQ context; queue saturation handled; driver teardown drains/cancels work. |
| **11** | `feat(irq): add APIC timer and IPI integration tests` | Calibrate APIC timer in QEMU; validate timer delivery, reschedule IPI, TLB shootdown IPI, and local EOI. | Timer ticks are monotonic within tolerance; IPI reaches intended CPU; no interrupt storm or lost reschedule. |
| **12** | `feat(irq): add IRQ affinity and CPU-hotplug policy` | Add affinity masks and migration/shutdown handling only after SMP scheduling and CPU lifecycle work. | IRQs target only online/allowed CPUs; queues quiesced before moving/disabling; last eligible CPU offline behavior documented. |
| **13** | `feat(irq): validate AArch64 GICv3 dispatch on QEMU` | Connect exception-vector entry, GIC acknowledge/EOI, interrupt IDs, and generic dispatch on `virt` machine. | QEMU AArch64 boots and passes timer/device IRQ tests; spurious IDs and EOI behavior covered. |
| **14** | `feat(irq): validate RISC-V PLIC dispatch on QEMU` | Connect trap entry, PLIC claim/complete, source IDs, and generic dispatch. | QEMU RISC-V boots and passes timer/external IRQ tests; invalid/zero claims safe. |
| **15** | `test(irq): add fault-injection and interrupt-storm suite` | Add simulation for missing EOI, repeated edge/level events, registration failure, queue saturation, and teardown during dispatch. | Tests detect hangs, storms, lost IRQs, use-after-free, and double-completion; each failure has a regression case. |
| **16** | `feat(irq): add interrupt statistics and trace interface` | Expose per-controller/per-CPU counts, unhandled counts, handler duration, deferred-work delay, and affinity. | Counters are bounded, concurrency-safe, and do not allocate in hard-IRQ context; trace output rate-limited. |
| **17** | `docs(irq): publish driver guide and architecture support matrix` | Document handler API, context restrictions, IRQ flow, driver registration, deferred work, diagnostics, and architecture support. | Every supported claim links to booted tests and exact QEMU/hardware configuration. |

**Immediate Milestone Focus:** PRs 1–8 plus the QEMU timer path in PR 11 form the required initial baseline before broad driver adoption.

---

## Periodic Maintenance PR Cadence

| Cadence | Task | Required Evidence |
|---------|------|-------------------|
| **Every PR** | `ci(irq): run architecture entry and flow checks` | Build, frame-layout tests, generic flow tests, and affected QEMU test pass. |
| **Weekly** | `maint(irq): triage unhandled vectors and interrupt regressions` | Reproducer, vector/source, architecture/controller, owner, impact, and fix. |
| **Weekly** | `test(irq): review storm and deferred-work failures` | Queue depth, handler duration, unhandled count, dropped/coalesced work documented. |
| **Monthly** | `audit(irq): review handler registration and teardown safety` | In-flight synchronization, ownership tokens, shared handlers, and device removal verified. |
| **Monthly** | `test(irq): run timer, device, and spurious-IRQ matrix` | QEMU test results for timer, device IRQ, unknown vector, no handler, level/edge, and EOI correctness. |
| **Monthly** | `security(irq): review privilege and interrupt-storm attack surface` | User-triggerable device paths, interrupt-rate abuse, logging bounds, and DoS controls audited. |
| **Quarterly** | `test(irq): validate SMP affinity and CPU-offline behavior` | Validate affinity, migration, quiesce, and online/offline recovery on SMP setups. |
| **Quarterly** | `docs(irq): review architecture and controller assumptions` | Recheck IDT/vector mapping, GIC/PLIC/APIC behavior, context rules, and trigger modes. |
| **Release Candidate** | `release(irq): certify interrupt support claims` | Architecture, QEMU machine, test results, limitations, and recovery behavior verified. |
| **Post-Incident** | `fix(irq): prevent recurrence of <lost IRQ, storm, or teardown race>` | Minimal reproducer, root cause, regression test, and fix backported. |

---

## Component-Specific Rules

- **Architecture entry stubs:** Save exactly the state required by the ABI and exception type; use distinct handling for CPU-pushed error codes; validate stack alignment and privilege transition.
- **Interrupt descriptor table/vectors:** Reserve exception vectors separately from device vectors; validate installation and privilege levels; reject duplicate or conflicting assignments.
- **Controller backends:** Keep chip operations small and precise; document whether acknowledge and EOI are separate; use correct MMIO ordering and barriers; never acknowledge an unrelated source.
- **Flow handlers:** Keep edge, level, fast-EOI, and per-CPU logic separate; define pending/retrigger behavior; handle shared sources only if each handler can identify its own device.
- **Device handlers:** Top half confirms the device asserted the IRQ, captures minimal state, and queues work; threaded/deferred portion drains device queues and performs blocking operations.
- **Exceptions:** Page faults, GP faults, divide errors, and breakpoints have distinct error/context semantics; recoverable faults must be resolved or translated into process exceptions.
- **Timer and scheduler:** Timer ISR updates bounded counters and requests rescheduling; actual task switching occurs only at a safe architecture return point.
- **SMP/IPIs:** Define target CPU, memory ordering, acknowledgement, timeout, and offline behavior. Do not wait synchronously for an IPI while holding resources the target handler needs.
- **Deferred work:** Define queue ordering, concurrency, cancellation, shutdown, and saturation; never assume queued work has run merely because enqueue succeeded.
- **Diagnostics:** Provide bounded per-vector counters and rate-limited logs; avoid printing or allocating from hard-IRQ context.

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

## Completion Criteria Tiers

- **Prototype:** Interrupt layouts, controllers, or flow logic are modeled/unit-tested, but no booted IRQ is proven.
- **Integrated:** A documented QEMU target receives a real timer and device interrupt, dispatches it to a registered handler, acknowledges/EOIs correctly, and returns safely.
- **Supported:** Repeatable tests cover all claimed architectures, trigger modes, handler lifecycle, spurious/storm behavior, deferred work, and SMP affinity if advertised.
