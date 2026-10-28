# Component Rules: Drivers

## Component owner
drivers-agent

## Upstream references
- Linux virtio device subsystem
- FreeBSD device driver architecture
- Redox OS userspace driver model

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS Hardware Driver components.

## Mandatory rules

1. A driver name does not imply hardware support.
2. Every driver must define probe, initialization, error, and shutdown behavior.
3. Device support requires a named device, driver version, test date, and log.
4. QEMU virtio devices should be validated before physical hardware claims.

## Prohibited

- Unsubstantiated parity or physical hardware support claims
- Missing initialization or shutdown cleanup handlers
- Direct port/MMIO access without safety wrappers
- Promotion to `Supported` status without runtime hardware logs

## Required evidence

- Driver unit test suite log
- QEMU virtio device verification output (virtio-blk, virtio-net, VGA)
- Named device evidence ledger entry with test log
- Regression test for driver stability

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
