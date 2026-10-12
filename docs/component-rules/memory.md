# Component Rules: Memory

## Component owner
memory-agent

## Upstream references
- xv6 kalloc and page table management
- Linux physical frame allocator & buddy/slab allocators
- FreeBSD VM subsystem

## Purpose
Enforce allocation discipline, explicit OOM handling, page table invariants, and pointer safety.

## Mandatory rules
1. Allocation and deallocation must be paired and measurable.
2. Allocation failure must be handled explicitly without silent panics.
3. Paging changes require boot validation and memory-integrity tests.
4. No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.
5. Lock ordering (Virtual -> Paging -> Physical -> Swap) must be documented and respected.

## Prohibited
- Silent allocation failures or unhandled OOM conditions
- Double-free or use-after-free vulnerabilities
- Non-canonical virtual address mappings
- Bypassing W^X (Write XOR Execute) memory protection policies

## Required evidence
- Unit tests for physical frame, slab, and page table allocators
- Memory stress/exhaustion test execution report
- Memory leak and double-free detection log
- QEMU boot log with paging enabled

## Promotion criteria
| Level | Requirement |
|---|---|
| Proposed | Memory allocation policy ADR |
| Prototype | Unit tests for physical & virtual allocators |
| Integrated | Paging integrated into boot path with integration tests |
| Hardened | Exhaustion/stress tests, fuzzing, double-free checks |
| Supported | Proven memory safety and reproducible boot evidence |
