## 2026-03-31 - HashMap entry vs get_mut in INI parsing
**Learning:** In custom HashMap implementations or tight loops, using `get_mut` after checking/inserting section keys avoids re-allocating new String keys on every key-value line pair.
**Action:** Always check if a section map reference can be borrowed mutably via `get_mut` before falling back to `insert` with cloned section keys.
