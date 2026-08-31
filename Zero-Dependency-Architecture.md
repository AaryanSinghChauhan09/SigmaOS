# Zero Dependency Architecture — Implementation Guide

SigmaOS is built on a philosophy of **radical self-sufficiency**: the kernel and core libraries must not depend on any external C libraries, OS-provided runtime, or Rust `std`. This guide explains how this is achieved in practice.

## Philosophy

> "Every dependency is a trust boundary. Every trust boundary is an attack surface."

SigmaOS eliminates entire categories of supply-chain attacks by implementing core functionality from scratch in Rust, with zero `extern crate std` and zero `libc` linkage in the kernel.

---

## Dependency Hierarchy

```
┌────────────────────────────────────────────────────┐
│                  User Applications                 │
│           (may use std, glibc, etc.)               │
├────────────────────────────────────────────────────┤
│              SigmaOS userland (sigmalib)            │
│        (Rust + selective POSIX compatibility)       │
├────────────────────────────────────────────────────┤
│                  sigma-syscall ABI                  │
│         (stable syscall interface, C-FFI safe)      │
├────────────────────────────────────────────────────┤
│              SigmaOS Kernel (no_std)                │
│         NO external dependencies below here         │
│                                                    │
│  klib::Vec    klib::String   klib::BTreeMap         │
│  klib::Arc    klib::Mutex    klib::RwLock           │
│  klib::Rc     klib::Box      klib::HashMap          │
└────────────────────────────────────────────────────┘
              ↑ Everything in this box is
              ↑ implemented from scratch in Rust
```

---

## Custom `klib` — The Kernel Standard Library

All `std`-like functionality is re-implemented in `src/klib/`:

| `std` Type | `klib` Equivalent | Notes |
|-----------|-------------------|-------|
| `Vec<T>` | `klib::Vec<T>` | Uses custom slab/buddy allocator |
| `String` | `klib::String` | UTF-8, no heap fragmentation |
| `BTreeMap<K,V>` | `klib::BTreeMap<K,V>` | B-tree, no allocator dependency |
| `Arc<T>` | `klib::Arc<T>` | Atomic ref count, no weak |
| `Mutex<T>` | `klib::Mutex<T>` | Spin-lock based, no POSIX futex |
| `RwLock<T>` | `klib::RwLock<T>` | Reader-biased spin lock |
| `Box<T>` | `klib::Box<T>` | Allocates via global kernel allocator |
| `HashMap<K,V>` | `klib::HashMap<K,V>` | Robin Hood hashing |

---

## Eliminated External Dependencies

### Removed: `libc`
- **What was using it**: Memory allocation, printf-style logging, POSIX thread primitives
- **Replacement**: Custom allocator (`src/klib/buddy_allocator.rs`), custom serial/VGA logging, custom spinlock

### Removed: `std::collections`
- **Replacement**: `klib::Vec`, `klib::BTreeMap`, `klib::HashMap`

### Removed: OpenSSL / ring / RustCrypto
- **Replacement**: Custom crypto primitives in `src/crypto/`:
  - AES-256-GCM: `src/crypto/aes_gcm.rs`
  - ChaCha20-Poly1305: `src/crypto/chacha20.rs`
  - SHA-256/512: `src/crypto/sha.rs`
  - Argon2id: `src/crypto/argon2.rs`
  - X25519 ECDH: `src/crypto/x25519.rs`
  - Ed25519 signatures: `src/crypto/ed25519.rs`

### Removed: `log` crate
- **Replacement**: `src/klib/log.rs` — direct serial output with log levels

### Removed: `alloc` crate (partially)
- **Replacement**: All allocation goes through `klib::GlobalAllocator` which uses the buddy allocator

---

## Cargo.toml Policy

```toml
[profile.kernel]
# No std, no libc
panic = "abort"
opt-level = "s"
lto = true
codegen-units = 1

[dependencies]
# RULE: No external crates in kernel build
# All functionality must come from src/klib or src/crypto
```

Running `cargo tree --edges features` should show **zero** non-workspace crates for the kernel library target.

---

## Verifying Zero External Dependencies

```bash
# Check the kernel library target has no external crate deps
cargo tree --target x86_64-unknown-none --lib 2>&1 | grep -v "sigma" || echo "✅ Zero external deps"

# Check for any libc usage
grep -r "extern crate libc" src/ && echo "❌ libc found" || echo "✅ No libc"

# Check for std usage in kernel files
grep -r "use std::" src/ --include="*.rs" && echo "❌ std found" || echo "✅ No std in kernel"
```

---

## Trade-offs and Decisions

| Decision | Trade-off | Rationale |
|----------|-----------|-----------|
| Custom Vec | More code to maintain | Full control, no hidden allocations |
| Custom crypto | Potential for bugs | Audit once, never update supply chain |
| No log crate | Less ecosystem tooling | Log is a leaf dependency — easy to reimplement |
| Custom allocator | Performance tuning effort | Kernel needs predictable allocation latency |

---

## Contributing

When adding new kernel code:
1. **Never** `use std::` in `src/` kernel files
2. **Always** prefer `klib::` types over reinventing inline
3. If a new data structure is needed, add it to `klib/` first
4. All `unsafe` code must have a `// SAFETY:` comment
5. Run `cargo clippy -- -D clippy::std_instead_of_core` to catch accidental `std` usage

---

## See Also

- [klib API Reference](klib-API-Reference.md)
- [Custom Crypto Primitives](Crypto-Primitives.md)
- [Memory Management](Memory-Management.md)
- [Kernel Architecture](Architecture.md)
