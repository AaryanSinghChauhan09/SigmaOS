# SigmaOS Device Driver Development and Maintenance Plan

This document outlines the official architectural roadmap, driver lifecycle rules, proposed Pull Request (PR) sequence, maintenance cadences, component-specific rules, merge gates, and execution criteria for the SigmaOS device driver subsystem.

## Overview and Status Vocabulary

SigmaOS is an experimental Rust operating system. The existence of a driver module or passing unit tests in isolated modules does not constitute proof of runtime or hardware support.

To maintain engineering rigor and transparency, all driver capabilities must adhere to the standard SigmaOS status vocabulary:
- **Proposed:** Specification or plan drafted, but implementation has not started.
- **Prototype:** Model, stub, or module-level implementation without full bus or OS runtime path integration.
- **Integrated:** Code is wired into PCI/bus discovery, uses real DMA/IRQ abstractions, and is verified end-to-end in QEMU.
- **Supported:** Validated through repeatable, documented test matrices on exact physical or QEMU hardware with known limitations and rollback paths documented.

The initial strategy prioritizes driver infrastructure and virtual devices (VirtIO block and VirtIO network in QEMU) before broad physical hardware support claims.

---

## Driver Rules for Every PR

1. **Separate device discovery from driver logic:** Define a bus/device model with typed IDs and resource descriptions. Bus enumeration matches devices to drivers; leaf drivers do not scan raw hardware independently.
2. **Explicit probe/attach and remove/detach lifecycles:** Probe must return resources on failure in reverse allocation order. Detach quiesces the device and releases all owned resources.
3. **Platform DMA API isolation:** CPU virtual, CPU physical, and device DMA addresses are distinct types. Always use mapping/unmapping, direction, address-width masks, cache sync, and error validation.
4. **Strict interrupt context rules:** Handlers must be short, non-blocking, and limited to safe interrupt-context operations. Defer lengthy or sleeping work to a worker/task mechanism.
5. **Concurrency and memory ordering discipline:** Document descriptor ownership, locking, barriers, and ordering for every queue and MMIO access. Posted writes must be flushed where required.
6. **Quiesce before freeing:** Disable device interrupts, stop DMA engines, drain/cancel work, and synchronize with in-flight handlers before freeing memory or unmapping BARs.
7. **Safe Rust boundaries:** Keep volatile MMIO, raw pointers, DMA, and assembly behind narrow unsafe abstractions with safety contracts; leaf drivers use safe subsystem APIs.
8. **Test the real path:** Distinguish unit/mock tests from QEMU integration and physical hardware tests.
9. **No broad support claims from single tests:** Document exact device IDs, firmware versions, bus topologies, architecture, and QEMU machine configurations.
10. **Secure contribution controls:** Require review approvals and passing status checks for all driver/DMA/IRQ changes on protected branches.

---

## Proposed PR Sequence

| PR | Proposed Title | Main Work | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(drivers): audit driver status and define support evidence` | Reconcile driver source, tests, component wiki, `FEATURE_STATUS.toml`, and advertised hardware into a per-driver evidence table. | Every driver is explicitly Proposed, Prototype, Integrated, or Supported; compile/unit/QEMU/hardware evidence are separated. |
| 2 | `feat(drivers): define device, bus, and driver lifecycle interfaces` | Add typed device descriptors, bus enumeration interfaces, ID matching, driver registration, per-device state, and probe/remove callback semantics. | Mock bus enumerates, matches, probes, rejects, and removes devices; duplicate ownership and probe failure cleanup tested. |
| 3 | `feat(drivers): add managed resource acquisition and rollback` | Create RAII resource guards for MMIO mappings, IRQ registrations, DMA mappings, queues, and subsystem registrations. | Failure at any probe stage releases every acquired resource in reverse order with zero leaks or orphan callbacks. |
| 4 | `feat(mmio): add validated register access and ordering APIs` | Add typed MMIO mapping and width-aware volatile reads/writes; define alignment, bounds, endianness, and posted-write flush behavior. | Drivers cannot access MMIO without an owned mapping; out-of-bounds accesses fail safely; memory barriers documented. |
| 5 | `feat(dma): add device DMA mapping API with masks and ownership` | Add coherent and streaming mapping abstractions, address-width masks, directions, sync operations, and mapping failure handling. | CPU virtual, CPU physical, and DMA addresses are distinct types; map/unmap/sync pairing is enforced and tested. |
| 6 | `feat(irq): define interrupt registration and deferred work contract` | Add IRQ registration/removal, handler context rules, safe shared-device dispatching, and a deferred task queue. | Handlers cannot sleep or allocate unbounded memory; detach synchronizes with in-flight handlers; teardown races tested. |
| 7 | `feat(pci): add QEMU PCI enumeration and resource ownership` | Enumerate PCI config space, validate IDs/BARs, allocate resource ranges, and expose a device tree for driver matching. | PCI devices enumerated once; malformed BARs rejected safely; overlapping BAR ownership prevented. |
| 8 | `feat(drivers): integrate VirtIO block through PCI discovery` | Connect VirtIO block driver to PCI discovery, feature negotiation, queue setup, DMA, interrupts, and VFS block interface. | QEMU identifies VirtIO block, performs read/write/flush on test sectors, handles device reset, and shuts down cleanly. |
| 9 | `feat(drivers): connect VirtIO network queues to Ethernet stack` | Integrate partial VirtIO network driver with interface lifecycle, TX/RX buffers, DMA, and Ethernet receive/transmit paths. | QEMU NIC sends and receives frames through the real network stack; queue reset, malformed descriptors, and link state tested. |
| 10 | `feat(drivers): add device diagnostics and health reporting` | Add per-device identity, bound driver, state, last error, counters, and diagnostic snapshots; add safe reset/recovery hooks. | Diagnostics are read-only and redact secrets; failed devices report actionable status codes; recovery is opt-in and tested. |
| 11 | `feat(drivers): implement suspend resume and shutdown contracts` | Define power-state transitions and driver callbacks; stop DMA and interrupts before suspend/reboot; restore state on resume. | Power state sequence, partial failure rollback, and shutdown quiescing tested in QEMU/mocks. |
| 12 | `test(drivers): add fault-injection and lifecycle test harness` | Add alloc/map/IRQ/timeout/reset failure injection; test probe rollback, disconnect during I/O, and repeated bind/unbind. | Every failure point has a regression test; resource counts return to baseline; no hangs, leaks, or UAFs. |
| 13 | `test(drivers): add QEMU hardware matrix and boot smoke checks` | Automate supported QEMU machines and devices for PCI, VirtIO, serial, storage, and input; preserve logs and artifacts. | CI verifies discovery, bind, basic I/O, error path, and teardown on named configurations without external dependencies. |
| 14 | `security(drivers): add driver threat model and DMA isolation policy` | Document untrusted device assumptions, DMA confinement/IOMMU plans, firmware policy, privileged register access, and attack surfaces. | Threat model states trust assumptions; DMA cannot target arbitrary memory by default where IOMMU policy exists. |
| 15 | `docs(drivers): publish device support matrix and recovery guide` | Document known hardware and QEMU devices, firmware requirements, validated features, known issues, and recovery steps. | "Supported" entries link to repeatable evidence and exact configurations; limitations visible to users. |

---

## Recommended Implementation Order

1. **Foundations (PRs 1–7):** Audit, device/bus lifecycle, managed resource cleanup, MMIO ordering, DMA API, IRQ contract, PCI discovery.
2. **First Vertical Slice (PR 8):** VirtIO block storage through PCI, DMA, IRQ, and VFS.
3. **Second Vertical Slice (PR 9):** VirtIO network through PCI, DMA, IRQ, and Ethernet stack.
4. **Diagnostics and Power (PRs 10–11):** Health reporting, recovery, suspend/resume, and shutdown.
5. **Hardening and Evidence (PRs 12–15):** Fault injection, QEMU matrix CI, threat model, published support matrix.

---

## Periodic Maintenance Cadence

| Cadence | Task / PR Title | Required Evidence |
|---|---|---|
| Every PR | CI Check Suite | Formatting, compilation, unit tests, targeted integration/DMA/IRQ tests. |
| Weekly | `chore(drivers): triage hardware and QEMU regressions` | Failures tied to issue, reproducer, and owner; stale "supported" claims updated. |
| Weekly | `test(drivers): review fuzz and malformed-device findings` | Descriptor/MMIO parser crashes fixed and converted to regression tests. |
| Monthly | `audit(drivers): verify probe rollback and detach cleanup` | Reverse-order cleanup verified; fault-injection results recorded. |
| Monthly | `audit(dma): check mapping ownership and synchronization` | Every mapping has direction, error handling, paired unmap/sync, and barriers. |
| Monthly | `test(drivers): run QEMU discovery and basic I/O matrix` | Retained boot logs, data integrity checks, and teardown logs across named targets. |
| Quarterly | `docs(drivers): review device IDs, firmware, and compatibility` | Device ID tables, firmware versions, and architecture scope revalidated. |
| Quarterly | `security(drivers): review DMA, MMIO, and device trust boundaries` | Threat model updated; IOMMU/PMP address bounds verified. |
| Release Candidate | `release(drivers): certify supported hardware matrix` | Release test matrix passed on documented QEMU/physical hardware configs. |
| Post-Incident | `fix(drivers): prevent recurrence of <incident>` | Reproducer fails before fix, passes after; impact/mitigation documented safely. |

---

## Component-Specific Rules

- **PCI and Bus Enumeration:** Validate IDs and BAR sizes; reserve resources before mapping; prevent overlapping MMIO; handle absent devices gracefully.
- **ACPI and Firmware Discovery:** Separate firmware-described devices from bus-discoverable devices; validate table checksums and resource ranges.
- **DMA and Queues:** Enforce descriptor ownership transitions, alignment, length limits, address masks, directions, barriers, and error recovery.
- **Interrupt Handlers:** Acknowledge IRQ source promptly, avoid blocking/sleeping, defer work to worker threads, and synchronize handlers before detach.
- **Storage:** Validate command fields and completion lengths; bound retries and timeouts; test flush, reset, and partial I/O data integrity.
- **Networking:** Validate packet lengths before passing to network stack; ensure TX/RX descriptor ownership and link lifecycle are tested.
- **USB, Input, Audio, Display:** Keep core bus responsibilities separate from leaf drivers; use subsystem APIs instead of custom userland interfaces.
- **Power and Hotplug:** Quiesce I/O, stop DMA and IRQs, drain work queues, and release resources in deterministic order during suspend/shutdown.
- **Firmware:** Explicitly check firmware source, version compatibility, integrity/authenticity, and license terms; fail safely on incompatible images.
- **Rust Safety:** Restrict `unsafe` to audited MMIO/DMA/IRQ abstractions; leaf drivers use safe subsystem APIs.
- **Diagnostics:** Provide identity, bound driver, state, and actionable error codes; rate-limit logs and redact sensitive memory.

---

## PR Review Template and Merge Gates

```text
## Summary
What device capability, bug, or integration gap does this PR address?

## Device and scope
Bus, vendor/device or compatible IDs, architecture, subsystem, and exact operations.

## Lifecycle
Probe/attach steps, resources acquired, failure rollback, remove/detach, reset,
suspend/resume, and shutdown behavior.

## DMA and concurrency
DMA mask/direction, buffer ownership, sync/barriers, interrupt context,
locking, and deferred work.

## Security and failure handling
Untrusted inputs, MMIO bounds, timeouts, retry limits, firmware assumptions,
DMA isolation, and recovery behavior.

## Tests and evidence
Unit/mock, fault injection, QEMU, and physical-device tests.
List exact device/machine/firmware configuration and anything not tested.

## Status and documentation
Proposed | Prototype | Integrated | Supported
Link updated support matrix, component page, and test evidence.

## Checklist
- [ ] Probe failure releases every acquired resource
- [ ] Detach quiesces IRQs and DMA before freeing memory
- [ ] DMA addresses are distinct from CPU addresses
- [ ] Interrupt handler is bounded and non-blocking
- [ ] Malformed device input and timeout paths are tested
- [ ] Documentation and maturity status match evidence
- [ ] Required CI passes; review conversations are resolved
```

---

## Exit Criteria

- **Integrated Status:** Requires a documented boot target (e.g. QEMU `virt` or `pc-q35`) that enumerates and binds the device, exercises its real subsystem path through DMA/IRQ, and handles at least one device error or reset safely.
- **Supported Status:** Requires repeatable test matrix runs across operations, lifecycle/teardown, error recovery, fault injection, and documented hardware/firmware scope.
