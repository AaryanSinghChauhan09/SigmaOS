# Bolt's Journal - Critical Learnings Only

## 2026-10-27 - Zero-Copy Package Classification
**Learning:** Performing zero-copy header scanning directly on byte slices avoided unnecessary memory allocations during 250+ format inspections.
**Action:** Always prefer slice pattern matching over intermediate String/Vec allocations in package format classification.

## 2026-10-28 - Zero-Allocation Query Search in Launcher
**Learning:** Calling `.to_lowercase()` on query strings per keystroke allocated transient heap `String`s when case-insensitive comparison helpers (`contains_ignore_case`, `starts_with_ignore_case`, `position_ignore_case`) already operate directly on `&[u8]` via `eq_ignore_ascii_case`.
**Action:** Pass unlowercased string slices directly to `eq_ignore_ascii_case` slice methods during search filtering to eliminate transient String heap allocations on every keypress.
