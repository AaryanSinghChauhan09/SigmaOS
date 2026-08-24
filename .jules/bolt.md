# ⚡ Bolt's Journal — SigmaOS Performance & Optimization

This journal logs CRITICAL performance bottlenecks, compiler optimization analyses, and resource-efficiency enhancements implemented across SigmaOS.

---

## 2026-08-01 - Eliminating Index-Based Modulo Division in Loop Bodies
**Learning:** Using standard indexing loops with modulo division (`i % key.len()`) inside hot loops introduces two severe performance penalties:
1. **Division Overhead:** Integer division/modulo is one of the slowest CPU instructions (typically 10-40 cycles depending on the architecture).
2. **Bounds Checks:** Direct array indexing (`key[index]`) forces the Rust compiler to insert branch/panic instructions for out-of-bounds safety checks, preventing auto-vectorization and loop unrolling.

Using a pre-allocated vector and a single-pass iterator chain (`.iter().cycle()`) zipped with the input iterator completely eliminates modulo division and array bounds checks, enabling the compiler to optimize the loop into highly efficient SIMD/vectorized instructions.
**Action:** Always prefer `.zip(key.iter().cycle())` over index-modulo loops for symmetric/XOR encryption and decryption operations.

## 2026-08-01 - Avoiding Heap Allocations in Dependency Traversal
**Learning:** Recursively traversing dependency trees with naive `to_visit: Vec<String>` structures incurs heavy heap reallocation and copy overhead if strings are cloned at every node visit. Storing references (`&str`) or using `to_visit` stacks with capacity pre-allocation dramatically cuts allocator stress during package dependency resolution.
**Action:** Pre-allocate capacity for traversal stacks and use borrowed string references where lifetimes allow.

## 2026-08-09 - Transitioning dynamic formatting out of hotpaths
**Learning:** Performing dynamic formatting like `format!("...")` inside critical execution loops blocks register reuse and triggers standard allocator locks. Replacing them with pre-allocated trace buffers saves microsecond context processing times.
**Action:** Always use static lifetime strings or write directly to static ring buffers in critical kernel tasks.

## 2026-08-10 - Target-Conditional Collection Re-Exports for Zero-Allocation & Host Compilation
**Learning:** Re-exporting custom `klib` collection structures (`klib::HashMap`, `klib::HashSet`) unconditionally under host targets (`target_os != "none"`) caused severe type inference errors and disabled standard compiler vectorization.
**Action:** Conditionally re-export standard `std::collections` on hosted targets and custom `klib` collections on bare-metal (`target_os = "none"`), ensuring optimal compilation speed and full host test compatibility.

## 2026-08-15 - Explicit String Length Storage for Fixed-Size Byte Buffer Records
**Learning:** Repeatedly invoking linear scans (`.position(|&b| b == 0)`) across fixed-size byte buffer arrays (such as `[u8; 16]`, `[u8; 64]`, or `[u8; 256]`) inside frequent loop bodies (such as package vulnerability remediation and report generation) creates an unnecessary $O(N)$ scanning bottleneck per record. Storing explicit length fields (`cve_id_len: u8`, `affected_package_len: u8`) upon `VulnerabilityReport` record creation reduces slice retrieval to an instantaneous $O(1)$ operation, avoiding CPU cache misses and byte-by-byte comparison overhead during bulk vulnerability audits.
**Action:** Always store explicit byte lengths alongside fixed-size buffer arrays when records are repeatedly sliced/compared during high-frequency subsystem scans.

## 2026-08-19 - Caching Explicit Slicing Lengths for Fixed Byte Array Fields in Logging Subsystems
**Learning:** In fixed-size buffer structures (like `[u8; 256]` in `SimpleLogFile`), retrieving slice paths via `.position(|&b| b == 0)` runs an $O(N)$ scan up to 256 bytes for every single path reference or log rotation event. Storing an explicit `path_len: u8` field during struct initialization replaces linear scans with instant $O(1)$ index slicing `&self.path[..self.path_len as usize]`, eliminating linear scanning overhead during high-frequency log operations.
**Action:** Store explicit byte lengths (`path_len: u8`) when initializing fixed byte array fields in log files or IO handles to guarantee $O(1)$ slice retrieval.

## 2026-08-20 - Storing Cached Byte Lengths for Fixed-Size Log Buffers
**Learning:** In fixed-size string/byte arrays (`[u8; 64]`, `[u8; 128]`, `[u8; 512]`), converting to string representations via `.position(|&b| b == 0)` causes $O(N)$ linear byte scans on every log message output, serialization, or network dispatch. Storing explicit length fields (`component_len`, `message_len`, `module_len`) during struct instantiation turns slice operations into instantaneous $O(1)$ lookups.
**Action:** Store cached length fields alongside fixed-size buffer fields to avoid linear zero-byte scans during frequent display, formatting, or serialization routines.

## 2026-08-21 - Single-Pass Process Candidate Selection in EEVDF Scheduler
**Learning:** In scheduler loops (`Scheduler::schedule_on_core` and `Scheduler::check_preemption`), running sequential filtering passes over the process vector (`self.processes.iter().filter(...).min_by_key(...)`) multiplies CPU iteration cycles per tick when distinguishing real-time tasks from standard tasks or finding candidate processes for preemption. Performing a single linear pass that simultaneously tracks the best real-time candidate (`best_rt`), standard candidate (`best_eligible`), and running process eliminates up to 50% of vector traversal overhead during scheduling ticks.
**Action:** Replace sequential multi-pass vector iterations with single-pass accumulator loops when evaluating process candidates in kernel scheduler hot paths.

## 2026-08-21 - Caching Byte Lengths in User Authentication Structs
**Learning:** In authentication and user management routines (`SimpleUser`), calling `User::username(&self)` repeatedly executed an $O(N)$ linear scan (`.position(|&b| b == 0)`) over 32 bytes on every username comparison and access. Caching `username_len: u8` during `SimpleUser::new()` instantiation reduces slice retrieval to an instantaneous $O(1)$ constant-time lookup `&self.username[..self.username_len as usize]`, eliminating linear scanning overhead in user authentication routines.
**Action:** Store `username_len: u8` during struct creation when managing fixed-size username byte arrays to ensure $O(1)$ constant-time username slicing.

## 2026-08-22 - Caching Stored Byte Lengths for SimpleShellCommand
**Learning:** In command resolution and execution hot paths (`SimpleShellCommand`), calling `cmd.name()` or `cmd.help()` repeatedly executed an $O(N)$ linear byte scan (`.position(|&b| b == 0)`) over 32-byte name and 128-byte description arrays. Caching `name_len: u8` and `description_len: u8` during `SimpleShellCommand::new()` initialization turns slice retrieval into an instantaneous $O(1)$ constant-time lookup, eliminating linear scanning overhead during shell command lookup and dispatch.
**Action:** Always store explicit byte lengths alongside fixed-size byte array fields in shell command structures to guarantee $O(1)$ constant-time slice lookups.

## 2026-08-23 - Functional Iterator min_by_key in CPU Scheduler Task Selection
**Learning:** In CPU scheduler task selection loops (`EevdfScheduler`, `BoreScheduler`, `CfsScheduler`, `SchedDeadline`), using manual imperative indexing loops with conditional tracking variables introduces bounds-checking overhead and manually updated state registers. Replacing imperative loops with functional `tasks.iter().enumerate().min_by_key(...)` iterator chains allows LLVM to eliminate unnecessary bounds checking branch instructions and unroll minimum element search loops efficiently during high-frequency scheduler ticks.
**Action:** Prefer functional iterator chains like `.iter().enumerate().min_by_key(...)` over manual loop indexing state trackers for task selection loops in kernel scheduling algorithms.

## 2026-08-23 - Sliding-Window Rate Limiting via Ring Buffers
**Learning:** Using a fixed-size circular ring buffer `[u64; 32]` with saturating timestamp subtraction `current_timestamp.saturating_sub(ts)` for sliding-window packet rate limiting avoids heap allocation and complex timer heap traversals.
**Action:** Use fixed-size ring buffers with saturating arithmetic for high-throughput network rate limiters in bare-metal routers.

## 2026-08-24 - Caching Byte Lengths in SimplePermission Structs
**Learning:** In privacy management and permission checking routines (`SimplePermission`), calling `name()` and `category()` repeatedly executed $O(N)$ linear scans (`.position(|&b| b == 0)`) over 64-byte name and category arrays during category indexing and permission checks. Caching `name_len: u8` and `category_len: u8` during `SimplePermission::new()` initialization turns slice retrieval into instantaneous $O(1)$ constant-time lookups `&self.name[..self.name_len as usize]`, eliminating linear scanning overhead in permission verification hot paths.
**Action:** Store `name_len: u8` and `category_len: u8` during struct creation when managing fixed-size byte arrays in security and privacy permission models.

## 2026-08-25 - Caching Stored Byte Lengths for SimpleDevice
**Learning:** In device management and driver probing hot paths (`SimpleDevice`), calling `dev.name()` repeatedly executed an $O(N)$ linear byte scan (`.position(|&b| b == 0)`) over 64-byte name arrays on every device lookup, list, and status check. Storing `name_len: u8` during `SimpleDevice::new()` initialization turns slice retrieval into an instantaneous $O(1)$ constant-time lookup `&self.name[..self.name_len as usize]`, eliminating linear scanning overhead during high-frequency hardware device management routines.
**Action:** Always store explicit byte lengths (`name_len: u8`) alongside fixed-size byte array fields in device management structures to guarantee $O(1)$ constant-time slice lookups.

## 2026-08-26 - Caching Stored Byte Lengths for SimplePackage
**Learning:** In package management and resolution hot paths (`SimplePackage`), calling `pkg.name()` or `pkg.version()` repeatedly executed $O(N)$ linear byte scans (`.position(|&b| b == 0)`) over 64-byte name and 32-byte version arrays on every package query or dependency resolution step. Storing `name_len: u8` and `version_len: u8` during `SimplePackage::new()` initialization turns slice retrieval into instantaneous $O(1)$ constant-time lookups `&self.name[..self.name_len as usize]`, eliminating linear scanning overhead during high-frequency package queries.
**Action:** Store explicit byte lengths (`name_len: u8`, `version_len: u8`) alongside fixed-size byte array fields in package records to guarantee $O(1)$ constant-time slice lookups.
