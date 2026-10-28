# Sentinel's Journal - Critical Learnings Only

## 2026-10-27 - Zeroize Sensitive Memory Buffers
**Learning:** Using zeroize wrappers on cryptographic signature and key buffers prevents memory inspection during core dumps or kernel panics.
**Action:** Wrap all sensitive key material in Zeroize memory containers.
