# AI Agent Buffer Overrun Management Architecture (`docs/AGENTS_BUFFER_OVERRUN.md`)

This guide details the architectural safeguards, FFI boundary checks, and AI agent monitoring protocols for buffer overrun and off-by-one mitigation in SigmaOS.

---

## 1. Subsystem Architectural Safeguards

SigmaOS enforces defense-in-depth mechanisms to eliminate buffer overruns:

### A. Safe FFI & C-String Boundary Checks
- Located in `src/klib/ffi.rs`.
- Provides bounds-safe string operations (`cstrlen`, `cstrcmp`, `rust_string_to_cstr`, `cstr_to_rust_string`) that validate null-terminators within fixed boundary limits to prevent out-of-bounds memory reads.

### B. Guarded Heap & Stack Allocation
- The resource allocator (`src/kernel/memory/resource_allocator.rs`) provides `alloc_with_guard_page` to catch heap overruns via unmapped guard pages.
- CPU context and user space descriptors (`src/arch/cpu_sys.rs`) enforce stack guard zones (`has_guard_page = true`).

### C. IPC & Ring Buffer Safety
- Inter-process communication and ring buffers validate write capacity prior to copying payload data, returning explicit overflow errors when buffers are full.

---

## 2. AI Agent Monitoring & Remediation Protocol

1. **Automated Static Scanning:** AI security agents review all modified Rust code for raw pointer operations or unchecked slice indexing.
2. **Runtime Fault Trap:** Memory access violations at guard page boundaries trigger instant kernel page faults and thread isolation.
3. **Automated Verification:** Execute `./run_sigma_tests.sh` to ensure all 220+ memory safety and unit tests pass.
