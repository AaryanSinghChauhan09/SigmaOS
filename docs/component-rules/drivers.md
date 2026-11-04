# Component Rules: Drivers

## Component owner
driver-agent

## Upstream references
- Linux virtio device drivers (virtio-blk, virtio-net, virtio-gpu)
- FreeBSD device driver framework and bus architecture
- OSDev PCI enumeration and MMIO standards

## Purpose
Define required driver lifecycle, initialization sequences, fault recovery, and validation
for SigmaOS hardware abstraction and device drivers.

## Mandatory rules

1. A driver name does not imply hardware support.
2. Every driver must define probe, initialization, error, and shutdown behavior.
3. Device support requires a named device, driver version, test date, and log.
4. QEMU virtio devices should be validated before physical hardware claims.
5. All hardware MMIO/PIO accesses must be guarded by memory mapping validation.
6. DMA transfers must enforce buffer alignment and cache invalidation invariants.
7. Device disconnect or unresponsiveness must produce timeouts rather than kernel hangs.

## Prohibited

- Unverified hardware support claims
- Unsynchronized concurrent MMIO operations
- Unbounded busy loops during hardware polling
- Status promotion without device-specific QEMU or hardware logs

## Required evidence

- Unit test report
- Integration test report
- QEMU device-specific test log
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
