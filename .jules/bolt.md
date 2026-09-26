# Bolt's Journal

## 2026-07-20 - [O(1) Length Cache for Fixed Buffer Slices in no_std/Custom Vec Modules]
**Learning:** In zero-dependency/`no_std` modules with custom fixed array buffer structs (`[u8; 128]`, `[u8; 64]`), methods looking up or slicing element names often perform O(N) linear zero-byte scans (`.position(|&b| b == 0)`). Storing the exact byte length (`name_len: u8`) directly during initialization eliminates repeated linear scans on lookup/removal hot paths, converting name slice evaluation into instantaneous O(1) constant time.
**Action:** Always cache the explicit length (`len: u8`) when copying byte slices into fixed array buffers.

## 2026-03-31 - HashMap entry vs get_mut in INI parsing
**Learning:** In custom HashMap implementations or tight loops, using `get_mut` after checking/inserting section keys avoids re-allocating new String keys on every key-value line pair.
**Action:** Always check if a section map reference can be borrowed mutably via `get_mut` before falling back to `insert` with cloned section keys.