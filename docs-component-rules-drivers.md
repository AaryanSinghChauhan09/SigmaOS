# Component Rules: Drivers

## Component owner
driver-agent

## Upstream references
- Linux Drivers & VirtIO Specification
- WDM (Windows Driver Model) Architecture
- FreeBSD Network & Storage Drivers

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Device Driver components (PCI, USB, VirtIO, Framebuffer, Network, Storage).

## Mandatory rules

1. A driver name does not imply hardware support; support requires a named device, driver version, test date, and log.
2. Every driver must define explicit probe, initialization, error handling, and shutdown behavior (WDM-style IoManager/DriverObject/DeviceObject).
3. QEMU virtio devices (virtio-blk, virtio-net, virtio-gpu) must be validated in QEMU before physical hardware claims are made.
4. Device MMIO operations must use bounded, volatile, typed memory maps.
5. DMA operations must validate physical buffer bounds and enforce IOMMU / alignment constraints.

## Prohibited

- Claiming physical hardware compatibility without driver evidence logs
- Unbounded MMIO reads/writes or unmapped physical address access
- Unhandled device interrupts or missing IRQ handler cleanup
- Driver crash propagating to unrecoverable kernel panic
- Status promotion without QEMU or hardware diagnostic log

## Required evidence

- Driver probe/init/shutdown unit test logs
- QEMU VirtIO device test execution output
- Diagnostics log from `collect_diagnostics.sh` and driver evidence test
- Hardware support ledger entry in `docs/platform/HARDWARE_SUPPORT.md`

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Driver specification and hardware matrix registration |
| Prototype | Standalone unit tests & simulated MMIO probe tests |
| Integrated | QEMU VirtIO device integration & IO control tests |
| Hardened | Hot-plug/removal fault injection, DMA stress tests |
| Supported | Real hardware diagnostic log and reproducible boot evidence |
