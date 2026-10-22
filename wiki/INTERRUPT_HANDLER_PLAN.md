# SigmaOS Interrupt Handling Master Architecture & Support Matrix Plan

SigmaOS lists x86_64 GDT/IDT and interrupt stubs, an APIC with EOI and timer/IPI support, plus AArch64 GICv3 and RISC-V PLIC components. But the repository’s own inventory rates x86_64 interrupt work as structural and needing QEMU boot validation; AArch64 and RISC-V are compile-only pending QEMU boots. The immediate work is therefore to connect **vector entry → generic dispatch → controller flow → registered handler → safe return**, then prove it with booted tests before claiming hardware support.

This plan distinguishes **exceptions** (CPU-detected faults/traps), **device interrupts** (APIC/GIC/PLIC-delivered IRQs), and **deferred work** (work run later in a schedulable context). Linux’s generic IRQ design separates driver registration, interrupt-flow handling, and controller-specific operations; FreeBSD’s filter/threaded-handler model provides a complementary BSD reference.

## Rules and invariants

- **Separate architecture entry, IRQ flow, and controller operations.** Architecture code enters the generic layer; flow policy handles edge/level/per-CPU behavior; controller code supplies mask, acknowledge, EOI, and affinity operations. Do not combine these in one monolithic handler.
- **Keep hard-interrupt work bounded.** A hard-IRQ handler should identify and acknowledge the source, capture minimal state, and defer work that can block or take significant time. Linux and FreeBSD both distinguish quick interrupt-context work from threaded/deferred handling.
- **Specify edge vs level semantics.** Define when to mask, acknowledge, service, unmask, and EOI. An incorrect order can lose edge-triggered events or cause repeated level-triggered interrupts. Linux provides separate generic flow handlers for edge, level, fast-EOI, and per-CPU IRQs.
- **Make ownership and teardown explicit.** Registration returns a stable handler token; teardown disables/masks the source, waits for in-flight handlers and deferred work, then releases state. Never free handler data while an IRQ can still reference it. Linux exposes `free_irq` and synchronization APIs for this class of lifecycle.
- **Define spurious and unhandled behavior.** Unmapped, unexpected, or repeatedly unhandled IRQs must be counted and contained; they must not panic the system by default.
- **Do not sleep in hard-IRQ context.** Enforce context rules in debug builds; use threaded handlers or work queues for blocking I/O, allocation, and long processing.
- **Treat interrupt numbers as mapped identities.** Keep hardware IRQ numbers distinct from kernel logical IRQs; validate controller mappings and reject duplicates. Linux’s `irq_domain` maps hardware IRQs to logical IRQ numbers.
- **SMP requires synchronization.** Specify whether a handler may run concurrently on multiple CPUs, how shared device state is protected, and what affinity means. Linux supports IRQ affinity and CPU-offline migration, but devices/queues must be quiesced when interrupts are disabled or moved.
- **Test by evidence tier.** Unit/model tests validate state machines; QEMU tests validate real vector entry/controller behavior; hardware tests validate actual devices. SigmaOS explicitly warns that module tests do not establish bootability or hardware support.

## Proposed PR sequence

| PR | Proposed title | Scope | Acceptance criteria |
|---|---|---|---|
| 1 | `docs(irq): audit interrupt paths and define support matrix` | Audit x86_64 IDT/APIC, AArch64 exception vectors/GICv3, RISC-V traps/PLIC, timer/IPI code, device handlers, and deferred work. Document implemented paths and test evidence. | Each architecture and IRQ source has a status, source links, controller type, test level, and known limitations; compile-only paths are clearly labeled. |
| 2 | `docs(irq): define handler context and lifecycle contracts` | Define hard-IRQ, threaded/deferred, exception, NMI/critical, and process contexts; registration, masking, synchronization, removal, shared IRQ, and error semantics. | Every API states blocking/locking restrictions, ownership, ordering, and teardown requirements; unsupported contexts are rejected in debug builds. |
| 3 | `test(irq): add vector and interrupt-frame layout checks` | Add compile-time/runtime checks for IDT entries, exception/IRQ frame layouts, stack alignment, error-code handling, and architecture-specific register saves. | Tests verify frame construction for error-code and no-error-code vectors; invalid vector mappings are rejected; tests are marked structural until QEMU entry is proven. |
| 4 | `feat(irq): add generic IRQ descriptors and logical mapping` | Introduce IRQ descriptors, stable logical IDs, hardware-ID mappings/domains, handler registrations, and per-IRQ state. | Duplicate mappings and invalid IDs fail safely; unregister invalidates a handler token; lookup is bounded and tested. Linux `irq_domain` provides a reference for hardware-to-logical IRQ translation. |
| 5 | `feat(irq): define interrupt-controller chip operations` | Create controller interface for enable/disable, mask/unmask, acknowledge, EOI, trigger type, affinity, and pending-state query. | APIC/GIC/PLIC backends implement only supported operations; unsupported operations return explicit errors; register-access ordering is documented. |
| 6 | `feat(irq): implement generic edge level and fasteoi flows` | Add separate flow handlers and state transitions for edge, level, fast-EOI, simple, and per-CPU sources as supported. | Simulated tests verify exact mask/ack/service/EOI sequence; pending edges are not lost; level IRQs do not storm while unhandled. Linux’s generic IRQ layer uses these distinct flow types. |
| 7 | `feat(irq): connect x86_64 IDT entry to APIC dispatch` | Wire real IDT entry stubs and APIC vector mapping to the generic IRQ dispatcher; distinguish exceptions, external IRQs, and IPIs. | QEMU x86_64 test receives a timer IRQ, invokes the registered handler, sends EOI exactly once, and returns with registers/stack intact. The inventory currently lists x86_64 interrupt structures as needing QEMU validation. |
| 8 | `feat(irq): add handler registration and synchronized teardown` | Implement register/unregister, enable/disable nesting semantics, shared handler cookies if supported, and in-flight synchronization. | Concurrent IRQ and teardown stress does not call freed data; unregister waits or reports in-flight work according to contract; repeated enable/disable is balanced. |
| 9 | `feat(irq): add spurious and unhandled interrupt containment` | Add unhandled counters, rate-limited diagnostics, storm detection, masking/quarantine policy, and recovery/reset hooks. | Unknown or unhandled IRQs do not panic; repeated storms are bounded; diagnostics identify vector/controller without unbounded logging. |
| 10 | `feat(irq): add threaded handlers and deferred-work interface` | Connect minimal worker/taskqueue support; allow a short top half to acknowledge and queue a threaded handler. | Deferred work runs outside hard-IRQ context; queue saturation is handled; driver teardown drains/cancels work; no interrupt source is unmasked before device state is safe. Linux threaded IRQs and FreeBSD threaded handlers/taskqueues provide references. |
| 11 | `feat(irq): add APIC timer and IPI integration tests` | Calibrate APIC timer in QEMU; validate timer delivery, reschedule IPI, TLB shootdown IPI, and local EOI. | Timer ticks are monotonic within tolerance; IPI reaches intended CPU; no interrupt storm or lost reschedule; dispatcher is invoked only at safe points. APIC calibration is a stated inventory milestone. |
| 12 | `feat(irq): add IRQ affinity and CPU-hotplug policy` | Add affinity masks and migration/shutdown handling only after SMP scheduling and CPU lifecycle work. | IRQs target only online/allowed CPUs; queues are quiesced before moving/disabling; last eligible CPU offline behavior is documented. Linux’s managed-affinity model requires queue quiescence when disabling/migrating. |
| 13 | `feat(irq): validate AArch64 GICv3 dispatch on QEMU` | Connect exception-vector entry, GIC acknowledge/EOI, interrupt IDs, and generic dispatch on `virt` machine. | QEMU AArch64 boots and passes timer/device IRQ tests; spurious IDs and EOI behavior are covered; status remains architecture-specific. The inventory currently marks AArch64 boot validation as outstanding. |
| 14 | `feat(irq): validate RISC-V PLIC dispatch on QEMU` | Connect trap entry, PLIC claim/complete, source IDs, and generic dispatch. | QEMU RISC-V boots and passes timer/external IRQ tests; invalid/zero claims are safe; status remains architecture-specific. The inventory currently marks RISC-V boot validation as outstanding. |
| 15 | `test(irq): add fault-injection and interrupt-storm suite` | Add simulation for missing EOI, repeated edge/level events, registration failure, queue saturation, teardown during dispatch, and affinity changes. | Tests detect hangs, storms, lost IRQs, use-after-free, and double-completion; each failure has a regression case. |
| 16 | `feat(irq): add interrupt statistics and trace interface` | Expose per-controller/per-CPU counts, unhandled counts, handler duration, deferred-work delay, and affinity through a read-only versioned interface. | Counters are bounded, concurrency-safe, and do not allocate in hard-IRQ context; trace output is rate-limited and excludes sensitive data. |
| 17 | `docs(irq): publish driver guide and architecture support matrix` | Document handler API, context restrictions, IRQ flow, driver registration, deferred work, diagnostics, and architecture/device support. | Every supported claim links to booted tests and exact QEMU/hardware configuration; unsupported MSI/MSI-X, hotplug, NMI, or affinity features are explicit. |

**Priority order:** complete PRs **1–8** plus the QEMU timer path in PR 11 before broad driver adoption. SigmaOS’s inventory already identifies APIC/GIC/PLIC components but lacks booted validation for all architectures; integration and proof are more urgent than adding additional IRQ types.

## Periodic maintenance PR cadence

| Cadence | Proposed recurring PR/task | Required evidence |
|---|---|---|
| Every PR | `ci(irq): run architecture entry and flow checks` | Build, frame-layout tests, generic flow tests, and affected QEMU test; skipped architecture tests explained. |
| Weekly | `maint(irq): triage unhandled vectors and interrupt regressions` | Reproducer, vector/source, architecture/controller, owner, impact, and fix or tracked issue. |
| Weekly | `test(irq): review storm and deferred-work failures` | Queue depth, handler duration, unhandled count, dropped/coalesced work, and recovery behavior from fixed test workloads. |
| Monthly | `audit(irq): review handler registration and teardown safety` | In-flight synchronization, ownership tokens, shared handlers, device removal, worker drain, and lifetime rules reviewed. |
| Monthly | `test(irq): run timer, device, and spurious-IRQ matrix` | QEMU test results for timer, one device IRQ, unknown vector, no handler, repeated level/edge, and EOI correctness. |
| Monthly | `security(irq): review privilege and interrupt-storm attack surface` | User-triggerable device paths, interrupt-rate abuse, untrusted descriptors, logging bounds, and denial-of-service controls. |
| Quarterly | `test(irq): validate SMP affinity and CPU-offline behavior` | Run only when SMP/CPU hotplug exists; validate affinity, migration, quiesce, and online/offline recovery. |
| Quarterly | `docs(irq): review architecture and controller assumptions` | Recheck IDT/vector mapping, GIC/PLIC/APIC behavior, interrupt-context rules, and unsupported trigger modes. |
| Each release candidate | `release(irq): certify interrupt support claims` | Exact architecture, QEMU machine/controller, IRQ test results, known limitations, and recovery behavior. |
| After each incident | `fix(irq): prevent recurrence of <lost IRQ, storm, or teardown race>` | Minimal reproducer, root cause, regression test, affected hardware/configuration, and backport decision. |

These are review tasks or candidate PRs, not auto-merges. Linux’s KUnit and kselftest split is a useful test-design reference: unit-level flow tests and booted userspace/system tests establish different kinds of evidence.

## Component-specific rules

- **Architecture entry stubs:** Save exactly the state required by the ABI and exception type; use distinct handling for CPU-pushed error codes; validate stack alignment and privilege transition; never assume all vectors have identical frames.
- **Interrupt descriptor table/vectors:** Reserve exception vectors separately from device vectors; validate installation and privilege levels; reject duplicate or conflicting assignments.
- **Controller backends:** Keep chip operations small and precise; document whether acknowledge and EOI are separate; use correct MMIO ordering and barriers; never acknowledge an unrelated source.
- **Flow handlers:** Keep edge, level, fast-EOI, and per-CPU logic separate; define pending/retrigger behavior; handle shared sources only if each handler can identify its own device.
- **Device handlers:** Top half confirms the device asserted the IRQ, captures minimal state, and queues work; threaded/deferred portion drains device queues and performs blocking operations. Shared IRQ registration needs an explicit per-device cookie and teardown-safe ownership.
- **Exceptions:** Page faults, GP faults, divide errors, and breakpoints have distinct error/context semantics; recoverable faults must be resolved or translated into process exceptions, while fatal kernel faults use controlled panic diagnostics.
- **Timer and scheduler:** Timer ISR updates bounded counters and requests rescheduling; actual task switching occurs only at a safe architecture return point. Keep timekeeping separate from scheduler policy.
- **SMP/IPIs:** Define target CPU, memory ordering, acknowledgement, timeout, and offline behavior. Do not wait synchronously for an IPI while holding resources the target handler needs.
- **Deferred work:** Define queue ordering, concurrency, cancellation, shutdown, and saturation; never assume queued work has run merely because enqueue succeeded. Linux workqueues and FreeBSD taskqueues are useful references.
- **Diagnostics:** Provide bounded per-vector counters and rate-limited logs; avoid printing or allocating from hard-IRQ context.
- **Status reporting:** Use Prototype for models/structural tests, Integrated for actual QEMU dispatch, and Supported only for repeatable tests on documented configurations.

## PR template and merge gates

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

Require one maintainer review for isolated tests/docs and two qualified reviews for architecture entry assembly, controller backends, SMP affinity, or driver teardown. Protect the default branch with required PR reviews and passing checks.

## Completion criteria

- **Prototype:** interrupt layouts, controllers, or flow logic are modeled/unit-tested, but no booted IRQ is proven.
- **Integrated:** a documented QEMU target receives a real timer and device interrupt, dispatches it to a registered handler, acknowledges/EOIs correctly, and returns safely.
- **Supported:** repeatable tests cover all claimed architectures, trigger modes, handler lifecycle, spurious/storm behavior, deferred work, and SMP affinity if advertised.

The immediate gate is **x86_64 QEMU validation of IDT/APIC entry and timer dispatch**, followed by a small device IRQ path. Do not mark AArch64 GICv3 or RISC-V PLIC as supported until each boots and passes its own runtime matrix; the feature inventory currently says those paths still need QEMU validation.
