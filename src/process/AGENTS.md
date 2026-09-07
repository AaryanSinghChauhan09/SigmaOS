# AI Agent Development Instructions for Process Management Subsystem (`src/process/`)

This directory contains high-level process management engines, ELF executable loading, job object control groups, `/proc` and `/sys` virtual filesystem emulation, activity management, and process spawning logic for SigmaOS.

## Subsystem Architecture & Directives

1. **ELF Executable Loader (`elf_loader.rs`)**
   - Parse 64-bit ELF headers (`e_ident`), program headers (`PT_LOAD`, `PT_INTERP`, `PT_DYNAMIC`), and calculate virtual address load offsets (`e_entry`).
   - Validate ELF magic numbers (`\x7fELF`) and ABI compatibility before mapping memory segments.

2. **Advanced Process Control & Job Objects (`advanced_process_control.rs` & `job_objects.rs`)**
   - Manage job object limits (CPU rate limiting, active process limits, memory working sets).
   - Enforce process tree termination when a job object is closed or killed.

3. **Linux `/proc` & `/sys` Emulation (`linux_proc.rs` & `linux_sysfs.rs`)**
   - Dynamically generate POSIX `/proc/[pid]/status`, `/proc/[pid]/stat`, `/proc/meminfo`, `/proc/cpuinfo`, and `/proc/cmdline` text nodes without disk allocations.
   - Ensure virtual files implement clean zero-copy reads via stream buffers.

4. **Sovereign Process Engine (`sovereign_process_engine.rs` & `manager.rs`)**
   - Maintain process tracking tables and activity state updates.
   - Synchronize process state changes with desktop UI monitors and logging subsystems.

5. **Concurrency & Verification**
   - Ensure thread-safe access to global process tables using lock-free data structures or `SigmaSpinlock`.
   - Verify changes by running `cargo check --lib` to ensure no warnings or broken imports.
