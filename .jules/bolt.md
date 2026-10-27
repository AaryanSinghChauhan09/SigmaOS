# Bolt's Journal - Critical Learnings Only

## 2026-10-27 - Zero-Copy Package Classification
**Learning:** Performing zero-copy header scanning directly on byte slices avoided unnecessary memory allocations during 250+ format inspections.
**Action:** Always prefer slice pattern matching over intermediate String/Vec allocations in package format classification.

## 2026-10-28 - O(N) Quickselect for Top-K Application Recommendations
**Learning:** Replacing O(N log N) full vector sorting with O(N) `select_nth_unstable_by` quickselect when retrieving top-5 suggested items eliminates unnecessary comparison work across large collections.
**Action:** Use `select_nth_unstable_by` for partial selection when retrieving small top-K subsets from unsorted vectors or maps.
