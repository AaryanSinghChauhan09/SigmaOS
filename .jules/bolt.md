# Bolt's Journal - Critical Learnings

## 2026-10-18 - Zero-Allocation String Matching Pattern
**Learning:** In Rust `no_std` / alloc-sensitive components, calling `.to_lowercase()` or `.to_string()` during search operations allocates heap `String` objects per iteration/entry. Replacing this with window-based ASCII case-insensitive comparison (`contains_ignore_case` / `windows().position()`) eliminates heap allocations and significantly accelerates search throughput.
**Action:** Always prefer ASCII window comparison (`zip`, `to_ascii_lowercase`) over heap-allocating string transforms in search or lookup loops.
