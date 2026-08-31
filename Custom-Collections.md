# SigmaOS Custom Zero-Dependency Collections

SigmaOS aims to achieve ultimate self-sufficiency and reduce dependencies on predefined libraries and standard templates. To support this architecture, we have implemented custom, zero-dependency, `#![no_std]` compliant collections directly utilizing raw pointer allocations.

## Custom Dynamic Vector (`Vec<T>`)

Located in [`src/klib/vec.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/klib/vec.rs), this structure replaces `std::vec::Vec`.

### Key Features

*   **Manual Allocation**: Directly invokes `alloc`, `dealloc`, and `realloc` from the allocator shim interface.
*   **Dynamic Growth**: Implements a geometric progression growth strategy (doubles capacity on exhaustion).
*   **Custom Iteration**: Implements `IntoIterator` for ownership-consuming, reference-based (`VecIter`), and mutable reference-based (`VecIterMut`) iteration.
*   **Closure-based Queries**: Features a custom `contains` method utilizing predicate closures to support complex types without enforcing generic traits.

### Usage Example

```rust
use crate::klib::Vec;

let mut v = Vec::new();
v.push(10);
v.push(20);

// Safe iteration over references
for val in &v {
    println!("{}", val);
}
```

***

## Custom Hash Map (`HashMap<K, V>`)

Located in [`src/klib/hashmap.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/klib/hashmap.rs), this structure replaces `std::collections::HashMap`.

### Key Features

*   **Bucket Array Layout**: Backed by a custom `Vec` containing list buckets to handle collisions via chaining.
*   **Custom Hashing Interface**: Employs a custom FNV-inspired hash driver avoiding std-provided hash configurations.
*   **Entry API**: Supports the standard entry API pattern (`Occupied` vs `Vacant`) for convenient lookup-and-insert cycles.
