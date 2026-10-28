# Component Rules: Drivers

## Component owner
driver-agent

## Upstream references
- Linux virtio subsystem
- FreeBSD device driver framework
- Redox driver schemes

## Purpose
Define hardware probe, lifecycle contracts, and validation evidence
for SigmaOS Device Driver components.

## Mandatory rules

1. A driver name does not imply hardware support.
2. Every driver must define probe, initialization, error, and shutdown behavior.
3. Device support requires a named device, driver version, test date, and log.
4. QEMU virtio devices should be validated before physical hardware claims.

## Prohibited

- Claiming physical hardware compatibility without test logs
- Ignoring driver probe failure or device disconnects
- Bypassing I/O memory safety boundaries
- Status promotion without device-specific runtime logs

## Required evidence

- Unit test report
- QEMU virtio device test report
- Hardware diagnostic log (when physical claims made)
- Regression test for driver crashes

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
