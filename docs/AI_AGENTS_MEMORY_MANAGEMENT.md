# 🧠 AI Agents Memory Management Specification (`docs/AI_AGENTS_MEMORY_MANAGEMENT.md`)

This specification defines the memory management architecture, allocation policies, and security invariants for autonomous AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) operating on SigmaOS.

---

## 1. Zero-Allocation Hot Path Policy (`src/klib/`)

To guarantee deterministic sub-microsecond latency:
- Critical agent loops must operate within pre-allocated static or ring buffers (`src/klib/ring_buffer.rs`).
- Dynamic heap allocations in critical agent hot paths are strictly prohibited.
- `SigmaString` and `SigmaVec` primitives operate via `copy_from_slice` without intermediate heap cloning.

---

## 2. Allocator Architecture & Primitives

Agents interface with native, zero-dependency allocators in `src/klib/`:
- **Slab Allocator (`SlabAllocator`)**: Fixed-size block allocation for frequent agent message objects.
- **Buddy Allocator (`src/klib/buddy_allocator.rs`)**: Power-of-two page frame allocation for large buffer pools.
- **Custom Global Allocator (`src/klib/custom_allocator.rs`)**: Integrated `no_std` kernel allocator fallback.

---

## 3. Kernel Pool Memory Rules & IRQL Constraints

Agent processes interfacing with kernel structures must obey IRQL paging invariants:
- **`NonPagedPool`**: Guaranteed physical RAM backing. Required for allocations at `IRQL >= DispatchLevel`.
- **`PagedPool`**: Pageable virtual memory. Accessing `PagedPool` at `IRQL >= DispatchLevel` triggers an immediate `DoubleFault` exception (`PAGE_FAULT_IN_NONPAGED_AREA`).

---

## 4. Memory Hardening, Scrubbing & Security

- **Secure Memory Zeroization**: Deallocated agent secrets and capability tokens are overwritten immediately upon release.
- **ASLR & Stack Protection**: Agent binaries link with address space layout randomization and canary defense.
