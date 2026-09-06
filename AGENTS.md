# AI Agent Directives & Memory Management Guide for SigmaOS

Welcome, AI Engineer / Agent! This document specifies core operational guidelines and memory management procedures for working with the SigmaOS codebase.

---

## 🧠 Memory Management Principles in SigmaOS

1. **Zero-Allocation Primitives (`klib`)**
   - Core kernel and low-level subsystem primitives (`src/klib/`) operate in `#![no_std]` environment.
   - Prefer stack-based formatting (`format_u64_stack` in `src/klib/conversion.rs`) and FNV-1a zero-allocation hashing (`fnv1a_hash_64`) over dynamic heap allocations.

2. **Single-Buffer In-Place Formatting**
   - When serializing structured data (e.g. JSON in `src/klib/json.rs`), use `append_json_string` or single-buffer mutators rather than returning newly allocated `String` objects per field/key.

3. **Safe Memory Allocation Layering**
   - Userland components (`src/desktop/`, `src/sigpkg/`) use standard `alloc::vec::Vec` and `alloc::boxed::Box`.
   - Kernel memory management (`src/memory/`) usesBuddy Allocator (`sigma_buddy.rs`), Slab Cache (`slab.rs`), and Physical Memory Manager (`pmm_vmm.rs`). Never create custom unsafe `Vec<T>` structs.

4. **Virtual Memory Paging Indexing**
   - In 4-level paging (`SimpleVMM` in `src/klib/paging.rs`), maintain unique table indexing:
     - `pd_table_idx = pml4_idx * 512 + pdpt_idx`
     - `pt_table_idx = (pml4_idx * 512 + pdpt_idx) * 512 + pd_idx`

---

## ⚙️ Testing & Verification Procedures

- **Kernel Primitives (`klib`):**
  `cargo test --lib -- klib::json` or `./run_sigma_tests.sh`
- **Linux/BSD System Gap Engines:**
  `rustc --test src/distro/linux_bsd_distro_gaps.rs --edition=2021 -o build/distro_gaps_test && ./build/distro_gaps_test`
- **Python Integration Suite:**
  `pytest tests/test_unit_core.py tests/test_integration_system.py tests/test_stress_fuzz_bench.py`
