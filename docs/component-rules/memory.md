# Component Rules: Memory

## Component owner
memory-agent

## Upstream references
- Linux page allocation & paging
- xv6 physical memory allocator
- FreeBSD VM subsystem

## Purpose
Define allocation safety, paging contracts, and validation
for SigmaOS Memory components.

## Mandatory rules

1. Allocation and deallocation must be paired and measurable.
2. Allocation failure must be handled explicitly.
3. Paging changes require boot validation and memory-integrity tests.
4. No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.

## Prohibited

- Memory leaks or untracked raw allocations
- Unchecked allocation failures leading to unexpected panics
- Raw pointer exposure without safety documentation
- Status promotion without memory-integrity test evidence

## Required evidence

- Unit test report
- Allocation/exhaustion test report
- QEMU boot memory validation log
- Regression test for every memory fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
