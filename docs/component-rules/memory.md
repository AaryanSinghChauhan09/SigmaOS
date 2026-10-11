# Component Rules: Memory Management

## Component owner
memory-management-agent

## Upstream references
- xv6 physical frame allocator
- Linux paging and demand paging
- FreeBSD VM subsystem

## Purpose
Enforce allocation safety, paging invariants, and memory isolation across SigmaOS.

## Mandatory rules

1. Allocation and deallocation must be paired and measurable.
2. Allocation failure must be handled explicitly; null or dangling dereferences are prohibited.
3. Paging changes require boot validation and memory-integrity tests.
4. No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.
5. Physical frame allocators must guard against double-free and reserved memory corruption.
6. Page tables must enforce W^X (Write XOR Execute) protections across user and kernel address spaces.

## Prohibited

- Unbounded raw pointer dereferences across module boundaries
- Silent ignoring of memory allocation failures
- Double-freeing physical memory frames
- Executable and writable memory page mappings simultaneously without security authorization

## Required evidence

- Memory management unit tests (`cargo test --lib`)
- Allocation failure and out-of-memory handling test execution
- Memory integrity and double-free guard test reports

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and page table architecture design |
| Prototype | Unit tests for allocator and page table mapping |
| Integrated | QEMU boot runtime path with active demand paging |
| Hardened | Shadow memory / ASan validation and stress tests |
| Supported | Reproducible evidence under full physical memory pressure |
