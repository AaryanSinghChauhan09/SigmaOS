# Device Driver Development and Maintenance Plan

## Overview

SigmaOS device driver subsystem follows a structured development and maintenance plan inspired by Linux and BSD driver development practices. This document outlines the architectural roadmap, driver lifecycle rules, proposed Pull Request (PR) sequence, maintenance cadences, and execution criteria.

## Status Vocabulary

To maintain engineering rigor and transparency, all driver capabilities must adhere to the standard SigmaOS status vocabulary:

- **Proposed:** Specification or plan drafted, but implementation has not started
- **Prototype:** Model, stub, or module-level implementation without full bus or OS runtime path integration
- **Integrated:** Code is wired into PCI/bus discovery, uses real DMA/IRQ abstractions, and is verified end-to-end in QEMU
- **Supported:** Validated through repeatable, documented test matrices on exact physical or QEMU hardware with known limitations and rollback paths documented

The initial strategy prioritizes driver infrastructure and virtual devices (VirtIO block and VirtIO network in QEMU) before broad physical hardware support claims.

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

## Implemented Components

The following driver infrastructure components have been implemented in `src/drivers/sovereign_device_driver_development_and_pr_suite.rs`:

### 1. Driver PR Tracker & Status Vocabulary
- Tracks and validates the 15 proposed driver PRs
- Implements DriverSupportStatus enum (Proposed, Prototype, Integrated, Supported)
- Provides overall status calculation

### 2. Bus Device Lifecycle Governor
- Bus enumeration with typed IDs and device matching
- Explicit probe/attach and remove/detach with rollback
- Driver registration and per-device state management
- Probe failure rollback logging

### 3. Platform DMA API Governor
- Strongly-typed CPU virtual, physical, and DMA addresses
- Address-width masks, directions, sync operations
- Mapping/unmapping with error validation

### 4. Interrupt and Deferred Work Contract
- IRQ registration/removal
- Non-blocking handler context rules
- Safe shared-device dispatching
- Deferred task queue

### 5. PCI Enumeration and Resource Governor
- PCI config space scanning
- BAR resource allocation
- Overlapping MMIO prevention
- Device tree generation

### 6. VirtIO Device Integration Suite
- VirtIO block PCI discovery, feature negotiation, queue setup
- VirtIO network PCI discovery, TX/RX virtqueues
- DMA and IRQ integration
- State machine management

### 7. Device Diagnostics and Health/Power Governor
- Health reporting, error tracking
- Read-only diagnostic snapshots
- Power state quiescing

## Future Development Plan

### Phase 1: Foundation Infrastructure (Priority: High)
- [ ] Wire PCI discovery to real hardware scanning
- [ ] Integrate DMA API with physical memory allocator
- [ ] Connect IRQ handlers to APIC/IOAPIC
- [ ] Implement QEMU validation tests

### Phase 2: First Vertical Slice (Priority: High)
- [ ] Complete VirtIO block driver with VFS integration
- [ ] Implement read/write/flush operations
- [ ] Add device reset and shutdown cleanup
- [ ] QEMU boot-to-shell with block device

### Phase 3: Second Vertical Slice (Priority: Medium)
- [ ] Complete VirtIO network driver
- [ ] Integrate with Ethernet stack
- [ ] Implement packet TX/RX path
- [ ] Add link state management

### Phase 4: Diagnostics and Power (Priority: Medium)
- [ ] Implement health monitoring daemon
- [ ] Add power state transitions
- [ ] Implement suspend/resume support
- [ ] Add recovery hooks

### Phase 5: Hardening and Evidence (Priority: Low)
- [ ] Add fault injection test harness
- [ ] Implement QEMU hardware matrix CI
- [ ] Document threat model
- [ ] Publish support matrix

## Testing Strategy

### Unit Tests
- Driver PR tracker validation
- Bus device lifecycle testing
- Probe failure rollback verification
- DMA API mapping/unmapping
- IRQ registration and dispatch
- PCI enumeration validation
- VirtIO block/state machine
- VirtIO network integration
- Device diagnostics and power

### Integration Tests
- QEMU boot with VirtIO block
- QEMU boot with VirtIO network
- Device hotplug simulation
- DMA under memory pressure
- IRQ storm handling
- Multiple device binding

### Evidence Requirements
- Compile pass on x86_64
- Unit test coverage > 80%
- QEMU integration test logs
- Memory leak verification
- Resource cleanup validation

## Maintenance Cadence

| Cadence | Task | Required Evidence |
|---|---|---|
| Every PR | CI Check Suite | Formatting, compilation, unit tests |
| Weekly | Triage hardware and QEMU regressions | Failures tied to issue, reproducer, owner |
| Weekly | Review fuzz and malformed-device findings | Descriptor/MMIO parser crashes fixed |
| Monthly | Verify probe rollback and detach cleanup | Reverse-order cleanup verified |
| Monthly | Check mapping ownership and synchronization | Every mapping has direction, error handling |
| Monthly | Run QEMU discovery and basic I/O matrix | Boot logs, data integrity checks |
| Quarterly | Review device IDs, firmware, compatibility | Device ID tables, firmware versions revalidated |
| Quarterly | Review DMA, MMIO, device trust boundaries | Threat model updated, IOMMU/PMP verified |
| Release Candidate | Certify supported hardware matrix | Release test matrix passed |
| Post-Incident | Prevent recurrence of incident | Reproducer fails before fix, passes after |

## Component-Specific Rules

### PCI and Bus Enumeration
- Validate IDs and BAR sizes
- Reserve resources before mapping
- Prevent overlapping MMIO
- Handle absent devices gracefully

### ACPI and Firmware Discovery
- Separate firmware-described devices from bus-discoverable devices
- Validate table checksums and resource ranges

### DMA and Queues
- Enforce descriptor ownership transitions
- Alignment, length limits, address masks
- Directions, barriers, error recovery

### Interrupt Handlers
- Acknowledge IRQ source promptly
- Avoid blocking/sleeping
- Defer work to worker threads
- Synchronize handlers before detach

### Storage
- Validate command fields and completion lengths
- Bound retries and timeouts
- Test flush, reset, partial I/O data integrity

### Networking
- Validate packet lengths before passing to network stack
- Ensure TX/RX descriptor ownership
- Test link lifecycle

### Rust Safety
- Restrict `unsafe` to audited MMIO/DMA/IRQ abstractions
- Leaf drivers use safe subsystem APIs

### Diagnostics
- Provide identity, bound driver, state, actionable error codes
- Rate-limit logs and redact sensitive memory

## References

- Linux kernel driver documentation: https://www.kernel.org/doc/html/latest/driver-api/
- FreeBSD driver development: https://docs.freebsd.org/en/books/arch-handbook/
- OpenBSD device drivers: https://www.openbsd.org/cgi-bin/man.cgi?query=intro&sektion=9
- QEMU device emulation: https://www.qemu.org/docs/master/system/
