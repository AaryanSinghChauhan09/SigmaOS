# 🛡️ Sentinel's Security Journal

This journal chronicles security-centric insights, threat mitigation strategies, and critical vulnerabilities resolved during the hardening of the SigmaOS microkernel.

---

## 2026-03-02 - Unchecked Integer Overflows in VFS Writes
**Vulnerability:** Unchecked file offset and size increments allow maliciously crafted writes to trigger integer overflows, leading to out-of-bounds array indexing or memory exhaustion.
**Learning:** Monolithic filesystems often rely on loose validation checks. A microkernel VFS must enforce strict, compile-time checked arithmetic before committing any state changes.
**Prevention:** Use `.checked_add()` to compute new sizes and offsets, returning `NoSpace` or `InvalidFd` errors immediately if an overflow condition is detected.

## 2026-03-01 - Atomic Structs in Cloneable Types
**Vulnerability:** Attempting to derive `Clone` on structs containing non-cloneable thread-safe atomics (such as `AtomicBool` in privilege mitigation pledges) prevents compilation.
**Learning:** Rust's standard `#[derive(Clone)]` is shallow. When a security-critical struct houses an atomic thread state, `Clone` must be manually implemented.
**Prevention:** Provide a custom, manual `Clone` implementation that loads the atomic value with explicit ordering constraints (e.g., `Ordering::SeqCst`) and re-initializes it.
