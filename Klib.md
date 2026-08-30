# Klib — Native Collections

`src/klib/` provides zero-`std` replacements for all standard collections and memory primitives.

## Why klib?

SigmaOS targets `#[no_std]` bare-metal environments. `std::collections` requires OS services (heap allocation via `std::alloc`, which in turn needs OS memory management). klib provides:

1.  Collections that work directly with SigmaOS's own allocators
2.  No implicit libc or OS system calls
3.  Full control over memory layout for cache efficiency

## Modules

### `vec.rs` — Dynamic Array

Drop-in for `std::vec::Vec`:

```rust
use crate::klib::Vec;
let mut v: Vec<u32> = Vec::new();
v.push(1);
v.push(2);
assert_eq!(v.len(), 2);
```

Implements: `push`, `pop`, `len`, `is_empty`, `iter`, `iter_mut`, `Deref`, `IntoIterator`, `Index`/`IndexMut`.

### `hashmap.rs` — Hash Map

Robin Hood open-addressing hash map:

```rust
use crate::klib::HashMap;
let mut map = HashMap::new();
map.insert("key", 42u32);
assert_eq!(map.get("key"), Some(&42));
```

### `btreemap.rs` — Ordered Map

B-tree map for sorted key access:

```rust
use crate::klib::BTreeMap;
let mut m = BTreeMap::new();
m.insert(3u32, "three");
m.insert(1u32, "one");
// Iteration is in sorted order
```

### `hashset.rs` — Hash Set

```rust
use crate::klib::HashSet;
let mut s = HashSet::new();
s.insert("alpha");
s.insert("beta");
assert!(s.contains("alpha"));
```

### `vecdeque.rs` — Double-Ended Queue

Ring-buffer based deque for O(1) push\_front and push\_back:

```rust
use crate::klib::VecDeque;
let mut dq: VecDeque<u32> = VecDeque::new();
dq.push_back(1);
dq.push_front(0);
assert_eq!(dq.pop_front(), Some(0));
```

### `buddy_allocator.rs` — Kernel Heap

Power-of-two buddy system for the kernel heap:

```rust
// Used internally by klib types
// Manages physical frames in 2^n blocks
// Merges freed buddies to avoid fragmentation
```

### `hash.rs` — Hash Trait

Provides the `Hash` trait for custom types used as map keys.

***

## Migration Guide (std → klib)

| Replace | With |
|---|---|
| `std::vec::Vec<T>` | `crate::klib::Vec<T>` |
| `std::collections::HashMap<K,V>` | `crate::klib::HashMap<K,V>` |
| `std::collections::BTreeMap<K,V>` | `crate::klib::BTreeMap<K,V>` |
| `std::collections::HashSet<T>` | `crate::klib::HashSet<T>` |
| `std::collections::VecDeque<T>` | `crate::klib::VecDeque<T>` |
| `std::string::String` | `crate::klib::String` (or `alloc::string::String`) |
| `format!("{}", x)` | Use klib string builder or manual serialization |

***

## Performance Notes

*   `Vec`: realloc doubles capacity (amortized O(1) push)
*   `HashMap`: Robin Hood probing; load factor 75%
*   `BTreeMap`: B-tree with branching factor 8; O(log n) all operations
*   `buddy_allocator`: O(log n) alloc/free with O(1) coalescing
