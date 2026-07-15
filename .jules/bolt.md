# Bolt Performance Journal

This document contains performance optimization learnings and actions for SigmaOS.

## 2026-07-12 - [SIMD String to_uppercase Bitwise Optimization & Rust Dynamic Trait Compatibility]
**Learning:**
1. A bitwise SIMD conversion function for ASCII string case switching (`simd_to_uppercase` using SSE2) can cause silent logical bugs if bitwise AND is improperly masked over non-lowercase ranges, leading to zeroing out all non-lowercase characters.
2. In single-threaded and `no_std` environments, generic parameters on trait methods prevent dyn compatibility (such as the `LlmBackend` stream interface). Transitioning these to use `&mut dyn FnMut` trait objects resolves compilation bottlenecks perfectly without sacrificing flexibility.
3. Sub-workspaces in Cargo containing their own `[workspace]` keys must be omitted from the top-level workspace members to prevent 'multiple workspace roots found' failures.
**Action:** Use bitwise inverse logical AND (`_mm_andnot_si128`) instead of direct bitwise AND when masking range selections in SIMD vector modifications. Ensure traits destined for dynamic dispatch are dyn-compatible.

## 2026-07-12 - [Hoisting State and Operations Out of Low-Level Framebuffer Pixel Loops]
**Learning:** In resource-constrained `no_std` environments, doing high-frequency pixel-by-pixel framebuffer operations can easily bottleneck early boot and drawing sequences. Hoisting atomic checks, Option matches, bounds checks, and address arithmetic outside of inner pixel loops and using bulk direct writes (`core::ptr::copy` or pointer-walking) dramatically improves performance. For instance, hoisting out-of-loop matching in `fill_rect` reduces CPU-bound instruction counts per pixel, and optimizing `blit` to use bulk row copies (`core::ptr::copy`) acts as highly efficient SIMD memory transfers (`memmove`), eliminating standard pixel-by-pixel translation bottlenecks completely.
**Action:** Always inspect low-level rendering loops for redundant helper function calls (`putpixel`/`getpixel`), and optimize via hoisted state matching, contiguous pointer arithmetic, and bulk row copies.

## 2026-07-14 - [Allocation-Free SemVer Comparison in Package Manager]
**Learning:** Executing recurrent string splitting and parsing within SemVer constraint checking allocates short-lived arrays or vectors (`Vec`) on the heap, introducing considerable heap fragmentation and slowing down topological sorting in deep dependency structures.
**Action:** Replace heap-allocating string parsing with an inline, lazy iterator mapping process (`s.split('.').map(...)`) and retrieve components directly to completely avoid dynamic vector allocations.

## 2026-07-14 - [DAG Topological Sort for Service Dependency Resolution]
**Learning:** When developing `.Jules` rules or `sigma_init` services, deep dependency trees can cause lock inversions or circular wait conditions if not sorted topologically. Kahn's Algorithm provides a linear-time (`O(V+E)`) approach to resolving dependencies, which guarantees safety and avoids deadlocks during system boot or test executions.
**Action:** When evaluating a set of dependencies (e.g., in a boot script or package resolution), always perform a topological sort (Kahn's) and fail fast on cycle detection rather than attempting a naive iterative traversal.

## 2026-07-14 - [Allocation-Free Case-Insensitive Substring Search in Registry]
**Learning:** Repetitively calling `.to_lowercase()` inside filter closures during package searches triggers frequent heap allocations for temporary String instances, causing substantial GC/allocator strain when searching large repositories.
**Action:** Implement an allocation-free case-insensitive substring search helper via `.as_bytes().windows().any(...)` to completely bypass dynamic String allocations.
