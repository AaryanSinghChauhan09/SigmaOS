# 🚀 SigmaOS Pull Request Checklist & Submission Guide

## 📋 Overview
- **Title**: *[Component / Subsystem]: Concise description of change (max 50 chars)*
- **Type**: `[Bugfix / Feature / Driver / Optimization / Security / Documentation / Refactor]`
- **Subsystem**: `[Kernel / Memory / IPC / Drivers / Network / VFS / Security / Distro / Compatibility]`

---

## 🛡️ Architecture & Integrity Checklist
Every pull request in **SigmaOS** must strictly comply with kernel architecture and safety rules:

- [ ] **`#![no_std]` Strict Compliance**: Verification performed ensuring zero third-party crates or `std` usages in kernel and driver space (except `#[cfg(test)]`).
- [ ] **Capability Token Validation**: Capability checks (`CapabilityToken` / `verify_token`) are enforced on all public entrypoints and syscall dispatchers.
- [ ] **Explicit Type Annotations**: Public APIs and key state structs include explicit type declarations.
- [ ] **Memory Pool Safety**: Distinction between `Paged` and `NonPaged` memory allocations is preserved, and bounds clamping is verified on all buffer copies.
- [ ] **Driver Lifecycle Architecture**: Driver PRs adhere to standard `DriverObject`, `DeviceObject`, and `DeviceExtension` structures with explicit create/attach/detach/destroy unit tests.
- [ ] **Zero Unsafe Warnings**: Any `unsafe` block includes explicit `// SAFETY:` rationale and bounds checks.

---

## 🧪 Testing & Verification
- [ ] **Standalone File Test Execution**: Passed standalone module unit test:
  ```bash
  rustc --test --edition=2021 <path_to_file.rs> -o build/test_bin && ./build/test_bin
  ```
- [ ] **Master Test Runner Verification**: Passed `./run_sigma_tests.sh` with zero failing assertions.
- [ ] **Automated Script Audits**: Passed `./scripts/no_std_check.sh`.

---

## 📝 Summary of Changes
Provide a clear description of the problem solved, design choices made, and testing results:
1. ...
2. ...
3. ...
