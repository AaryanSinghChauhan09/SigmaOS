# ADR 0002: Safe, Observable, and Testable Memory Subsystem Architecture

## Context and Problem Statement

SigmaOS requires a deterministic, safe, observable, and testable memory subsystem. Historically, OS kernels suffer from memory leaks, race conditions during physical frame allocation, unvalidated page table mutations, and unexpected panics caused by unbounded kernel allocations under memory exhaustion.

SigmaOS explicitly requires that unit-tested models connect to a validated boot-to-shell path, turning model abstractions into an evidence-backed kernel memory subsystem.

## Decision Drivers

- **Memory Safety**: Eliminate physical frame leaks, double-frees, frame reuse races, and undefined behavior.
- **Strict Isolation**: Enforce kernel vs. user address space separation and W^X (Write XOR Execute) page protection policies.
- **Observable OOM**: Treat Out-Of-Memory (OOM) as a designed error state returning typed errors (`Err(MemoryError::OutOfMemory)`) rather than triggering kernel panics.
- **Fail-Safe Swap Subsystem**: Provide race-free swap-out/swap-in queuing with bounded metadata overhead.
- **Evidence-Based Maintenance**: Require unit tests, integration tests, and boot-to-shell validation for all memory-affecting changes.

## Architectural Design

### 1. Authoritative 11-Stage Memory Lifecycle
SigmaOS establishes one single canonical lifecycle contract across all memory components:

```text
boot -> physical allocator init -> paging init -> virtual memory init ->
allocation -> mapping -> protection -> swap -> reclaim -> free -> cleanup
```

### 2. Component Boundaries
- **Physical Memory Manager (`src/memory/physical.rs`, `src/memory/frame.rs`)**:
  Manages physical memory discovery, region reservation, bounded frame allocation bitmap, double-free validation, and frame reuse.
- **Paging & Page Tables (`src/mm/paging.rs`, `src/mm/page_table.rs`)**:
  Manages 4-level x86_64 page table translation, canonical address enforcement, mapping capacity bounds, and TLB invalidation hooks.
- **Virtual Memory Manager (`src/mm/virtual.rs`)**:
  Manages Virtual Memory Areas (VMAs), permissions enforcement (Read, Write, Execute, W^X policy), and user/kernel address space isolation.
- **Swap Manager (`src/swap/core.rs`, `src/swap/enqueue.rs`)**:
  Manages priority-queued swap slots, zram/zswap compression stages, and failure-safe swap enqueueing.

## Concurrency & Locking Discipline

- **Physical Allocator**: Mutex-protected or lock-free bitmap search.
- **Page Tables**: Locking hierarchy ensures Page Directory / Page Table locks precede VMA locks.
- **Swap Queue**: Thread-safe mutex queue with lost-wakeup prevention via atomic state flags.

## Status

**Accepted** - Implemented and validated across unit and integration test suites.
