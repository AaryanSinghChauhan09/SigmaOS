# Bolt's Performance Journal

## 2026-07-12 - [SIMD String to_uppercase Bitwise Optimization & Rust Dynamic Trait Compatibility]
**Learning:**
1. A bitwise SIMD conversion function for ASCII string case switching (`simd_to_uppercase` using SSE2) can cause silent logical bugs if bitwise AND is improperly masked over non-lowercase ranges, leading to zeroing out all non-lowercase characters.
2. In single-threaded and `no_std` environments, generic parameters on trait methods prevent dyn compatibility (such as the `LlmBackend` stream interface). Transitioning these to use `&mut dyn FnMut` trait objects resolves compilation bottlenecks perfectly without sacrificing flexibility.
3. Sub-workspaces in Cargo containing their own `[workspace]` keys must be omitted from the top-level workspace members to prevent 'multiple workspace roots found' failures.

**Action:**
Use bitwise inverse logical AND (`_mm_andnot_si128`) instead of direct bitwise AND when masking range selections in SIMD vector modifications. Ensure traits destined for dynamic dispatch are dyn-compatible.
