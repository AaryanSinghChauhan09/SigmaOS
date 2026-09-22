# AI Agent Memory Management Architecture in SigmaOS

This document serves as the technical reference for AI agents working on memory management, allocation strategies, zero-copy IPC pipelines, and paging algorithms within SigmaOS.

---

## 🏛️ 1. Subsystem Architecture

SigmaOS partitions memory management into distinct layers:

```
+-----------------------------------------------------------------------+
| Userland & Package Subsystems (src/desktop/, src/sigpkg/)              |
| Uses std / alloc::vec::Vec, alloc::boxed::Box                          |
+-----------------------------------------------------------------------+
| Sovereign Kernel Library (src/klib/)                                  |
| #![no_std] zero-allocation primitives, stack buffers, FNV-1a hashing  |
+-----------------------------------------------------------------------+
| Physical & Virtual Memory Management (src/memory/)                    |
| PMM/VMM Bitmap, Buddy Allocator, Slab Object Cache, CoW Page Faults  |
+-----------------------------------------------------------------------+
```

---

## ⚡ 2. Zero-Allocation Best Practices for AI Agents

1. **Stack Conversions (`src/klib/conversion.rs`)**
   - Use `parse_u64_str` and `u64_to_hex_str_stack` for numeric conversions without heap allocations.

2. **In-Place JSON Formatting (`src/klib/json.rs`)**
   - When extending `SovereignJsonValue`, implement `append_json_string(&self, out: &mut String)` to format JSON into a pre-allocated buffer.

3. **Zero-Copy Page Splice (`src/distro/wiki_ideas_implementation.rs`)**
   - For high-throughput I/O pipelines, use zero-copy page splicing (`pipe_splice`) rather than copying buffers into userland memory.

---

## 🔒 3. Memory Security & Hardening

- **Amnesic Memory Scrubbing (`TailsAmnesicEngine` in `src/distro/missing_distro_innovations.rs`)**
  - Zeroizes session pages (`wipe_all_memory_on_shutdown`) upon system exit.
- **Resource Quotas & Hardened Guard Pages**
  - Use `HardenedGuardPageAllocator` in `src/memory/resource_allocator.rs` to trap out-of-bounds buffer overflows.
