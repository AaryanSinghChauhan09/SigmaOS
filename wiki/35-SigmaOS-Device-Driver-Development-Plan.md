# 35-SigmaOS-Device-Driver-Development-Plan

# SigmaOS Device Driver Development and Maintenance Plan

## Overview & Executive Summary
SigmaOS implements a unified, evidence-gated device driver model inspired by the Linux Device Driver Model and FreeBSD Newbus architecture. While SigmaOS lists modules for serial, RTC, display, input, storage, USB, audio, and networking, unit tests alone do not prove runtime or physical hardware support.

This document defines the authoritative architecture, lifecycle contracts, DMA isolation policies, 15-PR implementation roadmap, periodic maintenance cadences, component-specific rules, PR review templates, and support status criteria for device drivers in SigmaOS.

## Status Vocabulary & Evidence Bar
All driver claims in SigmaOS must strictly use the four-tier status vocabulary:

| Status | Definition | Evidence Requirement |
| :--- | :--- | :--- |
| **Proposed** | Driver specification or PR proposed; no functional code integrated. | PR proposal document in `docs/` or `wiki/`. |
| **Prototype** | Experimental implementation; passes synthetic unit or mock tests. | Unit tests in `src/driver/` passing via `rustc --test`. |
| **Integrated** | Device discovers on target bus (e.g. QEMU PCI), binds driver, exercises real subsystem paths, and handles failure safely. | QEMU automated boot smoke check log exercising real read/write/interrupts. |
| **Supported** | Full hardware/QEMU validation, teardown/lifecycle safety, error recovery, and documented hardware/firmware matrix. | Repeatable hardware or multi-QEMU matrix tests covering operations, resets, and error paths. |

### Current Driver Inventory Audit

| Driver / Subsystem | Bus | Vendor / Device IDs | Arch | Current Status | Tested Operations | Failure Behavior | Known Limitations |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| 16550 UART | ISA / MMIO | N/A (Standard COM) | x86_64, aarch64, riscv64 | Supported | Polled/IRQ TX/RX, baud setting | Ignores invalid baud, safe fallback | FIFO buffer depth fixed |
| VirtIO Block | PCI / VirtIO | `0x1AF4:0x1001` | x86_64, aarch64, riscv64 | Integrated | QEMU sector read/write/flush, virtqueue reset | Aborts invalid sector, probe rollback | Needs real DMA IOMMU confinement |
| VirtIO Network | PCI / VirtIO | `0x1AF4:0x1000` | x86_64, aarch64, riscv64 | Integrated | Ethernet RX/TX frame queueing, link state | Drops malformed frames, reset queue | MTU fixed at 1500 bytes |
| Intel e1000 NIC | PCI | `0x8086:0x100E` | x86_64 | Prototype | Ring setup, frame send/receive | Reset on ring overflow | QEMU e1000 only; physical hardware untested |
| NVMe Storage | PCIe | Class `0x010802` | x86_64, aarch64 | Prototype | Admin queue identify, sector I/O simulation | Timeout error returned | Requires real DMA page mapping on metal |
| AHCI SATA | PCI | Class `0x010601` | x86_64 | Prototype | FIS command construction, port probe | FIS error flag set | Needs hardware interrupt routing validation |
| DRM KMS / GPU | PCI | Various (Intel, AMD, NVIDIA) | x86_64 | Prototype | Atomic commit state, dumb buffer create | Returns EINVAL on invalid property | Page-flip path simulated; hardware GPU pending |

---

## Driver Rules for Every PR

1. **Separate Device Discovery from Driver Logic**
   - Bus enumeration (PCI, ACPI, USB, Platform) matches devices to drivers via typed `DeviceId` rules.
   - Drivers must never scan raw global hardware independently. Firmware-described devices (ACPI) and bus-enforceable devices (PCI) use distinct discovery paths.

2. **Explicit Probe/Attach and Remove/Detach Lifecycles**
   - Probe function acquires resources sequentially. Any probe failure must trigger automatic reverse-order cleanup of all acquired resources.
   - Detach must quiesce device interrupts and DMA before unregistering subsystem handles and releasing memory.

3. **Platform DMA API & Address Type Isolation**
   - CPU Virtual (`CpuVirtualAddress`), CPU Physical (`CpuPhysicalAddress`), and Device DMA (`DeviceDmaAddress`) addresses are distinct, non-coercible Rust types.
   - All DMA mappings must specify mask, direction, and sync operations (`sync_for_cpu`, `sync_for_device`). Map/unmap failures must fail safely.

4. **Interrupt Context Constraints**
   - Primary IRQ handlers must be short, non-blocking, and limited to acknowledging the interrupt and scheduling deferred work.
   - IRQ context prohibits memory allocations, blocking mutexes, or sleeping.

5. **Concurrency and Memory Ordering**
   - MMIO register accesses and DMA descriptors require explicit memory barriers (`Ordering::SeqCst` / fence) before notifying device hardware.
   - Lock ordering and descriptor ownership transitions must be documented.

6. **Quiesce Before Freeing**
   - Teardown sequence: disable device interrupts -> stop DMA transfers -> drain deferred task queues -> synchronize with running handlers -> unmap DMA/MMIO -> free driver memory.

7. **Safe Rust Abstractions & Narrow Unsafe Boundaries**
   - Volatile MMIO, raw pointers, and architecture-specific ops are encapsulated behind safe wrapper abstractions with explicit `# Safety` doc contracts.
   - Leaf drivers interact only with safe subsystem interfaces.

8. **Evidence-Based Test Separation**
   - Distinguish unit/mock parser tests from QEMU integration tests and physical hardware tests.
   - Claims of "Supported" status require QEMU or physical hardware evidence.

9. **Configuration Transparency**
   - Document exact vendor/device IDs, firmware versions, architecture, bus type, and QEMU machine flags used during testing.

10. **Secure Contribution Controls**
    - All driver changes require maintainer code review, passing CI checks, and protected default branch enforcement.

---

## 15-PR Implementation Roadmap

| PR | Title | Main Work | Acceptance Criteria | Status |
| :-: | :--- | :--- | :--- | :-: |
| **1** | `docs(drivers): audit driver status and define support evidence` | Reconcile driver source, tests, wiki, and `FEATURE_STATUS.toml`. | Evidence separated; per-driver status table published. | Supported |
| **2** | `feat(drivers): define device, bus, and driver lifecycle interfaces` | Typed device descriptors, bus enumeration, probe/detach lifecycle. | Mock bus matches, probes, and cleans up failed probes. | Integrated |
| **3** | `feat(drivers): add managed resource acquisition and rollback` | RAII resource guards for MMIO, IRQs, DMA, and subsystem handles. | Injected failures at any probe step release all resources. | Integrated |
| **4** | `feat(mmio): add validated register access and ordering APIs` | Typed MMIO mapping, volatile width-aware access, bounds checks. | Out-of-bounds MMIO rejected; barriers documented. | Integrated |
| **5** | `feat(dma): add device DMA mapping API with masks and ownership` | Coherent/streaming DMA mappings, address width masks, directions. | Distinct address types enforced; map/unmap/sync paired. | Integrated |
| **6** | `feat(irq): define interrupt registration and deferred work contract` | IRQ registration, context checks, shared dispatch, deferred task queue. | Handlers non-blocking; detach waits for running handlers. | Integrated |
| **7** | `feat(pci): add QEMU PCI enumeration and resource ownership` | PCI config space scan, BAR size calculation, device tree generation. | Devices enumerated once; overlapping BARs rejected. | Integrated |
| **8** | `feat(drivers): integrate VirtIO block through PCI discovery` | Connect VirtIO block to PCI discovery, virtqueues, DMA, IRQs, VFS. | QEMU VirtIO block sector read/write/flush and shutdown. | Integrated |
| **9** | `feat(drivers): connect VirtIO network queues to Ethernet stack` | VirtIO net interface lifecycle, TX/RX virtqueues, Ethernet stack. | QEMU NIC frame send/receive, queue reset, link state. | Integrated |
| **10** | `feat(drivers): add device diagnostics and health reporting` | Per-device identity, state, error counters, devlink-style health. | Diagnostics read-only; secrets redacted; recovery hooks. | Integrated |
| **11** | `feat(drivers): implement suspend resume and shutdown contracts` | Power state transitions; quiesce DMA/IRQs before suspend/shutdown. | Power state sequence tested; DMA/IRQ stopped before off. | Integrated |
| **12** | `test(drivers): add fault-injection and lifecycle test harness` | Alloc/map/IRQ/timeout/reset failure injection harness. | Failure points covered; resource counts return to baseline. | Integrated |
| **13** | `test(drivers): add QEMU hardware matrix and boot smoke checks` | Automate QEMU machines for PCI, VirtIO, serial, storage, input. | CI verifies discovery, bind, I/O, error, and teardown. | Integrated |
| **14** | `security(drivers): add driver threat model and DMA isolation policy` | Untrusted device assumptions, DMA confinement, privileged registers. | Threat model defined; DMA address bounds enforced. | Integrated |
| **15** | `docs(drivers): publish device support matrix and recovery guide` | Document QEMU/hardware devices, firmware, and recovery steps. | Supported entries link to repeatable evidence. | Supported |

---

## Recommended Implementation Order
1. **Foundation (PRs 1–7)**: Establish unified bus model, managed RAII resources, MMIO access, Platform DMA API, IRQ deferred work, and PCI enumeration.
2. **First Vertical Slice (PR 8)**: Integrate VirtIO block end-to-end through PCI discovery to provide real storage backing for boot/shell smoke tests.
3. **Networking Slice (PR 9)**: Integrate VirtIO network through Ethernet stack and virtqueues.
4. **Diagnostics & Lifecycle (PRs 10–12)**: Devlink health reporting, power management, suspend/resume, and fault-injection testing.
5. **Validation & Security (PRs 13–15)**: Automated QEMU matrix, threat model, DMA isolation, and support matrix documentation.

---

## Periodic Maintenance PR Cadence

- **Every PR**: Run `ci(drivers)` verifying format, build, unit tests, and relevant bus/DMA/IRQ tests.
- **Weekly**: Triage hardware and QEMU regressions; review fuzz/malformed device input findings.
- **Monthly**: Audit probe failure rollback and detach cleanup across all drivers; check DMA mapping synchronization.
- **Quarterly**: Review device IDs, firmware requirements, DMA/MMIO security bounds, and hardware compatibility.
- **Each Release Candidate**: Certify supported hardware matrix with exact QEMU machine configurations and physical test logs.
- **After Each Incident**: Publish post-mortem and regression test for driver or DMA failure.

---

## Component-Specific Rules

- **PCI & Bus Enumeration**: Validate vendor/device IDs and BAR sizes; reject overlapping MMIO allocations; support bus-mediated matching.
- **ACPI & Firmware Discovery**: Separate firmware-described devices from bus-enforceable PCI devices; validate table checksums and resource ranges.
- **DMA & Queues**: Enforce explicit descriptor ownership transitions, alignment, address width masks, and barriers.
- **Interrupt Handlers**: Primary handler must be non-blocking; detach must quiesce IRQ stream before handler removal.
- **Storage**: Validate command fields and buffer lengths; bound retries/timeouts; implement explicit sector flush.
- **Networking**: Validate frame lengths before passing to stack; manage TX/RX descriptor rings safely.
- **USB / Input / Audio / Display**: Use standardized subsystem abstractions rather than direct userland interfaces.
- **Power & Hotplug**: Explicit D0-D3 state transitions; suspend must stop new I/O, drain queues, and quiesce DMA.
- **Rust Safety**: All `unsafe` blocks restricted to low-level MMIO/DMA abstractions with explicit `# Safety` contracts.

---

## PR Review Template & Merge Gates

```text
## Summary
What device capability, bug, or integration gap does this PR address?

## Device and scope
Bus, vendor/device IDs, architecture, subsystem, and target operations.

## Lifecycle
Probe steps, resources acquired, failure rollback, remove/detach, reset, power transitions.

## DMA and concurrency
DMA mask/direction, address types, sync/barriers, IRQ context, deferred work.

## Security and failure handling
Untrusted device bounds, timeouts, retry limits, firmware assumptions, DMA isolation.

## Tests and evidence
Unit/mock tests, fault injection, QEMU configuration, and physical device test evidence.

## Status and documentation
Proposed | Prototype | Integrated | Supported

## Checklist
- [ ] Probe failure releases every acquired resource in reverse order
- [ ] Detach quiesces IRQs and DMA before freeing memory
- [ ] DMA addresses are distinct types from CPU Virtual/Physical addresses
- [ ] Interrupt handler is bounded and non-blocking
- [ ] Malformed device input and timeout paths are tested
- [ ] Required CI checks pass; documentation updated
```

### Reviewer Requirements
- **1 Maintainer Approval**: Required for isolated documentation, mock drivers, or diagnostic additions.
- **2 Maintainer Approvals**: Mandatory for changes touching DMA mapping APIs, interrupt handling, unsafe Rust blocks, PCI bus resource allocation, or security confinement.

---

## Industry References & Evidence Bar
- **Linux Kernel**: PCI driver model (`drivers/pci`), DMA mapping guide (`Documentation/core-api/dma-api.rst`), devlink health framework (`net/core/devlink.c`), KUnit and kselftest suites.
- **FreeBSD**: Newbus architecture (`sys/kern/subr_bus.c`), PCI resource management handbook, interrupt teardown contracts.
