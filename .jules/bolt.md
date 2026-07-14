# Bolt's Performance Journal

## 2026-07-12 - [SIMD String to_uppercase Bitwise Optimization & Rust Dynamic Trait Compatibility]

**Learning:**

1. A bitwise SIMD conversion function for ASCII string case switching (`simd_to_uppercase` using SSE2) can cause silent logical bugs if bitwise AND is improperly masked over non-lowercase ranges, leading to zeroing out all non-lowercase characters.
2. In single-threaded and `no_std` environments, generic parameters on trait methods prevent dyn compatibility (such as the `LlmBackend` stream interface). Transitioning these to use `&mut dyn FnMut` trait objects resolves compilation bottlenecks perfectly without sacrificing flexibility.
3. Sub-workspaces in Cargo containing their own `[workspace]` keys must be omitted from the top-level workspace members to prevent 'multiple workspace roots found' failures.


**Action:**
Use bitwise inverse logical AND (`_mm_andnot_si128`) instead of direct bitwise AND when masking range selections in SIMD vector modifications. Ensure traits destined for dynamic dispatch are dyn-compatible.

## 2026-07-12 - Hoisting State and Operations Out of Low-Level Pixel Loops

**Learning:** In resource-constrained `no_std` environments, doing high-frequency pixel-by-pixel framebuffer operations can easily bottleneck early boot and drawing sequences. Hoisting atomic checks, Option matches, bounds checks, and address arithmetic outside of inner pixel loops and using bulk direct writes (`core::ptr::copy` or pointer-walking) dramatically improves performance. For instance, hoisting out-of-loop matching in `fill_rect` reduces CPU-bound instruction counts per pixel, and optimizing `blit` to use bulk row copies (`core::ptr::copy`) acts as highly efficient SIMD memory transfers (`memmove`), eliminating standard pixel-by-pixel translation bottlenecks completely.
**Action:** Always inspect low-level rendering loops for redundant helper function calls (`putpixel`/`getpixel`), and optimize via hoisted state matching, contiguous pointer arithmetic, and bulk row copies.

## 2026-07-14 - Allocation-Free SemVer Comparison in Package Manager

**Learning:** Executing recurrent string splitting and parsing within SemVer constraint checking allocates short-lived arrays or vectors (`Vec`) on the heap, introducing considerable heap fragmentation and slowing down topological sorting in deep dependency structures.
**Action:** Replace heap-allocating string parsing with an inline, lazy iterator mapping process (`s.split('.').map(...)`) and retrieve components directly to completely avoid dynamic vector allocations.

## 2026-07-14 - DAG Topological Sort for Service Dependency Resolution

**Learning:** When implementing an init system (`sigma_init`) that models services as a Directed Acyclic Graph, using a Kahn's algorithm with in-degree counting is significantly more performant and debuggable than recursive DFS-based approaches. Kahn's algorithm naturally detects cycles (when the sorted output length doesn't match node count) and provides a deterministic, breadth-first service start order. Crucially, the queue-based approach avoids stack overflows on deeply nested dependency chains that DFS would cause in `no_std` environments.
**Action:** Prefer Kahn's in-degree algorithm over DFS for topological sorting in service DAGs. Always check `order.len() != nodes.len()` as the canonical cycle detection guard.

## 2026-07-14 - Zero-Copy Ring Buffers for Audio Latency

**Learning:** For `sigma_audio`, using pre-allocated `Vec<f32>` ring buffers with head/tail pointer arithmetic eliminates all heap allocation during audio playback. The key insight is that audio streams are inherently bounded (e.g., 4096 samples per buffer at 48kHz), so a fixed-size ring buffer avoids the `Vec::push` reallocation overhead that causes audible glitches in real-time audio paths.
**Action:** Always pre-allocate audio buffers to the maximum expected size and use modular index arithmetic (`head % capacity`) instead of dynamic resizing.
