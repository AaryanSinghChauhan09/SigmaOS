# ⚡ Bolt's Journal — SigmaOS Performance Optimizations

This journal contains CRITICAL performance learnings discovered during profiling, compiling, and optimizing SigmaOS.

---

## 2024-07-15 - Unnecessary External Dependencies in Utility Modules
**Learning:** External crates like `rand` and `uuid` are heavy, introduce substantial compilation times, and are highly inefficient for simple simulation metrics or identifier generations. Replacing them with specialized local, zero-dependency implementations (such as a 48-bit Linear Congruential Generator for pseudo-random numbers and timestamp-nanoseconds for unique snapshot IDs) completely eliminates standard-library binding costs, reduces compiler overhead, and provides sub-nanosecond execution speeds.
**Action:** Always prefer lightweight, mathematically simple local algorithms over heavy external crate imports for simulation, telemetry, and non-cryptographic utility operations.

## 2024-07-15 - Ownership and Moves in Allocator Merge Trees
**Learning:** In Buddy Allocator merge operations, taking ownership of memory blocks by-value during a merge search leads to premature values being dropped if buddy merging fails. This forces expensive re-allocation or unnecessary clone overheads. Returning ownership of the original block in a `Result<MemoryBlock, MemoryBlock>` if buddy merging fails avoids all move-borrow complications, preserves zero-allocation guarantees, and maintains perfect linear execution speed.
**Action:** When designing hardware or memory managers in Rust, use `Result` wrappers to pass ownership back and forth safely without any allocation or cloning of control blocks.
<<<<<<< HEAD
=======

## 2026-07-17 - Zero-Allocation Version Parsing
**Learning:** Splitting a string and collecting the slices into a heap-allocated collection (such as `version_str.split('.').collect::<Vec<&str>>()`) in frequently called utility methods introduces performance overhead. Replacing this with an iterator-based inline parsing method completely avoids heap allocations and significantly reduces memory usage and execution time.
**Action:** Always utilize iterators and inline parsing for string manipulation/parsing rather than collecting intermediate elements into heap-allocated collections.

<<<<<<< HEAD
## 2026-07-23 - Zero-Allocation Shell Command Parser
**Learning:** Collecting all parsed segments of a command string into a heap-allocated `Vec<&str>` before routing can result in wasteful allocation overheads, especially for single-word terminal commands (e.g., `help`, `ps`, `ls`, `clear`, `exit`) or simple parameter queries. Refactoring the command scanner to stream tokens sequentially via standard Rust iterators like `split_whitespace` eliminates all dynamic collections on the hot paths, guaranteeing zero heap allocations for standard utility command matches.
**Action:** Leverage native iterator state machines to parse CLI input, retrieving commands and operands iteratively on-demand and avoiding upfront vector collections.
>>>>>>> temp-resolve-branch
=======
## 2026-07-23 - DPLL SAT Solver Recursion Caching
**Learning:** Evaluating SAT solver dependency graphs recursively on massive, nested package manifests poses a threat of stack overflow and incurs $O(N^2)$ traversal costs. Introducing a memoization cache maps package names directly to previously evaluated resolution lists, bypassing redundant sub-graph lookups and achieving optimal $O(N)$ execution bounds.
**Action:** Cache the outputs of recursive traversal sub-routines using state-passing or standard caches when analyzing multi-branch dependency matrices.
>>>>>>> temp-resolve-branch
