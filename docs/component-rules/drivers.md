# Component Rules: Drivers

## Component owner
driver-agent

## Upstream references
- VirtIO 1.2 specification
- Linux driver model
- FreeBSD devstat & bus attachment

## Purpose
Define required device driver initialization, lifecycle management, and validation standards.

## Mandatory rules

1. A driver name does not imply hardware support.
2. Every driver must define probe, initialization, error, and shutdown behavior.
3. Device support requires a named device, driver version, test date, and log.
4. QEMU virtio devices should be validated before physical hardware claims.
5. Interrupt handlers and DMA operations must be race-free and bounded.

## Prohibited

- Unsubstantiated real-hardware support claims without hardware evidence logs
- Missing driver cleanup or memory leaks on device detachment/unplug
- Direct unsafe IO port/register access without safety bounds
- Unhandled device timeout or reset loops

## Required evidence

- VirtIO block and network device unit tests
- QEMU device probe and I/O integration test report
- Hardware evidence log (for physical driver promotion)
- Failure injection test report

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Driver specification and hardware matrix entry |
| Prototype | Unit tests for driver state machine |
| Integrated | QEMU VirtIO device boot/IO validation |
| Hardened | Hot-plug, error recovery, and DMA boundary verification |
| Supported | Physical hardware execution log and evidence record |
