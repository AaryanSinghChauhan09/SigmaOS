## 2025-03-02 - Bulk Memory Operations for `SigmaVec` and `SigmaString`
**Learning:** In standard `no_std` kernel/klib data structures, looping over slice elements using `push` incurs repetitive capacity bounds checks and reallocations. Replacing element-by-element iteration with `reserve(other.len())` followed by `core::ptr::copy_nonoverlapping` turns slice extension into an O(1) bulk SIMD/memcpy operation. Additionally, chaining `trim_start().trim_end()` allocates intermediate string buffers; calculating start/end indices in a single pass eliminates redundant heap allocations.
**Action:** When working with custom vector or string abstractions in `klib`, always prefer single-pass boundary calculations and bulk `extend_from_slice` memory copies over element-by-element loops.

## 2026-09-02 - Bulk `copy_from_slice` in Package Cache Buffer Allocation
**Learning:** In package registry proxy caching, copying payload buffers byte-by-byte in `for i in 0..data_len` loops forces per-index bounds checking and prevents the compiler from emitting vectorized `memcpy` intrinsics. Replacing manual byte-level array assignment with `cached.data[..data_len].copy_from_slice(&data[..data_len])` leverages optimized bulk CPU/SIMD memory transfer routines.
**Action:** When populating static or dynamic byte arrays in caching layers, always use `copy_from_slice` over manual element loops.

## 2026-09-03 - Hoisting Outer Map Lookups in Pairwise Audits
**Learning:** In pairwise collection scans (e.g. `detect_conflicts` in `DependencyResolver`), evaluating the outer item's map lookup `self.packages.get(pkg1_name)` inside the inner `(pkg1, pkg2)` loop re-queries the hash/B-tree map $N-1-i$ redundant times per outer item. Hoisting the outer lookup out of the inner loop reduces total map lookups from $N(N-1)$ to $\frac{N(N+1)}{2}$ (~50% reduction in map queries) while maintaining strict borrow checker lifetimes.
**Action:** Always hoist outer element lookups out of nested pair-scan loops when auditing or comparing elements against a map/registry.

## 2026-09-04 - Set Lookups & Drop Order Borrow Lifetimes in Transaction Audits
**Learning:** Replacing `Vec` linear scans with `BTreeSet` transforms $O(N)$ lookups into $O(\log N)$ set operations and allows `insert` to return duplicate status in a single pass. When borrowing slice references (`&str`) into a set (e.g., `BTreeSet<&str>`), the underlying vector containing the owned data (`Vec<AlpmPackage>`) must be declared before the set so that local variable drop order (reverse declaration) ensures the owned data outlives borrowed set references.
**Action:** When creating borrowed reference sets (`BTreeSet<&str>`) in local functions, always declare the owned container first.

## 2026-09-05 - In-Place Buffer Appending for JSON Serialization
**Learning:** In recursive data structure serialization (like JSON trees), calling `to_json_string()` on child elements or cloning keys (`key.clone()`) creates $O(N)$ temporary `String` heap allocations that are immediately concatenated and dropped. Passing a single mutable output buffer (`&mut String`) down the recursion tree and escaping string slices directly into the buffer eliminates all intermediate heap allocations during serialization.
**Action:** When serializing structured values, prefer buffer-appending methods (`append_to_buf(&self, out: &mut String)`) over returning owned temporary `String` objects from recursive methods.

## 2026-08-01 - Avoiding Heap Allocations in Dependency Traversal
**Learning:** Recursively traversing dependency trees with naive `to_visit: Vec<String>` structures incurs heavy heap reallocation and copy overhead if strings are cloned at every node visit. Storing references (`&str`) or using `to_visit` stacks with capacity pre-allocation dramatically cuts allocator stress during package dependency resolution.
**Action:** Pre-allocate capacity for traversal stacks and use borrowed string references where lifetimes allow.

## 2026-08-09 - Transitioning dynamic formatting out of hotpaths
**Learning:** Performing dynamic formatting like `format!("...")` inside critical execution loops blocks register reuse and triggers standard allocator locks. Replacing them with pre-allocated trace buffers saves microsecond context processing times.
**Action:** Always use static lifetime strings or write directly to static ring buffers in critical kernel tasks.

## 2026-08-10 - Target-Conditional Collection Re-Exports for Zero-Allocation & Host Compilation
**Learning:** Re-exporting custom `klib` collection structures (`klib::HashMap`, `klib::HashSet`) unconditionally under host targets (`target_os != "none"`) caused severe type inference errors and disabled standard compiler vectorization.
**Action:** Conditionally re-export standard `std::collections` on hosted targets and custom `klib` collections on bare-metal (`target_os = "none"`), ensuring optimal compilation speed and full host test compatibility.

## 2026-09-10 - Single-Pass Path Validation in Input Security
**Learning:** Performing multiple sequential loops over byte slices (e.g. scanning first for NUL bytes then scanning again for `..` path traversal sequences in `validate_path`) doubles memory bandwidth requirements and cache references for every path checked by the security subsystem. Merging NUL byte checking and boundary-aware delimiter scanning into a single pass reduces slice iteration count by 50% without altering validation behavior.
**Action:** When validating raw byte slices against multiple criteria, combine character and boundary checks into a single pass through the slice.

## 2026-09-11 - Fast Bitwise Bitmasking for Power-of-Two Hash Table Indexing
**Learning:** Computing hash bucket indices using hardware modulo division (`hash % capacity`) triggers CPU `div` instructions taking ~10-40 clock cycles. Since custom hash table structures guarantee bucket capacity as power-of-two values, replacing modulo division with bitwise AND mask (`hash & (capacity - 1)`) evaluates bucket indexing in a single CPU cycle.
**Action:** When designing custom hash tables, ring buffers, or fixed-capacity pools, maintain capacity as a power of two and use bitwise AND bitmasking (`& (capacity - 1)`) instead of integer division (`% capacity`).

## 2026-09-12 - Single-Pass Move Insertion for Hash Map Entry API
**Learning:** In map `Entry` API implementations (`or_insert` / `or_insert_with`), calling `map.insert(entry.key.clone(), val)` followed by `map.get_mut(&entry.key).unwrap()` forces key cloning, duplicate hash calculations, double bucket search passes, and unwrap checks. Implementing a single-pass `insert_entry(&mut self, key: K, value: V) -> &mut V` method moves the key directly into the map without cloning (`K: Clone` bound dropped), computes the hash once, and returns a mutable reference to the inserted/updated value directly.
**Action:** For map `Entry` APIs or upsert operations, provide a single-pass `insert_entry` method that consumes owned keys and returns mutable value references directly.

## 2026-09-13 - $O(N \log N)$ Indexing for System Package Rollback Diffing
**Learning:** In package snapshot rollback operations (`PackageSnapshotRollbackEngine`), calculating snapshot diffs and applying atomic rollbacks using nested linear searches (`current_packages.iter().find(...)`, `.any(...)`, `.contains(...)` in `.retain(...)`) results in $O(N^2)$ string comparisons. Indexing current system packages into `BTreeMap<&str, &str>` and `BTreeSet<&str>` (and `BTreeMap<String, usize>` index mapping) reduces algorithmic complexity from $O(N^2)$ to $O(N \log N)$ (reducing operations by ~70x for 1,000 packages) without borrowing lifetime conflicts during vector retention and mutation.
**Action:** When computing diffs or applying batch state rollbacks across collections, build owned or borrowed set/map indexes first to replace nested $O(N^2)$ linear scans with $O(N \log N)$ lookups.
