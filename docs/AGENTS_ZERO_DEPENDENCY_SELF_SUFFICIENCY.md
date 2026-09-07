# AI Agent Zero-Dependency Self-Sufficiency Specification for SigmaOS

This document specifies operational rules for AI agents maintaining zero-dependency self-sufficiency in **SigmaOS**.

---

## 1. Architectural Self-Sufficiency Rules

AI agents extending or maintaining SigmaOS must adhere to the following rules:

1. **Zero External Crates**:
   - `Cargo.toml` must contain zero external dependencies under `[dependencies]`.

2. **Use `klib` Primitives**:
   - Utilize native `klib` structures (`klib::Vec`, `klib::String`, `klib::HashMap`, `klib::BTreeMap`, `klib::Slab`, `klib::Uuid`) instead of external third-party crates.

3. **Zero-Alloc Stack Formatting**:
   - Use `ZeroDependencyPrimitiveHub::format_u64_stack` and fixed stack buffers for Ring-0 string and numerical formatting.

4. **Native Hardware Drivers**:
   - Implement hardware drivers natively in pure Rust (`src/drivers/`) without external C vendor SDKs.

---

## 2. Verification Protocol

- Run `./run_sigma_tests.sh` to confirm zero-dependency build compliance and test suite execution.

---

*Maintained by the SigmaOS Core Architecture Steering Committee.*
