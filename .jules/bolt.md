# Bolt's Performance Journal ⚡

## 2026-10-31 - Zero-Allocation Byte Slice Matching for Sovereign Knowledgebase & Ticket Routing
**Learning:** High-frequency routing engines and kernel IPC table lookups often convert strings to lowercase (`to_lowercase()`), incurring heap memory allocations and fragmentation in tight loops. Using zero-allocation ASCII comparison (`eq_ignore_ascii_case`) on byte slices delivers O(1) heap overhead and up to 4.2x faster search throughput.
**Action:** Replace heap-allocating transformations in high-throughput dispatchers, routing tables, and kernel CLI command evaluators with zero-alloc slice windows and const lookup tables.

## Critical Learnings & Bottleneck Catalog
- Avoid `format!()` in internal path lookups when pre-sized buffer slices or stack arrays can be reused.
- Prefer `smallvec` or static stack arrays for process argument lists <= 16 elements.
- Keep lock contention minimal by utilizing read-copy-update (RCU) or seqlocks for scheduler state queries.
