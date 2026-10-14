# Component Rules: Memory

## Component owner
memory-agent

## Upstream references
- xv6 Physical Memory Allocator
- Linux Paging & Virtual Memory Management
- FreeBSD VM Subsystem

## Purpose
Define required allocation discipline, paging integrity, and address space isolation rules for SigmaOS memory components.

## Mandatory rules

1. Allocation and deallocation must be paired and measurable.
2. Allocation failure must be handled explicitly without kernel panics unless unrecoverable invariant failure occurs.
3. Paging changes require boot validation and memory-integrity tests.
4. No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.
5. Address spaces must enforce strict user/kernel isolation and W^X (Write XOR Execute) security policies.

## Prohibited

- Unpaired allocation/deallocation causing double-frees or memory leaks
- Raw pointer exposure across module boundaries without safety invariant comments
- Non-canonical virtual address mapping or bypassing protection flags
- Unbounded kernel memory allocations

## Required evidence

- Memory unit test report (`tests/memory/physical.rs`, `tests/mm/paging.rs`, `tests/mm/virtual.rs`)
- Out-of-memory (OOM) handling and exhaustion test logs
- QEMU boot and page-fault stress evidence

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Memory policy ADR and lock ordering spec |
| Prototype | Frame allocator and page table unit tests |
| Integrated | Boot path paging validation and VM integration tests |
| Hardened | Double-free detection, fuzzing, W^X security audit |
| Supported | Reproducible stress/OOM rehearsal evidence |
