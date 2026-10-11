# Component Rules: Drivers & HAL

## Component owner
drivers-agent

## Upstream references
- Linux Device Drivers & VirtIO
- FreeBSD Device Drivers
- Windows Driver Model (WDM)

## Purpose
Govern hardware abstraction layers, device probing, ring buffers, and VirtIO hardware contracts.

## Mandatory rules

1. A driver name does not imply hardware support.
2. Every driver must define probe, initialization, error recovery, and shutdown behavior.
3. Device support requires a named device, driver version, test date, and log.
4. QEMU virtio devices should be validated before physical hardware claims are made.
5. All driver I/O operations must handle device timeout and reset gracefully without kernel panic.

## Prohibited

- Unsupported parity claims for hardware names without test logs
- Infinite polling loops without timeout guards
- Direct unsafe MMIO access outside authorized device extensions
- Status promotion without QEMU or physical device execution logs

## Required evidence

- Driver unit test execution (`cargo test --lib`)
- Device driver probe and initialization logs
- QEMU virtio-blk / virtio-net validation output

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and device registers specification |
| Prototype | Mock driver unit tests |
| Integrated | QEMU virtual hardware probe and I/O success log |
| Hardened | Device reset recovery and fault injection testing |
| Supported | Tested and verified physical or virtual hardware device log |
