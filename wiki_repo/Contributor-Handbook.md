# 📖 SigmaOS Contributor Handbook

Welcome to the SigmaOS development community! This handbook contains coding standards, testing workflows, and bug triage processes designed to make SigmaOS highly reliable and maintainable.

---

## 🎨 Coding Standards

To ensure complete sovereignty and compile-time correctness under `#![no_std]` bare-metal environments, all developers must adhere to these coding standards:

### 1. Zero Raw Pointer Leaks
*   Always prefer safe abstractions, traits, and owned collections backed by global allocators (`Box`, `Vec`).
*   Where `unsafe` pointer operations are absolutely required (e.g., paging, MMIO, DMA buffers), document safety invariants explicitly:
    ```rust
    // SAFETY: The address must be 4KB page-aligned and mapped in the PML4.
    unsafe { core::ptr::write(v_addr as *mut u64, val); }
    ```

### 2. Strict Trait-Based Design
*   Always structure subsystem capabilities as object-oriented traits (`SupersetApplicationCapability`, `ShreddingStrategy`, `AnalysisStrategy`).
*   This ensures mock implementation testability without needing real hardware or external dependencies.

### 3. Avoid Modulo for Multiples Check
*   Do not write modulo calculations (e.g., `ticks % 4 == 0`). Instead, implement or use native Rust helpers like `.is_multiple_of(4)` to prevent non-idiomatic compiler warnings.

### 4. Grouped and Ordered Header Imports
*   Group all imports logically: Standard Library `std`, Core/Alloc crates, then third-party dependencies, and finally local crate structures.

---

## 🔬 Testing Workflow

### 1. Local Testing
Before submitting any changes, you must ensure that all library tests compile and pass successfully:

```bash
# Run the entire library unit test suite
cargo test --lib

# Run all integration & binary tests
cargo test --all-targets
```

### 2. Formatting Checks
SigmaOS enforces strict style compliance. Run the following command to format and check formatting rules:

```bash
# Format all Rust files in-place
cargo fmt

# Verify formatting compliance
cargo fmt -- --check
```

### 3. Smoke Testing
Execute the local smoke test script before committing:

```bash
# Set executable permission
git update-index --chmod=+x scripts/smoke-test.sh

# Run smoke test suite
./scripts/smoke-test.sh
```

---

## 🐞 Bug Triage Guidelines

To keep the repository robust, we use an automated pipeline for triage:

1.  **Issue Auto-Generation**: GitHub Actions check runs will automatically parse test output. If a test fails, a GitHub Issue is automatically opened with complete compiler logs, file paths, and failure annotations.
2.  **Reproduction Cases**: Every bug report must be accompanied by a failing unit test inside the corresponding module or `tests/integration_test.rs`.
3.  **Triage Badges**: Contributors are rewarded with automated achievements (e.g., "Doc Master" for doc edits, "Kernel Hacker" for scheduler/MM updates) to gamify and encourage contributions.
