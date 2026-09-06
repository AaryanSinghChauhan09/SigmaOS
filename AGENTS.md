# SigmaOS AI Agent Memory Management & Codebase Directives

This document defines core directives, architecture rules, and memory management invariants for all AI agents (Jules, Sentinel, Palette, Bolt) operating on the SigmaOS codebase.

## 1. Zero-Dependency Bare-Metal Memory Architecture
- **No External Allocators**: All memory management routines must utilize internal `klib` and kernel allocators (`src/memory/pmm_vmm.rs`, `src/memory/manager.rs`, `src/klib/custom_allocator.rs`, `src/klib/buddy_allocator.rs`).
- **`#![no_std]` Compatibility**: Kernel core modules must maintain strict `#![no_std]` + `extern crate alloc` compatibility.

## 2. Memory Subsystem Invariants & Safeguards
- **Physical & Virtual Memory Management**: PMM/VMM operations in `src/memory/pmm_vmm.rs` must enforce 4KiB page alignment and 2MiB/1GiB huge page boundaries.
- **Guard Pages & Hardened Allocations**: Heap and stack allocations must use hardened guard page allocators (`src/memory/resource_allocator.rs`) and ASLR randomized malloc guards (`src/klib/custom_allocator.rs`).
- **Memory Descriptor List (MDL) Pinning**: I/O and DMA memory buffers must pin memory ranges before descriptor transfers to prevent page fault race conditions under high concurrency.
- **Volatile Scrubbing**: Memory deallocations containing sensitive cryptographic material or keys must perform explicit volatile memory wipes (`AmnesicRamWipe` / zeroization) before returning pages to the buddy allocator.

## 3. Multi-Architecture Paging & Interrupt Balancing
- **x86_64 / x86_32**: PML4/PML5 vs 2-level PAE page tables and x2APIC/PIC8259 IRQ routing (`src/hal/multi_arch.rs`).
- **ARM64 / ARM32**: TTBR0_EL1 4-level 48-bit/52-bit translation vs Armv7 2-level paging with GICv3/GICv2 IRQ controllers.
- **RISC-V 64 / 32**: Sv39/Sv48 3/4-level vs Sv32 2-level paging with PLIC/CLINT timers.

## 4. AI Agent Testing & Verification Directives
- **Proactive Unit Testing**: Every code change or newly introduced feature must be accompanied by unit tests.
- **Master Test Runner**: Run `./run_sigma_tests.sh` to verify 100% test pass rate across Rust, C++, and Python test suites.
- **Standalone Module Testing**: Fast-verify specific modules using `rustc --test --edition 2021 <filepath> -o build/test_bin && ./build/test_bin`.

For detailed specifications, see `docs/AGENTS_MEMORY_MANAGEMENT.md`, `docs/AGENTS_TESTING_GUIDELINES.md`, and `docs/memory-management.md`.
