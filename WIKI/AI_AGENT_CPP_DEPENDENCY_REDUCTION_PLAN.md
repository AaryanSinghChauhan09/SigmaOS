# SigmaOS AI Agent C++ Dependency Reduction & Pure-Rust Migration Plan

## 1. Executive Summary & Strategic Objectives

SigmaOS prioritizes a zero-dependency, memory-safe, pure-Rust system architecture. While early development phases introduced auxiliary C++ driver shims, header wrappers (`.hpp`, `.h`), and legacy userland stubs (`kernel/`, `userland/`, `drivers/`, `suites/`), the long-term roadmap mandates the systematic reduction and elimination of C++ code in favor of native zero-alloc Rust implementations in `src/` backed by `klib`.

This document establishes the official AI agent strategy, migration phases, and architectural guidelines for reducing C++ language dependencies across the SigmaOS repository.

---

## 2. Current C++ Component Audit & Replacement Targets

An audit of the repository identifies four main categories of C++ files slated for migration to Rust:

| C++ Category | Path Patterns | Primary Responsibilities | Target Pure-Rust Module |
| :--- | :--- | :--- | :--- |
| **Legacy Hardware Drivers** | `drivers/usb/`, `drivers/graphics/` | USB xHCI HCD, DRM/KMS GPU drivers | `src/drivers/modern_usb.rs`, `src/drivers/modern_nvme.rs` |
| **Kernel Core & IPC Stubs** | `kernel/core/`, `sigmaos/core/` | Cgroup, WASM, Syscall, Seccomp stubs | `src/kernel/cgroup_controllers.rs`, `src/kernel/ebpf_vm.rs` |
| **Userland Utilities & Daemons** | `userland/` | Shell, init, pkg resolver, terminal C++ tools | `src/shell/repl.rs`, `src/userland/shell.rs`, `src/bin/sigpkg.rs` |
| **HAL & Suite C++ Headers** | `suites/`, `klib/include/`, `include/` | `.hpp`, `.h` C++ header bridge files | Native Rust FFI bindings & `klib` core structures |

---

## 3. Phased Migration Strategy for AI Agents

AI agents refactoring or deprecating legacy C++ code must execute the following 4-phase transition plan:

### Phase 1: Pure-Rust Engine Feature Parity Verification
- Before removing any C++ component (`.cpp` / `.hpp`), verify that the corresponding Rust implementation in `src/` has 100% functional and test parity.
- For example, verify that `src/drivers/modern_usb.rs` fully subsumes `drivers/usb/sigma_usb_hcd.cpp`.

### Phase 2: Foreign Function Interface (FFI) Wrapping & C-ABI Shims
- Where legacy C/C++ host builds require interaction with Rust components, expose standard C-ABI functions using `#[no_mangle] extern "C"` in Rust rather than calling C++ code from Rust.
- Transition `extern "C++"` linkages to pure `extern "C"`.

### Phase 3: Build System Consolidation (`Cargo` & `rustc`)
- Deprecate root `CMakeLists.txt` targets and legacy C++ `Makefile` rules in favor of Cargo build targets (`Cargo.toml`) and `standalone_test` rustc scripts (`./run_sigma_tests.sh`).
- Remove `tests/cpp_host/` C++ test dependencies once standalone `rustc --test` and Cargo test harnesses cover all scenario matrices.

### Phase 4: Full Codebase Elimination & Purge
- Safely delete orphan C++ files once all functional paths, CLI commands, and test suites run entirely through native Rust modules (`src/`).

---

## 4. Coding Conventions for Pure-Rust Migration

AI agents porting C++ logic to Rust must strictly follow SigmaOS design conventions:

1. **Zero External Crates (`no_std`)**: Use `klib` custom collections (`klib::HashMap`, `klib::Vec`, `klib::String`, `klib::BTreeMap`) in Ring-0 and embedded modules.
2. **Memory Safety & Unsafe Bounds**: Eliminate raw pointer arithmetic and unchecked `reinterpret_cast` idioms common in C++; encapsulate unsafe hardware accesses within safe Rust abstractions.
3. **Error Handling**: Replace C++ exceptions (`try`/`catch`) and error codes with Rust `Result<T, &'static str>` or custom error enums.

---

## 5. Verification & Regression Testing

AI agents completing C++ to Rust migrations must pass full regression testing:

1. **Native Test Runner**: Execute `./run_sigma_tests.sh` to confirm all 13 test phases pass cleanly.
2. **Inspection Test Matrix**: Verify that zero C++ compilation warnings or symbol linkage errors occur during host or target builds.

---

*Approved by the SigmaOS System Architecture & Rust Migration Steering Committee.*
