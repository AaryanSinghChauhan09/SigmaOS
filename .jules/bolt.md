# Bolt Agent Journal ⚡

## 2025-05-18 - Shell Script Tool Dependency Fallback Optimization
**Learning:** Shell release gate scripts that depend on external arithmetic tools like `bc` fail silently or produce syntax errors when `bc` is missing in containerized or minimal environments. Using native Bash integer/awk fallback calculation eliminates external process dependency failures and reduces gate evaluation execution latency.
**Action:** Always prefer native POSIX shell arithmetic or fallback checks for build/release gates.

## 2025-05-20 - Boot Service Name Length Caching and Sorting Optimization
**Learning:** Fixed-size byte array structures like `[u8; 64]` often rely on `.position(|&b| b == 0)` linear scans for slice conversions, adding unnecessary $O(N)$ overhead during frequent lookups. Caching string byte length upon struct initialization turns slice range lookups into $O(1)$ operations while replacing manual nested loops with standard $O(N \log N)$ `sort_by_key` yields measurably cleaner and faster sorting.
**Action:** Store `len: u8` during fixed byte array construction for fast constant-time slice range indexing.

## 2025-05-22 - Bulk Buffer Operations and Single-Pass Registry Lookups
**Learning:** Replacing byte-by-byte `output.push(byte)` loops with `extend_from_slice` and pre-allocated `with_capacity` in hot shell command output builders eliminates repeated re-allocation checks. Replacing two-pass registry lookup (`.position()` followed by index subscripting) with a single-pass `iter_mut()` loop eliminates redundant linear scans over trait object pointers.
**Action:** Always pre-allocate capacity and use bulk slice extension for byte buffer formatting, and use single-pass iterator loops for trait object searches.

## 2025-05-25 - Zero-Allocation File Block Skipping via Seek
**Learning:** Core utilities processing archived/chunked data (such as `tar`) often discard data blocks by allocating `vec![0u8; bytes_to_skip]` and reading bytes off disk into heap memory, causing $O(N)$ allocation and disk throughput overhead during index/header listing operations. Replacing buffer allocation with $O(1)$ zero-allocation `File::seek(SeekFrom::Current(bytes_to_skip))` eliminates both memory allocation and byte copy overhead completely.
**Action:** Always use relative file seeking (`SeekFrom::Current`) instead of heap allocation buffers when skipping data blocks in stream processors and core utilities.

## 2025-05-25 - Pre-Allocating String Capacity in Key Combination Formatters
**Learning:** In desktop shortcut formatting functions where multiple modifier strings and key labels are concatenated in sequence, `String::new()` incurs dynamic heap reallocations as characters are appended. Pre-allocating capacity using `String::with_capacity(estimated_len)` based on modifier count and key length eliminates intermediate buffer reallocations.
**Action:** Pre-allocate String capacity using `String::with_capacity` when formatting multi-part key binding combinations or string labels.

## 2025-05-28 - Zero-Allocation Case-Insensitive Sorting Comparators
**Learning:** Calling `a.to_lowercase().cmp(&b.to_lowercase())` inside `slice::sort_by` generates $O(N \log N)$ heap `String` allocations per sort operation (hundreds of thousands of temporary heap allocations for large line lists). Using an iterator-based `cmp_ignore_case` helper (`a.bytes().map(|b| b.to_ascii_lowercase())` for ASCII fast-path and `char::to_lowercase` iterator chaining for non-ASCII) performs comparisons in-place with zero heap allocations.
**Action:** Always use iterator-based case-insensitive byte/char comparison helpers inside `sort_by` comparators to eliminate $O(N \log N)$ heap allocations.

## 2025-06-01 - Pre-Allocating Vector Capacity for Master Verification Engines
**Learning:** Initializing collection fields with `Vec::new()` in heavily instantiated engine descriptors causes multiple sequential heap reallocations as elements are registered. Utilizing `Vec::with_capacity(exact_or_upper_bound)` eliminates dynamic vector resizing and re-allocation overhead during initialization loops.
**Action:** Always specify expected collection capacity using `Vec::with_capacity` when populating static arrays or registry collections.

## 2025-06-05 - Zero-Allocation Search Matching for Notes & Desktop Applications
**Learning:** In interactive desktop note search loops, calling `note.title.to_lowercase()` and `note.content.to_lowercase()` per item incurs hundreds of dynamic heap `String` allocations per search query. Utilizing ASCII window byte matching (`contains_ignore_case` & `find_ignore_case`) performs in-place case-insensitive matching without intermediate heap allocations.
**Action:** Always use window slice matching helpers (`contains_ignore_case` / `find_ignore_case`) for text searching across interactive desktop models.

## 2026-10-17 - Zero-Allocation Matching for CLI Tools Suite
**Learning:** In CLI power tool search engines (`fd`, `ripgrep`, `zoxide`), invoking `.to_lowercase()` per line or per file path allocates temporary heap `String`s for every candidate item evaluated. Using a zero-allocation ASCII window byte slice matching helper (`contains_ignore_case`) eliminates all heap allocations during search operations on ASCII paths and document lines.
**Action:** Apply `contains_ignore_case` ASCII window byte matching to file walker and document search engines to avoid per-item heap allocations.
