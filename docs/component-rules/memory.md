# Component Rules: Memory

## Component owner
memory-agent

## Upstream references
- Linux MM (Buddy allocator, SLUB, Page tables, Swap)
- xv6 physical frame allocator
- FreeBSD VM Subsystem

## Purpose
Define allocation safety, paging contracts, swap constraints, and memory isolation rules for SigmaOS memory management.

## Mandatory rules

1. Allocation and deallocation must be paired, race-free, and measurable.
2. Allocation failure (OOM) must be handled explicitly as a designed state, rather than causing an accidental kernel panic.
3. Paging changes require boot validation and memory-integrity tests.
4. No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.
5. Lock ordering (`Virtual` -> `Paging` -> `Physical` -> `Swap`) must be strictly maintained to prevent deadlocks.
6. User and kernel address spaces must maintain strict W^X (Write XOR Execute) isolation.

## Prohibited

- Unbounded heap or physical frame allocations in `no_std` kernel paths
- Double-free or unmapped frame dereferences
- Cross-boundary raw pointer leaks without safety justification
- Status promotion without memory stress test evidence

## Required evidence

- Frame allocator unit test reports
- Paging table mapping test reports
- Memory exhaustion / OOM handling test logs
- QEMU boot memory validation evidence

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests for allocator and paging |
| Integrated | QEMU boot validation with active paging and allocation |
| Hardened | Stress/fuzz testing under low-memory conditions |
| Supported | Multi-architecture memory layout and hardware evidence |
