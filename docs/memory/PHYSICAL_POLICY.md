# SigmaOS Physical Memory Management Policy

## Overview

The physical memory manager (`src/memory/physical.rs`, `src/memory/frame.rs`) is responsible for managing physical memory frame discovery, region reservations, allocation, and deallocation across SigmaOS.

---

## Core Guarantees & Invariants

1. **Race-Free Frame Allocation**: Physical frame allocations and free operations are atomic or synchronized via internal mutex structures.
2. **Defined Frame Reuse Policy**: Freed physical frames are returned to the free pool / bitmap and can be safely reallocated to subsequent callers without dangling references.
3. **Typed Exhaustion Errors**: Physical frame exhaustion does not panic or trigger undefined behavior; it returns a explicit typed error (`MemoryError::OutOfMemory` or `FrameAllocationError::Exhausted`).
4. **Reserved Region Protection**: Physical memory regions marked as reserved (e.g., BIOS/UEFI firmware areas, ACPI tables, kernel executable text/data) are strictly protected and excluded from allocation pools.
5. **Double-Free Protection**: Freeing an already freed physical frame or an unallocated frame is detected and rejected via reference counting / bitmap assertions, returning a typed error (`FrameAllocationError::DoubleFree`).
6. **Lost-Wakeup Prevention**: Wait queues associated with frame reclamation notify blocked allocators atomically upon frame release.

---

## Implementation Specifications

- **Source Implementation**: `src/memory/physical.rs`, `src/memory/frame.rs`
- **Unit & Concurrency Tests**: `tests/memory/physical.rs`
- **Page Frame Size**: 4096 bytes (4 KiB) standard page size.
- **Physical Memory Map Parsing**: E820 / EFI memory descriptors map reserved and usable physical regions during kernel boot.

---

## Enforcement & Verification

- `cargo check --lib`
- `rustc --test --edition=2021 tests/memory/physical.rs -o build/test_phys && ./build/test_phys`
