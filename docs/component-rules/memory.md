# Component Rules: Memory

## Component owner
memory-agent

## Upstream references
- xv6 physical frame allocator
- Linux 4-level paging and folio allocator
- FreeBSD VM subsystem

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS physical frame allocation, paging, and kernel heap management.

## Mandatory rules

1. Allocation and deallocation must be paired and measurable.
2. Allocation failure must be handled explicitly.
3. Paging changes require boot validation and memory-integrity tests.
4. No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.
5. Virtual memory mappings must enforce page-level W^X (write XOR execute) permissions.
6. Kernel memory pools must prevent fragmentation and track peak consumption.
7. Frame exhaustion scenarios must trigger structured out-of-memory handling.

## Prohibited

- Unchecked raw pointers across module boundaries
- Silent ignoring of memory allocation failures
- Double-freeing physical frames or virtual address regions
- Status promotion without runtime allocation and fuzzing evidence

## Required evidence

- Unit test report
- Integration test report
- Allocation, exhaustion, and fuzz tests
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
