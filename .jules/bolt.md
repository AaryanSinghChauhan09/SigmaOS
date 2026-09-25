# Bolt's Journal

## 2026-07-20 - [O(1) Length Cache for Fixed Buffer Slices in no_std/Custom Vec Modules]
**Learning:** In zero-dependency/`no_std` modules with custom fixed array buffer structs (`[u8; 128]`, `[u8; 64]`), methods looking up or slicing element names often perform O(N) linear zero-byte scans (`.position(|&b| b == 0)`). Storing the exact byte length (`name_len: u8`) directly during initialization eliminates repeated linear scans on lookup/removal hot paths, converting name slice evaluation into instantaneous O(1) constant time.
**Action:** Always cache the explicit length (`len: u8`) when copying byte slices into fixed array buffers.
