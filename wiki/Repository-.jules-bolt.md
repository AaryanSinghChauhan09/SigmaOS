> Imported repository document from [`.jules/bolt.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/.jules/bolt.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

# Bolt's Journal

## 2026-03-31 - Zero-Allocation O(N) Min Selection over O(N^2) Sorting
**Learning:** Selecting a single minimum/maximum element from a collection (such as latency ranking across network mirrors or priority queues) using sorting algorithms (like bubble sort or `sort_by`) allocates temporary vectors and takes O(N^2) or O(N log N) time. Using `.min_by_key(...)` or `.max_by_key(...)` directly on the filtered iterator executes in linear O(N) time with zero heap allocations.
**Action:** Replace sorting loops with `.min_by_key(...)` or `.max_by_key(...)` when only the single extremum element is required.

## 2026-09-27 - Safe Slice Boundary Matching for Package Dependency Resolution
**Learning:** Hoisting dependency name lookups outside of candidate loops in package managers reduces linear zero-byte scans from O(D * P) to O(D). When matching package names against dependency name slices across trait boundaries (where candidate names may be trimmed slices or fixed-width null-padded buffers), using `.get(dep_len).map_or(true, |&b| b == 0)` ensures safe, panic-free boundary checking.
**Action:** Always hoist invariant slice lookups out of inner loops and use `slice.get(len).map_or(true, ...)` for safe null-padded slice boundary matching.

## 2026-07-20 - [O(1) Length Cache for Fixed Buffer Slices in no_std/Custom Vec Modules]
**Learning:** In zero-dependency/`no_std` modules with custom fixed array buffer structs (`[u8; 128]`, `[u8; 64]`), methods looking up or slicing element names often perform O(N) linear zero-byte scans (`.position(|&b| b == 0)`). Storing the exact byte length (`name_len: u8`) directly during initialization eliminates repeated linear scans on lookup/removal hot paths, converting name slice evaluation into instantaneous O(1) constant time.
**Action:** Always cache the explicit length (`len: u8`) when copying byte slices into fixed array buffers.

## 2026-03-31 - HashMap entry vs get_mut in INI parsing
**Learning:** In custom HashMap implementations or tight loops, using `get_mut` after checking/inserting section keys avoids re-allocating new String keys on every key-value line pair.
**Action:** Always check if a section map reference can be borrowed mutably via `get_mut` before falling back to `insert` with cloned section keys.

## 2026-09-30 - Zero-Allocation ASCII Substring Search in Menu Indexing & Launchers
**Learning:** Calling `.to_lowercase()` on string fields during application menu search indexing (e.g. `item.name.to_lowercase().contains(&q)`) allocates temporary `String`s on the heap for every field of every candidate item on every keystroke. Using ASCII byte window matching (`.as_bytes().windows(len).any(|w| w.eq_ignore_ascii_case(...))`) eliminates heap allocations entirely on ASCII search hot paths while falling back safely for UTF-8 input.
**Action:** Use zero-allocation ASCII substring window matching for case-insensitive search in UI menus and launchers instead of lowercasing input/target strings.

## 2026-09-30 - Zero-Allocation Substring Match in Foreign Package Index Search
**Learning:** Multi-distro package repository searching (`DistroRepoSyncEngine::search_indexed_manifests`) filtering foreign package manifests (`original_name`) repeatedly allocated heap strings via `.to_lowercase()`. Utilizing zero-allocation ASCII byte window matching (`contains_ignore_case`) removes heap allocation overhead during package repository search operations.
**Action:** Replace `.to_lowercase().contains()` with zero-allocation ASCII byte window matching in package manager search functions.

## 2026-08-10 - Target-Conditional Collection Re-Exports for Zero-Allocation & Host Compilation
**Learning:** Re-exporting custom `klib` collection structures (`klib::HashMap`, `klib::HashSet`) unconditionally under host targets (`target_os != "none"`) caused severe type inference errors and disabled standard compiler vectorization.
**Action:** Conditionally re-export standard `std::collections` on hosted targets and custom `klib` collections on bare-metal (`target_os = "none"`), ensuring optimal compilation speed and full host test compatibility.

## 2026-08-15 - Explicit String Length Storage for Fixed-Size Byte Buffer Records
**Learning:** Repeatedly invoking linear scans (`.position(|&b| b == 0)`) across fixed-size byte buffer arrays (such as `[u8; 16]`, `[u8; 64]`, or `[u8; 256]`) inside frequent loop bodies (such as package vulnerability remediation and report generation) creates an unnecessary $O(N)$ scanning bottleneck per record. Storing explicit length fields (`cve_id_len: u8`, `affected_package_len: u8`) upon `VulnerabilityReport` record creation reduces slice retrieval to an instantaneous $O(1)$ operation, avoiding CPU cache misses and byte-by-byte comparison overhead during bulk vulnerability audits.
**Action:** Always store explicit byte lengths alongside fixed-size buffer arrays when records are repeatedly sliced/compared during high-frequency subsystem scans.

## 2026-08-19 - Caching Explicit Slicing Lengths for Fixed Byte Array Fields in Logging Subsystems
**Learning:** In fixed-size buffer structures (like `[u8; 256]` in `SimpleLogFile`), retrieving slice paths via `.position(|&b| b == 0)` runs an $O(N)$ scan up to 256 bytes for every single path reference or log rotation event. Storing an explicit `path_len: u8` field during struct initialization replaces linear scans with instant $O(1)$ index slicing `&self.path[..self.path_len as usize]`, eliminating linear scanning overhead during high-frequency log operations.
**Action:** Store explicit byte lengths (`path_len: u8`) when initializing fixed byte array fields in log files or IO handles to guarantee $O(1)$ slice retrieval.

## 2026-08-21 - Length-Cached Byte Arrays Across Container and AI Orchestration Subsystems
**Learning:** Container and AI resource representations using fixed-size byte buffers (`[u8; 32]`, `[u8; 64]`, `[u8; 128]`) default to linear null-byte search scans (`position(|&b| b == 0)`) during string slice queries. Initializing an explicit `name_len: u8` / `image_len: u8` field during struct construction converts slice creation to an instantaneous $O(1)$ length slice `&buf[..len as usize]`, eliminating repeated scanning bottlenecks in high-frequency container lookup and model eviction loops.
**Action:** When defining fixed-size byte array records in high-frequency runtime subsystems, always include explicit `len` fields to enforce $O(1)$ slice retrieval.
