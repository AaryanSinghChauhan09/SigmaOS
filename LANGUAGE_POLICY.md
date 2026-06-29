# SigmaOS Language Policy & ABI Guidelines

To stabilize our polyglot codebase and avoid ABI conflicts, SigmaOS enforces strict guidelines on programming language usage.

---

## 🗺️ Language Domains

| Language | Primary Target Domain | Execution Rules |
| :--- | :--- | :--- |
| **Rust** | Microkernel core, critical modules, modern driver trees. | `#![no_std]`, no standard allocation unless in dedicated crates. |
| **Zig** | Leaf performance-sensitive drivers, low-level HAL utilities. | Minimal dependency, static leaf compilation only. |
| **Ada/SPARK** | Formal proof-critical security checkers, memory verifiers. | Must pass `gnatprove` analysis before merge. |
| **Nim** | Userspace tools, CLI helpers, store package managers. | Compiled with JS backend or C backend without GC when inside userspace. |

---

## 📞 FFI Boundaries (The kabi/ C-ABI Rules)

1. **repr(C)**: All cross-language structures MUST be defined in `kabi/` as `#[repr(C)]` Rust structures or equivalent C headers.
2. **Raw Pointers**: Avoid returning raw allocations. Pass structures by reference/pointer with explicit size boundaries.
3. **No Panic FFI**: Rust code must not panic across an FFI boundary. Use `catch_unwind` or check boundaries before execution.
