# Bolt's Journal

## 2026-07-12 - Hoisting State and Operations Out of Low-Level Pixel Loops
**Learning:** In resource-constrained `no_std` environments, doing high-frequency pixel-by-pixel framebuffer operations can easily bottleneck early boot and drawing sequences. Hoisting atomic checks, Option matches, bounds checks, and address arithmetic outside of inner pixel loops and using bulk direct writes (`core::ptr::copy` or pointer-walking) dramatically improves performance. For instance, hoisting out-of-loop matching in `fill_rect` reduces CPU-bound instruction counts per pixel, and optimizing `blit` to use bulk row copies (`core::ptr::copy`) acts as highly efficient SIMD memory transfers (`memmove`), eliminating standard pixel-by-pixel translation bottlenecks completely.
**Action:** Always inspect low-level rendering loops for redundant helper function calls (`putpixel`/`getpixel`), and optimize via hoisted state matching, contiguous pointer arithmetic, and bulk row copies.
