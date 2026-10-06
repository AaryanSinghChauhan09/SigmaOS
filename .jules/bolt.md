# Bolt's Journal - Critical Learnings

## 2026-10-06 - Zero-Allocation ASCII Substring Matching
**Learning:** Performing `.to_lowercase()` on `String` during search queries causes dynamic heap allocations for every element searched. Using zero-allocation ASCII window slice comparison eliminates string allocations on cold and hot search paths.
**Action:** Use `contains_ignore_case` helper for case-insensitive substring searching across search engines and filters.
