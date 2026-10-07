# Bolt's Journal - Critical Learnings

## 2026-10-06 - Zero-Allocation ASCII Substring Matching
**Learning:** Performing `.to_lowercase()` on `String` during search queries causes dynamic heap allocations for every element searched. Using zero-allocation ASCII window slice comparison eliminates string allocations on cold and hot search paths.
**Action:** Use `contains_ignore_case` helper for case-insensitive substring searching across search engines and filters.

## 2025-05-18 - Environment variable lookup hoisting & in-place update in shell sessions
**Learning:** In fixed-size array environment stores, searching keys inline causes repeated linear scan overhead per query. Appending keys on set without updating existing entries leads to unbounded accumulation of duplicate keys over long shell sessions.
**Action:** Hoist target key length calculation outside of the search loop using boundary checking, and always perform in-place updates when matching existing keys.
