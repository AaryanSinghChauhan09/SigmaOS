# Sentinel's Journal

## 2026-03-31 - [Subsystem Warning & Parameter Security Sanitization]
**Learning:** Prefixing unused syscall and performance tuning arguments with underscores (`_`) prevents compiler dead-code warnings while explicitly documenting unused parameters in security-sensitive kernel dispatcher entrypoints.
**Action:** Consistently sanitize unused function arguments in low-level syscall handlers and hardware tuning interfaces.

## 2026-10-02 - Unveil Sandboxing Path Traversal & Null-Byte Bypass Prevention
**Vulnerability:** `UnveilManager::validate_path` evaluated path restrictions without checking for NUL bytes, URL-encoded traversal patterns (`%2e%2e`, `%2f`), or `..` segments, allowing sandboxed processes to access restricted files outside unveiled directories.
**Learning:** Prefix matching alone is insufficient for sandboxing; paths must be checked for C-ABI truncation (embedded NUL bytes) and traversal sequences before boundary matching.
**Prevention:** Always validate path inputs against NUL byte truncation and directory traversal sequences before performing prefix or glob matches in sandboxing subsystems.

## 2026-10-02 - Landlock Sandboxing Path Traversal, Null-Byte Bypass, and String Allocation Optimization
**Vulnerability:** `LandlockRuleset::is_access_allowed` evaluated filesystem sandbox access without checking for NUL byte C-ABI truncation, URL-encoded traversal patterns (`%2e%2e`, `%2f`, `%5c`), or `..`/`.` directory traversal path segments. It also formatted strings (`format!("{}/", ...)`) inside the path matching loop on every access check.
**Learning:** Filesystem sandboxing rule engines must enforce input sanitization against truncation/traversal vectors prior to rule checking, and perform zero-allocation string slice prefix matching to optimize path evaluation loops.
**Prevention:** Always validate input paths against embedded NUL bytes, URL-encoded patterns, and traversal segments, using slice-based prefix boundary checks instead of heap allocations.
