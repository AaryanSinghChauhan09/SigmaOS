# Component Rules: Drivers

## Component owner
driver-agent

## Upstream references
- Linux Device Drivers & virtio framework
- Windows Driver Model (WDM) pattern
- FreeBSD Driver Subsystem

## Purpose
Define initialization lifecycle, hardware probe evidence standards, and device safety bounds for SigmaOS drivers.

## Mandatory rules

1. A driver name does not imply hardware support. Support claims require evidence.
2. Every driver must define explicit probe, initialization, error handling, and shutdown behavior.
3. Device support claims require a named device, driver version, test date, and validation log.
4. QEMU virtio devices (virtio-blk, virtio-net, virtio-gpu) must be validated before making physical hardware support claims.
5. All driver MMIO and DMA operations must be bounds-clamped and error-checked.

## Prohibited

- Unbounded MMIO reads/writes without bounds checking
- Claiming physical hardware compatibility based solely on stub structures
- Unhandled device timeout or register freeze
- Silent driver failure during kernel initialization

## Required evidence

- Driver probe unit test report
- QEMU virtio execution log
- Hardware evidence log in driver evidence matrix (when physical hardware is claimed)
- Fault injection recovery test output

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and device specification review |
| Prototype | Unit tests for register initialization and command structures |
| Integrated | QEMU virtio execution log and device probe verification |
| Hardened | DMA fault injection, hot-plug, and reset recovery validation |
| Supported | Physical hardware evidence log with reproducible diagnostic trace |
