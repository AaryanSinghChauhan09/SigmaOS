# 🚀 SigmaOS Pull Request Checklist & Quality Gates

Thank you for contributing to **SigmaOS**! Before submitting your PR, please verify that your changes adhere to SigmaOS architecture rules and quality standards.

---

## 📌 PR Type
- [ ] 🐛 Bug Fix
- [ ] ⚡ Performance Optimization
- [ ] 🛡️ Security / Capability Model Hardening
- [ ] 🔌 Driver / Hardware Abstraction
- [ ] 📦 Package Manager / Distro Feature
- [ ] 📖 Documentation / Ops Guide

---

## 📋 Architectural Verification Checklist

### 1. `no_std` Compliance & Memory Safety
- [ ] All code in `kernel/`, `drivers/`, `klib/`, and `src/` complies strictly with `no_std` execution constraints (no unverified direct `std::` imports outside `#[cfg(test)]`).
- [ ] Executed `./scripts/no_std_check.sh` locally and confirmed 100% compliance.

### 2. Capability Token Verification (`verify_token`)
- [ ] All new or modified syscall entrypoints verify incoming capability tokens (`CapabilityToken` / `verify_token`) before servicing requests.
- [ ] Included unit tests verifying token rejection for unauthorized or invalid tokens.

### 3. WDM Driver Object Lifecycle Standards
*(Mandatory for Driver PRs)*
- [ ] Implemented proper `DriverObject`, `DeviceObject`, and `DeviceExtension` driver abstractions.
- [ ] Included driver lifecycle unit tests covering device creation, attachment, detach, and cleanup.

### 4. Memory Pool Separation & Bounds Checking
- [ ] Explicitly verified allocations against **Paged** vs. **NonPaged** memory pool boundaries.
- [ ] All `copy_nonoverlapping` or unsafe pointer operations are strictly bounds-checked and clamped.

### 5. Code Style & Testing
- [ ] All public functions, structs, and non-trivial fields have explicit type annotations.
- [ ] All `unsafe` blocks are documented with `// SAFETY:` invariants justification.
- [ ] Verified standalone compilation and unit tests using `./scripts/changed_files_rustc_tests.sh`.
- [ ] Verified atomic test suite execution with `./run_sigma_tests.sh`.

---

## 🔗 Related Issues & Documentation
- Fixes/Relates to: `#`
- Relevant Spec/Doc Link: `docs/` or `wiki/`
