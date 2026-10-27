# Component Rules: Memory

## Component owner
memory-agent

## Upstream references
- Linux mm (buddy allocator & page tables)
- xv6 physical memory management
- FreeBSD VM subsystem

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS Memory management components.

## Mandatory rules

1. Allocation and deallocation must be paired and measurable.
2. Allocation failure must be handled explicitly.
3. Paging changes require boot validation and memory-integrity tests.
4. No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.

## Prohibited

- Unbounded memory allocations
- Double-freeing or unrecorded memory leaks
- Raw pointer exposure without documented safety invariants
- Unvalidated page table modifications

## Required evidence

- Memory unit test suite report
- Exhaustion and boundary test log
- QEMU paging and boot integrity evidence
- Regression test for every memory management fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
