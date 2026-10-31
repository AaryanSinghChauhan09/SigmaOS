# Component Rules: Memory

## Component owner
memory-agent

## Upstream references
- xv6 kalloc
- Linux page table management
- FreeBSD jemalloc and UMA

## Purpose
Define required behavior, memory safety boundaries, page lifecycle, and validation
for SigmaOS physical and virtual memory subsystems.

## Mandatory rules

1. Allocation and deallocation must be paired and measurable.
2. Allocation failure must be handled explicitly.
3. Paging changes require boot validation and memory-integrity tests.
4. No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.

## Prohibited

- Unchecked kernel heap allocations
- Double-freeing physical frames or virtual pages
- Silent memory corruption or leaking frame metadata
- Raw pointer leakage across crate or module interfaces without `unsafe` justification

## Required evidence

- Frame allocator paired allocation/deallocation test logs
- Paging stress test and allocation exhaustion benchmarks
- QEMU memory integrity boot logs
- Memory leak and double-free regression test reports

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Memory ADR and allocator specification |
| Prototype | Frame allocator unit tests |
| Integrated | QEMU virtual memory paging boot test |
| Hardened | OOM exhaustion and fuzz tests |
| Supported | Multi-gigabyte bare-metal validation |
