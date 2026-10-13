# ADR-002: Frame Allocator and 4-Level Paging

## Status
Accepted

## Upstream Study
Linux uses buddy allocator and sparse memory models; xv6 uses a simple free-list of 4KiB pages.

## SigmaOS Decision
Implement a bitmap-based physical frame allocator and 4-level x86_64 page table mapping.

## Scope
- E820 physical memory map parsing
- Bitmap physical page allocation
- Kernel identity mapping and higher-half transition

## Non-goals
- NUMA memory policy
- Memory hotplugging

## Validation
- Memory allocation unit tests
- QEMU boot test with 512MB RAM
