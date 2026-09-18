# SigmaOS Compile-Time Defenses, Build Hardening & Feature Gating Guide for AI Agents

This guide provides technical specifications, compile-time security invariants, `panic = "abort"` unwind prevention, compile-time feature gating rules, and build profile hardening guidelines for AI agents managing compile-time defenses in SigmaOS.

---

## 1. Zero-Dependency `#![no_std]` Compile-Time Security Architecture

SigmaOS enforces strict compile-time security guarantees at the compiler and build profile level (`Cargo.toml`):

* **`#![no_std]` Bare-Metal Isolation:**
  Prevents implicit standard library C runtime linkage or unsafe dynamic allocations in core kernel and subsystem modules.
* **Immediate Panic Abort (`panic = "abort"`):**
  Disables stack unwinding upon panic conditions in both `dev` and `release` build profiles, preventing stack unwinding exploit gadgets.
* **Compile-Time Feature Gating (`cfg(feature = "...")`):**
  Enforces explicit opt-in feature flags (e.g., `standalone_test`, `microkernel`, `custom_alloc_error_handler`) to minimize binary attack surface.

---

## 2. Compile-Time Defense Invariants & Build Rules

1. **Zero External Crates Rule:**
   `Cargo.toml` `[dependencies]` MUST remain empty. All algorithms, drivers, and utilities MUST be implemented natively in Rust.
2. **Opt-In Feature Gating:**
   Non-core features (such as `standalone_test` or `custom_alloc_error_handler`) MUST be gated with `#[cfg(...)]` attributes to prevent symbol collisions during standard builds.
3. **Panic Abort Policy:**
   Do NOT attempt to catch panics at runtime. The kernel relies on immediate termination and clean restart via supervisor engines.

---

## 3. Checklist for AI Agents Managing Compile-Time Defenses

1. **Verify Cargo.toml Profile Settings:** Ensure `panic = "abort"` is preserved across `[profile.dev]` and `[profile.release]`.
2. **Test Workspace Builds:**
   Verify compilation across standard and test feature configurations:
   ```bash
   cargo check
   ./run_sigma_tests.sh
   ```
