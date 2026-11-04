# Component Rules: Memory

## Component owner
memory-agent

## Upstream references
- xv6 memory allocator
- Linux paging subsystem
- seL4 capability-based page tables

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Memory management and paging components.

## Mandatory rules

1. Allocation and deallocation must be paired and measurable.
2. Allocation failure must be handled explicitly.
3. Paging changes require boot validation and memory-integrity tests.
4. No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.

## Prohibited

- Memory leaks and unmeasured heap growth
- Silent swallowing of out-of-memory (OOM) errors
- Exposing raw pointers across module boundaries without safety invariant
- Unchecked page table modifications

## Required evidence

- Frame allocator unit tests
- Paging integrity tests
- Allocation, exhaustion, and fuzz tests
- QEMU boot validation log
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests and memory fuzzing |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
