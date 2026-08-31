# SigmaOS Custom Library Implementations

A core design principle of SigmaOS is reducing dependency on predefined libraries (libc, standard library, etc.) by implementing custom versions of critical components.

## Philosophy

> "We build what we need, exactly as we need it, without excess dependencies."

This approach provides:
- **Security**: Every line of code is auditable
- **Performance**: No generality overhead
- **Portability**: No external dependency constraints
- **Control**: Full customization of behavior

## Custom Data Structures (`src/klib/`)

### `klib::vec::Vec<T>`
Custom vector implementation without `std::vec::Vec`:
- No heap allocator dependency (uses custom page allocator)
- Manual memory management with explicit capacity control
- `iter()`, `iter_mut()`, `IntoIterator` implementations
- No panics: all operations return `Option<T>` or `Result`

### `klib::collections::SigmaHashMap<K, V>`
Hash map without `std::collections::HashMap`:
- Robin Hood hashing
- Fixed-capacity buckets
- Open addressing collision resolution
- No-std compatible

### `klib::string::SigmaString`
String type without `std::string::String`:
- Fixed-size inline storage
- UTF-8 validated
- Stack-allocated for small strings

## Custom Algorithms (`src/klib/`)

### Sorting
- `introsort` - O(n log n) worst-case hybrid sort
- `radix_sort` - O(n) integer sort
- No dependency on `std::cmp::Ord` trait magic

### Hashing  
- `sigma_hash::fnv1a` - Fast non-cryptographic hash
- `sigma_hash::xxhash` - High-performance hash
- `sigma_hash::siphash` - DoS-resistant hash

## Custom Crypto (`src/crypto/`)

All cryptographic primitives implemented from scratch:
- AES-256-GCM
- ChaCha20-Poly1305
- BLAKE3 hashing
- Ed25519 signatures
- X25519 key exchange

No dependency on OpenSSL, ring, or other crypto crates.

## Custom I/O (`src/kernel/`)

### `kernel::ipc`
Inter-process communication without POSIX:
- Sovereign pipes with splice operation
- Structured message passing
- Zero-copy ring buffers

### `kernel::kqueue`
Event notification without libuv/tokio:
- BSD kqueue-compatible API
- No async runtime required
- Kernel-level polling

## Comparison Table

| Standard Library | SigmaOS Custom | Location |
|----------------|----------------|----------|
| `Vec<T>` | `klib::vec::Vec<T>` | `src/klib/vec.rs` |
| `HashMap<K,V>` | `SigmaHashMap<K,V>` | `src/klib/collections.rs` |
| `String` | `SigmaString` | `src/klib/string.rs` |
| `epoll` | `Kqueue` | `src/kernel/kqueue.rs` |
| `malloc` | `SlabCache` | `src/kernel/slab_alloc.rs` |
| `pthread` | `SigmaThread` | `src/thread/` |
| `socket` | `SigmaSocket` | `src/network/` |
