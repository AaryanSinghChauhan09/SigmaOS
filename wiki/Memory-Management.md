# Memory Management

**Capability state: Prototype.** Allocator and virtual-memory models have not been verified on a booted kernel. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

The repository contains memory-related modules, but there is no verified boot-time physical-memory initialization and user/kernel address-space path. Do not claim buddy/slab allocation, paging protections, swap, huge pages, ASLR, or memory isolation as supported without integration and failure evidence.

## Design references

Linux's page allocator and memory-management documentation, OpenBSD's W^X policy, FreeBSD VM boundaries, and xv6's compact page-table implementation are study references. Adopt only mechanisms that can be tested against SigmaOS's actual boot and process paths.

## Roadmap

1. Validate the bootloader memory-map handoff and reserve kernel/device regions.
2. Add one physical page allocator with exhaustion and invalid-range tests.
3. Establish page-table creation, mapping, unmapping, and permission checks on the selected architecture.
4. Add user address spaces, reclamation, and protection tests after process execution exists.

**Completion evidence:** allocator stress/exhaustion tests and QEMU integration checks demonstrate correct mapping, permission enforcement, and safe failure.
