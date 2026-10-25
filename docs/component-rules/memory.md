# Component Rules: Memory

## Component owner
memory-agent

## Upstream references
- xv6 physical memory allocator
- Linux paging & slab manager
- FreeBSD VM subsystem

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS Memory components.

## Mandatory rules

1. Allocation and deallocation must be paired and measurable.
2. Allocation failure must be handled explicitly without kernel crash loops.
3. Paging changes require boot validation and memory-integrity tests.
4. No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.
5. All physical frame allocators must prevent double-free and reserved range corruption.

## Prohibited

- Silent OOM memory corruption
- Raw pointer exposure across safe Rust module boundaries
- Unbounded allocation in `#![no_std]` paths
- Memory leak patterns in kernel slab/buddy allocators

## Required evidence

- Frame allocator unit tests
- Paging and virtual address space isolation integration tests
- OOM exhaustion stress test log
- Memory leak audit report

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and VM mapping gap analysis |
| Prototype | Frame allocator unit tests |
| Integrated | Paging validation and QEMU boot integration |
| Hardened | OOM fuzzing, heap isolation, safety audit |
| Supported | Reproducible memory stability evidence under high load |
