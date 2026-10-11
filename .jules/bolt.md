# Bolt Agent Journal ⚡

## 2025-05-18 - Shell Script Tool Dependency Fallback Optimization
**Learning:** Shell release gate scripts that depend on external arithmetic tools like `bc` fail silently or produce syntax errors when `bc` is missing in containerized or minimal environments. Using native Bash integer/awk fallback calculation eliminates external process dependency failures and reduces gate evaluation execution latency.
**Action:** Always prefer native POSIX shell arithmetic or fallback checks for build/release gates.

## 2025-05-20 - Boot Service Name Length Caching and Sorting Optimization
**Learning:** Fixed-size byte array structures like `[u8; 64]` often rely on `.position(|&b| b == 0)` linear scans for slice conversions, adding unnecessary $O(N)$ overhead during frequent lookups. Caching string byte length upon struct initialization turns slice range lookups into $O(1)$ operations while replacing manual nested loops with standard $O(N \log N)$ `sort_by_key` yields measurably cleaner and faster sorting.
**Action:** Store `len: u8` during fixed byte array construction for fast constant-time slice range indexing.
