# ADR-002: Frame Allocator and Paging Diagnostics

## Component
Memory

## Current gap
Frame allocation lacks leak detection and boundary diagnostics.

## Upstream study
xv6 `kalloc.c` uses physical page bitmap/freelist; Linux uses buddy allocator.

## Improvement
Implement bitmap-backed physical frame allocator with leak tracking in Rust.

## Scope
- Page frame allocation / deallocation
- Page table walk verification
- Leak detection diagnostics

## Validation
- Allocation tests
- QEMU memory trace
