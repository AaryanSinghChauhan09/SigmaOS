# AI Agent C++ Dependency Reduction Specification for SigmaOS

This document specifies operational standards for AI agents reducing C++ programming language dependencies and migrating legacy C++ code to pure Rust in **SigmaOS**.

---

## 1. C++ Reduction Guidelines

AI agents refactoring or replacing C++ code must follow these rules:

1. **Rust First**:
   - Implement all new drivers, kernel features, userland tools, and package handlers in pure Rust (`src/`).

2. **Parity Check**:
   - Ensure the Rust module in `src/` provides full feature parity before deprecating legacy `.cpp` / `.hpp` files.

3. **C-ABI FFI**:
   - Expose C-compatible interfaces via `#[no_mangle] extern "C"` in Rust rather than binding C++ classes.

4. **Zero-Alloc `no_std` Compliance**:
   - Use `klib` primitives (`klib::Vec`, `klib::String`, `klib::HashMap`) for Ring-0 kernel code.

---

## 2. Verification Protocol

- Run `./run_sigma_tests.sh` to execute the full test suite after any C++ to Rust refactoring.

---

*Maintained by the SigmaOS Core Architecture Committee.*
