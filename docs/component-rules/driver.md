# Component Rules: Drivers

## Component owner
driver-agent

## Upstream references
- Linux device driver model and VirtIO subsystem
- FreeBSD KLD and device tree
- OpenBSD dev/ architecture

## Purpose
Govern driver lifecycle, probe/initialization contracts, device identification, and hardware/QEMU validation.

## Mandatory rules
1. A driver name does not imply hardware support.
2. Every driver must define probe, initialization, error handling, and shutdown behavior.
3. Device support requires a named device, driver version, test date, and log.
4. QEMU virtio devices should be validated before physical hardware claims.
5. Foreign OS driver shims must execute inside Ring 3 / Capsicum / IOMMU fault containment containers.

## Prohibited
- Unverified hardware support claims
- Uncontained driver crashes affecting core kernel memory
- Driver initialization without error recovery paths
- Unsandboxed DMA transfers

## Required evidence
- Unit test for driver state transitions
- QEMU device smoke test log (virtio-blk, virtio-net, etc.)
- Driver crash containment fault injection test result
- Hardware matrix audit log

## Promotion criteria
| Level | Requirement |
|---|---|
| Proposed | Driver abstraction ADR and PCI/USB ID mapping |
| Prototype | Unit tests for driver probe and reset logic |
| Integrated | QEMU virtio device operational evidence |
| Hardened | Fault containment injection, stress testing |
| Supported | Verified physical hardware test log and evidence |
