# Component Rules: Memory

## Component owner
memory-agent

## Upstream references
- xv6 Memory Management
- Linux Page Frame Allocator & Paging
- FreeBSD VM Subsystem

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Memory Management components (Physical, Virtual, Paging, and Swap).

## Mandatory rules

1. Allocation and deallocation must be paired, race-free, and measurable.
2. Allocation failure (OOM) must be handled explicitly without kernel crashes.
3. Paging changes require boot validation and W^X memory-integrity tests.
4. No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.
5. Frame allocator double-free prevention and reserved physical region protections must be active.
6. 4-level x86_64 page table operations must flush TLB entries explicitly on unmap or remap.

## Prohibited

- Unbounded physical memory allocations
- Double-freeing or leaking allocated page frames
- Exposing raw pointers across module boundaries without safety invariant documentation
- Silently ignoring physical frame allocation failures
- Status promotion without memory-integrity runtime evidence

## Required evidence

- Unit test report for physical frame allocator and paging
- Memory exhaustion and OOM handling test logs
- QEMU memory paging and W^X protection execution logs
- Regression tests for memory leaks or double-frees

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and memory architecture analysis |
| Prototype | Physical allocator and paging unit tests |
| Integrated | Kernel address space integration & QEMU paging tests |
| Hardened | OOM stress tests, frame fuzzing, double-free validation |
| Supported | Multi-gigabyte physical boot and swap retention evidence |
