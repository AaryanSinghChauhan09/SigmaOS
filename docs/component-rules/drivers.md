# Component Rules: Drivers

## Component owner
driver-agent

## Upstream references
- Linux device drivers and virtio subsystems
- FreeBSD driver architecture
- Redox driver schemes

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS device drivers and hardware interfaces.

## Mandatory rules

1. A driver name does not imply hardware support.
2. Every driver must define probe, initialization, error, and shutdown behavior.
3. Device support requires a named device, driver version, test date, and log.
4. QEMU virtio devices should be validated before physical hardware claims.

## Prohibited

- Claiming hardware support without structured test logs
- Missing shutdown or error-recovery handlers
- Blocking interrupts indefinitely in driver top-half
- Status promotion without QEMU device test evidence

## Required evidence

- Driver lifecycle unit tests (probe, init, reset, shutdown)
- QEMU virtio device verification logs (virtio-blk, virtio-net, etc.)
- Device support matrix entry (named device, driver version, date, log)
- Failure injection and recovery test reports

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests and mock device validation |
| Integrated | QEMU virtio device-specific test and runtime integration |
| Hardened | Error injection, fuzzing, recovery verification |
| Supported | Reproducible boot/install/update evidence on documented hardware |
