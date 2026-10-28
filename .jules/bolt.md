# Bolt's Journal - Critical Learnings Only

## 2026-10-27 - Zero-Copy Package Classification
**Learning:** Performing zero-copy header scanning directly on byte slices avoided unnecessary memory allocations during 250+ format inspections.
**Action:** Always prefer slice pattern matching over intermediate String/Vec allocations in package format classification.
