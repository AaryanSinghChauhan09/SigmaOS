# 💾 AI Agents Allocation Management Specification (`docs/AI_AGENTS_ALLOCATION_MANAGEMENT.md`)

This specification defines memory allocation architectures, zero-allocation hot path invariants, allocator primitives, and pool memory concurrency policies for autonomous AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) in SigmaOS.

---

## 1. Native Allocator Architecture (`src/klib/`)

AI agents utilize native zero-dependency allocators:
- **Global Allocator (`src/klib/custom_allocator.rs`)**: `#![no_std]` custom heap allocator interface handling dynamic kernel and userspace requests.
- **SLUB-Style Slab Allocator (`SlabAllocator`)**: $O(1)$ constant-time allocation for fixed-size agent data objects.
- **Lookaside Lists (`LookasideList`)**: Pre-allocated per-CPU caches reducing lock contention.
- **Buddy Allocator (`src/klib/buddy_allocator.rs`)**: Power-of-two order frame allocator managing physical memory blocks.

---

## 2. Zero-Allocation Hot Path Principles

- **Pre-Allocated Static Ring Buffers (`src/klib/ring_buffer.rs`)**: Agent telemetry and IPC channels operate on fixed capacity buffers without heap re-allocations.
- **Direct Slice Operations**: `SigmaString` and `SigmaVec` avoid intermediate clones by performing direct `copy_from_slice` memory transfers.

---

## 3. IRQL Paging Invariants & Allocation Concurrency

- **`NonPagedPool` vs `PagedPool`**:
  - `NonPagedPool` memory allocations are locked in physical RAM and permitted at all IRQL levels (`IRQL >= PassiveLevel`).
  - `PagedPool` memory is pageable to backing storage. Allocating or accessing `PagedPool` at `IRQL >= DispatchLevel` triggers an unrecoverable `DoubleFault` exception (`PAGE_FAULT_IN_NONPAGED_AREA`).
- **Locking Primitives**: Allocation locks employ ticket spinlocks (`TicketSpinlock`) or lockless reader concurrency (`SequenceLock`).

---

## 4. AI Agent Allocation Responsibilities

- **⚡ Bolt**: Profiles allocation latency, monitors memory fragmentation, and tunes slab block caches.
- **🎨 Palette**: Manages framebuffer and GUI display buffer allocations in Zenith desktop compositing.
- **🛡️ Sentinel**: Audits allocation boundaries, enforces zeroization of deallocated secret buffers, and detects memory leak anomalies.
