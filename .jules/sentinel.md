# Sentinel's Journal

## 2026-03-31 - [Subsystem Warning & Parameter Security Sanitization]
**Learning:** Prefixing unused syscall and performance tuning arguments with underscores (`_`) prevents compiler dead-code warnings while explicitly documenting unused parameters in security-sensitive kernel dispatcher entrypoints.
**Action:** Consistently sanitize unused function arguments in low-level syscall handlers and hardware tuning interfaces.

## 2026-10-02 - Unveil Sandboxing Path Traversal & Null-Byte Bypass Prevention
**Vulnerability:** `UnveilSandbox::check_path_access` evaluated path restrictions without checking for NUL bytes, URL-encoded traversal patterns (`%2e%2e`, `%2f`, `%5c`), or `..`/`.` segments, allowing sandboxed processes to access restricted files outside unveiled directories.
**Learning:** Prefix matching alone without strict boundary checking (slash or end-of-string) allows prefix collision bypasses (`/tmp-secret` matching `/tmp`). Calling `unveil()` must also toggle `default_deny = true` to restrict non-unveiled path access.
**Prevention:** Always validate path inputs against NUL byte truncation, URL-encoded traversal sequences, and directory traversal segments, and enforce boundary character checks on prefix matching in path sandbox implementations.
