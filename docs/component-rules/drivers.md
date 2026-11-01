# Component Rules: Drivers

## Component owner
driver-agent

## Upstream references
- Linux virtio subsystem
- FreeBSD device drivers
- Redox virtio schemes

## Purpose
Define required device probe, lifecycle contracts, hardware support documentation,
and validation for SigmaOS device drivers.

## Mandatory rules

1. A driver name does not imply hardware support.
2. Every driver must define probe, initialization, error, and shutdown behavior.
3. Device support requires a named device, driver version, test date, and log.
4. QEMU virtio devices should be validated before physical hardware claims.

## Prohibited

- Claiming hardware support without structured runtime logs
- Unbounded driver polling loops without timeouts
- Ignoring device reset or shutdown sequences
- Panicking the kernel on device initialization failure

## Required evidence

- VirtIO block, network, or console QEMU runtime logs
- Device probe and teardown test execution traces
- Structured entries in `HARDWARE_SUPPORT.md`
- Fault recovery test results when device is detached or resets

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Driver specification and hardware register mapping ADR |
| Prototype | Virtual device probe unit tests |
| Integrated | QEMU VirtIO device runtime pass |
| Hardened | Device reset fuzzing and hotplug detachment tests |
| Supported | Multi-vendor physical hardware test logs and validation suite |
