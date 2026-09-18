# AI Agent C Programming Language Dependency Reduction Guidelines

## Purpose
These guidelines define operational rules, code refactoring patterns, and safety constraints for AI coding agents eliminating C language dependencies and C FFI blocks in SigmaOS.

---

## Directives for AI Agents

1. **Eliminate C FFI Blocks**:
   - Refactor `extern "C" { fn alloc... }` blocks into pure Rust safe memory allocation patterns using `alloc::vec::Vec` or `klib` allocators.
   - Never introduce new `.c` or `.h` source files into the repository.

2. **RAII and Memory Safety**:
   - Replace manual C pointer management with Rust RAII types (`Box`, `Vec`, `String`, `Arc`).
   - Use `core::mem::forget` or `ManuallyDrop` only when interfacing with raw hardware registers.

3. **Code Conversion Pattern: C FFI Allocator to Safe Rust**:
```rust
// Legacy C FFI pattern (TO BE ELIMINATED):
// extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }

// Safe Pure Rust Replacement:
use alloc::vec::Vec;

pub struct SafeSovereignBuffer<T> {
    data: Vec<T>,
}

impl<T> SafeSovereignBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self { data: Vec::with_capacity(capacity) }
    }
}
```

4. **Testing and Verification**:
   - Run `./run_sigma_tests.sh` to confirm safe Rust code conversion passes all 13 test stages.

---

## Related Files
- `docs/AI_AGENT_C_LANGUAGE_ELIMINATION_ARCHITECTURE.md`
- `wiki/AI_AGENT_C_LANGUAGE_ELIMINATION.md`
- `src/klib/`
