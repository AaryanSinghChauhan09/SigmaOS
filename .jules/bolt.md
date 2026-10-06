## 2025-10-06 - Zero-Allocation Case-Insensitive String Matching
**Learning:** Calling `.to_lowercase()` inside `.filter()` blocks during search query evaluation creates multiple temporary heap String allocations per candidate entry. Replacing `.to_lowercase().contains(...)` with a slice-window ASCII case-insensitive comparison helper (`contains_ignore_case`) eliminates heap allocations for ASCII inputs without compromising behavior.
**Action:** Use `contains_ignore_case` for string searching across UI desktop search, command palette, and launcher filters.
