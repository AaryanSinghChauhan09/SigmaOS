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

## 2025-05-25 - Pre-Allocating String Capacity in Key Combination Formatters
**Learning:** In desktop shortcut formatting functions where multiple modifier strings and key labels are concatenated in sequence, `String::new()` incurs dynamic heap reallocations as characters are appended. Pre-allocating capacity using `String::with_capacity(estimated_len)` based on modifier count and key length eliminates intermediate buffer reallocations.
**Action:** Pre-allocate String capacity using `String::with_capacity` when formatting multi-part key binding combinations or string labels.
