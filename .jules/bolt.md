# Bolt's Journal - Critical Learnings Only

## 2026-10-27 - Zero-Copy Package Classification
**Learning:** Performing zero-copy header scanning directly on byte slices avoided unnecessary memory allocations during 250+ format inspections.
**Action:** Always prefer slice pattern matching over intermediate String/Vec allocations in package format classification.

## 2026-10-28 - Zero-Allocation Category Classification
**Learning:** Adding a `category(&self)` helper on enum values and using `.filter().count()` directly over slices avoids allocating temporary `Vec` buffers and cloning struct instances during category statistics and filtering queries.
**Action:** Use zero-allocation iterators over slice elements instead of allocating intermediate collections when computing element counts or filtered properties.
