# Component Rules: Drivers

## Component owner
driver-agent

## Upstream references
- Linux virtio drivers & C symbol bridge
- FreeBSD KLD & OpenBSD dev shims
- Redox OS userland drivers

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS driver components.

## Mandatory rules

1. A driver name does not imply hardware support.
2. Every driver must define probe, initialization, error, and shutdown behavior.
3. Device support requires a named device, driver version, test date, and log.
4. QEMU virtio devices should be validated before physical hardware claims.

## Prohibited

- Overbroad hardware compatibility claims without hardware/QEMU logs
- Driver panics crashing the global kernel execution state
- Unsandboxed Tier 2+ driver access to host hardware

## Required evidence

- Driver unit and probe test reports
- QEMU device-specific test log (e.g. virtio-blk, virtio-net)
- Hardware test log if physical support is claimed

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests & mock probe |
| Integrated | QEMU virtio device integration |
| Hardened | Ring 3 driver fault containment |
| Supported | Reproducible hardware/QEMU test log |
