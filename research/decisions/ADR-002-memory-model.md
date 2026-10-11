# ADR-002: Physical Frame Allocator and Virtual Memory Paging Model

## Status
Accepted

## Upstream study
xv6 `kalloc`/`freerange` page allocator and Linux page frame reclaim / 4-level paging (`CR3`).

## SigmaOS decision
Implement a physical bitmap frame allocator alongside a 4-level page table manager (`PML4T`, `PDPT`, `PD`, `PT`) enforcing SMAP, SMEP, and NX flags for kernel/userspace isolation.

## Scope
- Bitmap-based 4KiB page frame allocator
- Virtual address space mapping and page table walk functions
- Access permission bits (`Present`, `Writable`, `User`, `NoExecute`)

## Non-goals
- 5-level paging (`LA57`)
- Dynamic page compaction under heavy fragmentation
- Swapping to disk in initial boot milestone

## Validation
- Frame allocation and free unit tests
- Virtual memory mapping integration tests
- Kernel page fault handler validation in QEMU
