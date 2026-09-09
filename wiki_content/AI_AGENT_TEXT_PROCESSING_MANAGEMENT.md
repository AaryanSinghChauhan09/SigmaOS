# AI Agent Text Processing & Regex Management Guidelines

## 1. Overview & Architecture
This document specifies AI agent standards for maintaining text processing filters (`grep`, `sed`, `awk`), fast string scanning algorithms, and regular expression parsing engines in SigmaOS (`src/klib/`, `src/tools/`).

---

## 2. Operational Directives for AI Agents

### 2.1 Fast String Scanning & Filtering
- **Vectorized Search**: Fast string search and pattern matching in `klib` must utilize SIMD acceleration (AVX2/NEON) where available for multi-gigabyte text file scanning.
- **Line & Token Processing**: `grep`, `sed`, and `awk` equivalents must support line-by-line streaming without buffering entire large files in RAM.

### 2.2 Regex Parsing Engine
- **POSIX & Extended Regex**: The native regex engine must compile and execute POSIX Basic Regular Expressions (BRE) and Extended Regular Expressions (ERE) deterministically in $O(N)$ time.
- **Zero Memory Leaks**: All AST allocations for compiled regex expressions must be safely deallocated upon stream completion.

---

## 3. Related Files
- `src/klib/`
- `src/tools/native_userland_replacements.rs`
- `docs/LINUX_DISTRO_PARITY_CHECKLIST.md`
