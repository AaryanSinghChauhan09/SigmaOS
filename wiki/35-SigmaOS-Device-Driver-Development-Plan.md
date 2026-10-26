# 35 SigmaOS Device Driver Development Plan

## Subsystem Overview
The **SigmaOS Device Driver Subsystem** establishes a unified, safe, managed device driver architecture inspired by Linux's PCI driver model/DMA API and FreeBSD's Newbus resource management.

SigmaOS lists drivers for serial, RTC, display, input, storage, USB, audio, and networking. A driver module or passing unit tests do not prove runtime or hardware support. The wiki’s status vocabulary—**Proposed**, **Prototype**, **Integrated**, **Supported**—governs all driver claims.

---

## Status Vocabulary
- **Proposed**: Planned PR/capability with defined architecture and interfaces.
- **Prototype**: Implementation undergoing mock or initial unit testing without bus/DMA integration.
- **Integrated**: Verified in end-to-end QEMU integration with real PCI discovery, DMA mapping, interrupt handling, and subsystem hooks.
- **Supported**: Verified across documented physical hardware or named QEMU target machines with repeatable evidence logs and failure recovery tests.

---

## Driver Rules for Every PR
1. **Separate device discovery from driver logic**: Bus enumeration matches devices to registered drivers; leaf drivers must never perform raw uncoordinated global scanning.
2. **Explicit probe/attach and remove/detach lifecycles**: Implement explicit probe and remove callbacks with reverse-order resource cleanup on failure.
3. **Never treat a DMA address as a CPU address**: Distinguish CPU virtual, physical, and DMA bus addresses explicitly.
4. **Define interrupt context**: Interrupt handlers must be short, non-blocking, and allocation-free, deferring lengthy work to task queues.
5. **Concurrency and memory ordering**: Document ownership, locking, barriers, and memory ordering for descriptor queues and MMIO.
6. **Quiesce before freeing**: Disable interrupts, stop DMA, drain/cancel work queues, and synchronize with running handlers before freeing buffers.
7. **Safe Rust boundaries**: Encapsulate volatile MMIO and DMA behind narrow unsafe abstractions with documented safety contracts.
8. **Test the real path**: Differentiate unit/mock tests from QEMU integration tests and physical hardware tests.
9. **No broad claims from single machines**: Document exact device IDs, firmware, bus, architecture, QEMU machine configuration, and tested operations.
10. **Secure contribution controls**: Require pull request reviews, status checks, and branch protection.

---

## 15-PR Implementation Gateway

| PR | Proposed Title | Main Work | Acceptance Criteria | Status |
|---|---|---|---|---|
| **1** | `docs(drivers): audit driver status and define support evidence` | Reconcile source, tests, wiki, and `FEATURE_STATUS.toml`. | Per-driver evidence table published; unsupported devices not implied. | **Supported** |
| **2** | `feat(drivers): define device, bus, and driver lifecycle interfaces` | Typed device descriptors, bus enumeration, probe/remove callback lifecycles. | Mock bus enumeration, probe failure cleanup tested. | **Integrated** |
| **3** | `feat(drivers): add managed resource acquisition and rollback` | RAII resource guards for MMIO mappings, IRQs, DMA mappings, queues. | Reverse-order rollback on probe failure tested. | **Integrated** |
| **4** | `feat(mmio): add validated register access and ordering APIs` | Typed MMIO mapping, volatile width-aware access, bounds checks. | Out-of-range MMIO accesses rejected; barriers documented. | **Integrated** |
| **5** | `feat(dma): add device DMA mapping API with masks and ownership` | Coherent & streaming DMA mappings, address masks, directions, sync. | CPU virtual vs DMA address typing enforced; sync operations tested. | **Integrated** |
| **6** | `feat(irq): define interrupt registration and deferred work contract` | IRQ registration, context checks, shared dispatch, deferred task queue. | Non-blocking handlers; synchronized detach; no sleep in IRQ. | **Integrated** |
| **7** | `feat(pci): add QEMU PCI enumeration and resource ownership` | PCI config space scan, BAR size calculation, MMIO allocation, device tree. | PCI devices enumerated once; overlapping BARs rejected. | **Integrated** |
| **8** | `feat(drivers): integrate VirtIO block through PCI discovery` | Connect VirtIO block to PCI discovery, virtqueue, DMA, IRQ, VFS. | QEMU VirtIO block sector read/write/flush and clean shutdown tested. | **Integrated** |
| **9** | `feat(drivers): connect VirtIO network queues to Ethernet stack` | Connect VirtIO net to TX/RX virtqueues, DMA, Ethernet stack. | QEMU NIC frame transmit/receive through real stack tested. | **Integrated** |
| **10** | `feat(drivers): add device diagnostics and health reporting` | Identity, state, error counters, read-only devlink diagnostic snapshots. | Diagnostics redact secrets; failed devices produce actionable diagnostic status. | **Integrated** |
| **11** | `feat(drivers): implement suspend resume and shutdown contracts` | Power state transitions; quiesce DMA/IRQs before suspend/reboot. | Power lifecycle tested; DMA/IRQ stopped before teardown. | **Integrated** |
| **12** | `test(drivers): add fault-injection and lifecycle test harness` | Alloc/map/IRQ/timeout/reset failure injection harness. | Failure points covered; resource counts return to baseline. | **Integrated** |
| **13** | `test(drivers): add QEMU hardware matrix and boot smoke checks` | Automate QEMU machines for PCI, VirtIO, serial, storage, input. | CI verifies discovery, bind, I/O, error, and teardown. | **Integrated** |
| **14** | `security(drivers): add driver threat model and DMA isolation policy` | Untrusted device assumptions, DMA confinement, privileged register rules. | Threat model defined; address bounds and DMA isolation rules enforced. | **Integrated** |
| **15** | `docs(drivers): publish device support matrix and recovery guide` | Document supported QEMU/hardware devices, firmware, recovery. | Supported entries link directly to repeatable evidence logs. | **Supported** |

---

## Periodic Maintenance Cadence
- **Every PR**: Run formatting, clippy, unit tests, and relevant bus/DMA/IRQ/integration tests.
- **Weekly**: Triage hardware and QEMU regressions; review fuzz & malformed-device findings.
- **Monthly**: Audit probe rollback & detach cleanup; check DMA mapping ownership & synchronization; run QEMU discovery & I/O matrix.
- **Quarterly**: Review device IDs, firmware, and compatibility claims; review DMA, MMIO, and device trust boundaries.
- **Release Candidate**: Certify supported hardware matrix with exact tested machine parameters.
- **Post-Incident**: Prevent recurrence with root-cause analysis and regression tests.

---

## Component-Specific Rules
- **PCI & Bus Enumeration**: Validate BAR sizes, reserve resources before mapping, reject overlapping MMIO.
- **DMA & Queues**: Validate descriptor ownership, alignment, direction, memory barriers, and cache coherency.
- **Interrupt Handlers**: Non-blocking handlers only; defer lengthy work to task queues; quiesce IRQs before detach.
- **Storage**: Bounded timeouts, sector flush validation, data integrity checks on partial I/O.
- **Networking**: Validate descriptors before handing buffers to stack; test link lifecycle and queue resets.
- **Rust Safety**: Encapsulate volatile operations behind reviewed unsafe abstractions; keep leaf drivers safe.

---

## Reference Implementation
The master driver gateway suite is implemented in:
`src/driver/sovereign_device_driver_development_and_pr_suite.rs`
Registered in `FEATURE_STATUS.toml` under `[driver.development_plan]`.
