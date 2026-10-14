# Component Rules: Drivers

## Component owner
driver-agent

## Upstream references
- Linux Device Drivers & virtio architecture
- FreeBSD WDM-Style Driver Model
- Redox Hardware Drivers

## Purpose
Define initialization lifecycle, hardware evidence requirements, and virtio validation rules for SigmaOS device drivers.

## Mandatory rules

1. A driver name does not imply hardware support.
2. Every driver must define explicit probe, initialization, error, and shutdown behavior (following WDM-style `IoManager`/`DriverObject`/`DeviceObject`).
3. Device support claims require a named device, driver version, test date, and evidence log.
4. QEMU virtio devices (`virtio-blk`, `virtio-net`, `virtio-gpu`) must be validated before physical hardware claims are made.
5. All driver MMIO and DMA operations must use bounds-checked memory abstractions.

## Prohibited

- Claiming physical hardware compatibility without driver test evidence logs
- Unbounded DMA buffer allocation or unmapped MMIO pointer dereference
- Blocking device initialization loops without timeout bounds
- Status promotion to `Integrated` or `Supported` without QEMU or hardware logs

## Required evidence

- Driver lifecycle unit test report
- QEMU device smoke test log (e.g. `virtio-blk` / `virtio-net` boot output)
- Hardware driver evidence ledger entry (`docs/platform/HARDWARE_SUPPORT.md`)

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Driver specification and device probe ADR |
| Prototype | Unit-tested driver state machine |
| Integrated | QEMU virtio emulation boot and device operation test |
| Hardened | Error recovery, interrupt stress, DMA bounds audit |
| Supported | Tested on physical hardware with published log evidence |
