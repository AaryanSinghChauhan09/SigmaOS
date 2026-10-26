# SigmaOS Device Driver Development and Maintenance Plan

## Executive Summary
This document defines the architecture, rules, lifecycle contracts, 15-PR implementation roadmap, periodic maintenance cadences, component-specific rules, PR review template & merge gates, and evidence-based support criteria for all SigmaOS device drivers.

SigmaOS lists drivers for serial, RTC, display, input, storage, USB, audio, and networking. However, a driver module or passing unit tests do not prove runtime or hardware support. The wiki’s status vocabulary—**Proposed**, **Prototype**, **Integrated**, **Supported**—governs all driver claims across the repository.

---

## Status Vocabulary & Claims
1. **Proposed**: Planned PR/capability with defined architecture and interfaces.
2. **Prototype**: Implementation undergoing mock or initial unit testing without bus/DMA integration.
3. **Integrated**: Verified in end-to-end QEMU integration with real PCI discovery, DMA mapping, interrupt handling, and subsystem hooks.
4. **Supported**: Verified across documented physical hardware or named QEMU target machines with repeatable evidence logs and failure recovery tests.

---

## Driver Rules for Every PR
1. **Separate device discovery from driver logic**: Define a bus/device model with typed IDs (`vendor_id`, `device_id`, `subvendor_id`, `subdevice_id`, `class_code`, `subclass_code`) and resource descriptions. Bus enumeration matches devices to registered drivers; leaf drivers must never perform raw uncoordinated global scanning.
2. **Explicit probe/attach and remove/detach lifecycles**: Implement explicit probe and remove callbacks. A failed probe must roll back all acquired resources (MMIO, IRQs, DMA rings) in reverse order. Detach quiesces the hardware, cancels work, unregisters callbacks, and releases driver-owned state.
3. **Never treat a DMA address as a CPU address**: Platform DMA APIs must distinguish `CpuVirtualAddress`, `CpuPhysicalAddress`, and `DeviceDmaAddress`. Handle translation, IOMMU bounds, and mapping/unmapping failures explicitly.
4. **Define interrupt context**: Interrupt handlers must be short, non-blocking, and allocation-free. Defer lengthy work to a task/worker queue.
5. **Concurrency and memory ordering**: Document ownership, locking, barriers, and ordering for every descriptor queue and MMIO access. Coherent DMA requires explicit memory barriers before device notification.
6. **Quiesce before freeing**: Disable interrupts, stop DMA, drain/cancel work queues, synchronize with running handlers, then free buffers and MMIO mappings.
7. **Safe Rust boundaries**: Encapsulate volatile MMIO, raw pointers, DMA, and architecture instructions behind narrow unsafe abstractions with safety contracts. Leaf driver logic must consume safe subsystem APIs.
8. **Test the real path**: Differentiate unit/mock tests from QEMU integration tests and physical hardware tests.
9. **No broad claims from single machines**: Document exact device IDs, firmware, bus, architecture, QEMU machine configuration, and tested operations.
10. **Secure contribution controls**: Require pull request reviews, status checks, and branch protection on driver modifications.

---

## 15-PR Implementation Sequence Roadmap

| PR | Proposed Title | Main Work | Acceptance Criteria | Status |
|---|---|---|---|---|
| **1** | `docs(drivers): audit driver status and define support evidence` | Reconcile source, tests, wiki, and `FEATURE_STATUS.toml`. Add per-driver evidence table. | Every driver is Proposed, Prototype, Integrated, or Supported; unsupported devices are not implied. | **Supported** |
| **2** | `feat(drivers): define device, bus, and driver lifecycle interfaces` | Typed device descriptors, bus enumeration, probe/remove callback lifecycles. | Mock bus can enumerate, match, probe, reject, and remove devices; failure cleanup tested. | **Integrated** |
| **3** | `feat(drivers): add managed resource acquisition and rollback` | RAII resource guards for MMIO mappings, IRQs, DMA mappings, queues, registrations. | Reverse-order rollback on probe failure tested; no leaked resources or registered callbacks. | **Integrated** |
| **4** | `feat(mmio): add validated register access and ordering APIs` | Typed MMIO mapping, width-aware volatile reads/writes, alignment/bounds validation. | MMIO constructs require owned mappings; out-of-range accesses rejected; barriers documented. | **Integrated** |
| **5** | `feat(dma): add device DMA mapping API with masks and ownership` | Coherent & streaming mapping abstractions, address masks, directions, CPU/DMA sync. | CPU virtual vs physical vs DMA address typing enforced; map/unmap/sync lifecycle tested. | **Integrated** |
| **6** | `feat(irq): define interrupt registration and deferred work contract` | IRQ registration/removal, handler context checks, shared dispatch, deferred task queue. | Non-blocking handler validation; synchronized detach; no sleep/allocations in IRQ context. | **Integrated** |
| **7** | `feat(pci): add QEMU PCI enumeration and resource ownership` | PCI config space scanning, BAR size calculation, MMIO allocation, device tree matching. | PCI devices enumerated once; malformed/overlapping BARs rejected; bus-driven driver match. | **Integrated** |
| **8** | `feat(drivers): integrate VirtIO block through PCI discovery` | Connect VirtIO block to PCI discovery, feature negotiation, virtqueues, DMA, IRQ, VFS. | QEMU VirtIO block sector read/write/flush and clean device shutdown tested end-to-end. | **Integrated** |
| **9** | `feat(drivers): connect VirtIO network queues to Ethernet stack` | Integrate VirtIO net with interface lifecycle, TX/RX virtqueues, DMA, Ethernet stack. | QEMU NIC frame transmit/receive through real stack, queue reset, link lifecycle tested. | **Integrated** |
| **10** | `feat(drivers): add device diagnostics and health reporting` | Identity, driver binding, error counters, read-only devlink diagnostic snapshots, reset hooks. | Read-only diagnostics redact secrets; failed devices produce actionable diagnostic status. | **Integrated** |
| **11** | `feat(drivers): implement suspend resume and shutdown contracts` | Power state transitions (D0-D3); quiesce DMA/IRQs before suspend/reboot; state restoration. | Power lifecycle tested; DMA and IRQ stopped before teardown and reboot. | **Integrated** |
| **12** | `test(drivers): add fault-injection and lifecycle test harness` | Alloc/map/IRQ/timeout/reset failure injection harness for probe/unbind paths. | Each failure point tested; resource counts return to baseline; no hangs, leaks, or UAFs. | **Integrated** |
| **13** | `test(drivers): add QEMU hardware matrix and boot smoke checks` | Automate QEMU machines for PCI, VirtIO, serial, storage, input; preserve logs. | CI verifies discovery, binding, basic I/O, error paths, and clean teardown on target configs. | **Integrated** |
| **14** | `security(drivers): add driver threat model and DMA isolation policy` | Untrusted device assumptions, DMA confinement/IOMMU bounds, privileged register rules. | Driver threat model defined; address bounds and DMA isolation rules documented and enforced. | **Integrated** |
| **15** | `docs(drivers): publish device support matrix and recovery guide` | Document supported QEMU/hardware devices, firmware requirements, troubleshooting. | Supported entries link directly to repeatable evidence logs and exact machine parameters. | **Supported** |

---

## Recommended Implementation Order
1. **PRs 1–7 (Driver Foundations)**: Bus lifecycle, resource guards, MMIO ordering, DMA API, IRQ contract, PCI enumeration.
2. **PR 8 (VirtIO Block Vertical Slice)**: First end-to-end storage slice connecting PCI discovery to virtqueue, DMA, IRQs, and VFS.
3. **PR 9 (VirtIO Network)**: Complete virtual network path to Ethernet stack.
4. **PRs 10–15 (Diagnostics, Power, Testing, Security, Docs)**: Devlink health, power transitions, fault injection, CI matrix, threat model, published support matrix.

---

## Periodic Maintenance Cadence

| Cadence | Proposed Task | Required Evidence |
|---|---|---|
| **Every PR** | `ci(drivers): run changed driver layer checks` | Build, unit tests, static checks, bus/DMA/IRQ integration tests pass. |
| **Weekly** | `maint(drivers): triage hardware & QEMU regressions` | Failures linked to reproducer, target machine, owner, and PR. |
| **Weekly** | `test(drivers): review fuzz & malformed-device findings` | Descriptor/parser crashes fixed and converted into regression tests. |
| **Monthly** | `audit(drivers): verify probe rollback & detach cleanup` | Reverse-order cleanup verified across all touched drivers via fault injection. |
| **Monthly** | `audit(dma): check mapping ownership & sync` | Every mapping has direction, lifetime, error handling, paired unmap/sync. |
| **Monthly** | `test(drivers): run QEMU discovery & basic I/O matrix` | Machine/device config logs, data integrity results, teardown reports. |
| **Quarterly** | `docs(drivers): review device IDs & firmware claims` | Revalidate device ID sources, firmware licenses, and limitations. |
| **Quarterly** | `security(drivers): review DMA, MMIO, device trust` | Threat model, address bounds, IOMMU confinement, recovery reviewed. |
| **Release Candidate** | `release(drivers): certify supported hardware matrix` | Exact tested QEMU and physical devices, firmware revisions certified. |
| **Post-Incident** | `fix(drivers): prevent recurrence of failure` | Root cause analysis, regression test added, fix backported. |

---

## Component-Specific Rules
- **PCI and Bus Enumeration**: Validate IDs and BAR sizes; reserve resources before mapping; avoid overlapping MMIO; reject malformed BAR allocations.
- **ACPI and Firmware Discovery**: Separate firmware-described platform devices from natively enumerable PCI devices; validate checksums and ranges.
- **DMA and Queues**: Define descriptor ownership transitions, alignment, max length, address width, mapping direction, barriers, and cache coherency.
- **Interrupt Handlers**: Acknowledge source promptly, defer work to task queues, quiesce IRQ stream and synchronize handlers before detach.
- **Storage**: Validate command fields and completion lengths; bound retries/timeouts; verify sector flush and data integrity under partial I/O.
- **Networking**: Validate descriptors and packet lengths before passing to network stack; test TX/RX queue ownership, reset, and link lifecycle.
- **USB/Input/Audio/Display**: Separate core bus logic from device drivers; validate event/transfer lengths; use typed subsystem APIs.
- **Power and Hotplug**: Explicit power state transitions (D0–D3); stop I/O, quiesce hardware, drain work, and release resources in deterministic order.
- **Firmware**: Explicit firmware source, version compatibility, checksum verification, licensing, and fail-closed error handling.
- **Rust Safety**: Keep unsafe operations inside minimal MMIO/DMA/IRQ abstractions with documented safety contracts; leaf drivers stay safe.
- **Diagnostics**: Identity, driver name, state, actionable error codes, rate-limited logging, secret redaction.

---

## PR Review Template and Merge Gates

```markdown
## Summary
What device capability, bug, or integration gap does this PR address?

## Device and scope
Bus, vendor/device or compatible IDs, architecture, subsystem, and exact operations.

## Lifecycle
Probe/attach steps, resources acquired, failure rollback, remove/detach, reset, suspend/resume, and shutdown behavior.

## DMA and concurrency
DMA mask/direction, buffer ownership, sync/barriers, interrupt context, locking, and deferred work.

## Security and failure handling
Untrusted inputs, MMIO bounds, timeouts, retry limits, firmware assumptions, DMA isolation, and recovery behavior.

## Tests and evidence
Unit/mock, fault injection, QEMU, and physical-device tests.
List exact device/machine/firmware configuration and anything not tested.

## Status and documentation
Proposed | Prototype | Integrated | Supported
Link updated support matrix, component page, and test evidence.

## Checklist
- [ ] Probe failure releases every acquired resource in reverse order
- [ ] Detach quiesces IRQs and DMA before freeing memory
- [ ] DMA addresses are distinct from CPU addresses
- [ ] Interrupt handler is bounded and non-blocking
- [ ] Malformed device input and timeout paths are tested
- [ ] Documentation and maturity status match evidence
- [ ] Required CI passes; review conversations are resolved
```

### Reviewer Gates
- **1 Maintainer Approval**: Required for isolated documentation updates or mock unit tests.
- **2 Maintainer Approvals**: Required for DMA mapping, interrupt teardown, unsafe Rust, bus resource allocation, firmware loading, or security-sensitive driver changes.

---

## Exit Criteria
- **Integrated Status**: Documented boot target enumerates and binds the device, exercises its real subsystem path, and handles at least one failure safely.
- **Supported Status**: Repeatable automated tests cover device operations, lifecycle/teardown, error recovery, exact hardware/firmware scope, and documented QEMU machine configurations.
