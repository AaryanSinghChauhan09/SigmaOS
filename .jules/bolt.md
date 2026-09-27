# Bolt's Journal

## 2026-09-27 - Lock-Free CAS Algorithms & Hot Path Allocation Minimization
**Learning:** Performance optimizations must benchmark baseline execution cycles prior to code changes, utilize lock-free atomic `Compare-And-Swap` (CAS) primitives and lock-free ring buffers over heavy mutex locks in hot paths, and eliminate unnecessary `Vec`/`String` heap allocations or clones during loop iterations. Micro-optimizations should document CPU cycle count gains without sacrificing code readability.
**Action:** Always measure before/after cycles, prefer CAS ring buffers in hot paths, and avoid premature cold-path optimizations.

## 2026-09-27 - Safe Slice Boundary Matching for Package Dependency Resolution
**Learning:** Hoisting dependency name lookups outside of candidate loops in package managers reduces linear zero-byte scans from O(D * P) to O(D). When matching package names against dependency name slices across trait boundaries (where candidate names may be trimmed slices or fixed-width null-padded buffers), using `.get(dep_len).map_or(true, |&b| b == 0)` ensures safe, panic-free boundary checking.
**Action:** Always hoist invariant slice lookups out of inner loops and use `slice.get(len).map_or(true, ...)` for safe null-padded slice boundary matching.

## 2026-07-20 - [O(1) Length Cache for Fixed Buffer Slices in no_std/Custom Vec Modules]
**Learning:** In zero-dependency/`no_std` modules with custom fixed array buffer structs (`[u8; 128]`, `[u8; 64]`), methods looking up or slicing element names often perform O(N) linear zero-byte scans (`.position(|&b| b == 0)`). Storing the exact byte length (`name_len: u8`) directly during initialization eliminates repeated linear scans on lookup/removal hot paths, converting name slice evaluation into instantaneous O(1) constant time.
**Action:** Always cache the explicit length (`len: u8`) when copying byte slices into fixed array buffers.

## 2026-03-31 - HashMap entry vs get_mut in INI parsing
**Learning:** In custom HashMap implementations or tight loops, using `get_mut` after checking/inserting section keys avoids re-allocating new String keys on every key-value line pair.
**Action:** Always check if a section map reference can be borrowed mutably via `get_mut` before falling back to `insert` with cloned section keys.
