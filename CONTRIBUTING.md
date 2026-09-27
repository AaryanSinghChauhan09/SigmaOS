# 🤝 Contributing to SigmaOS: Guidelines for Human Contributors & AI Agents

Thank you for contributing to **SigmaOS**! This document provides development guidelines, code quality standards, and contribution workflows inspired by Linux kernel maintainers and BSD distribution standards.

For detailed rules on kernel development, sandboxing, error handling, and PR requirements, please refer to the comprehensive [SIGMAOS_DEVELOPMENT_RULES_AND_GUIDELINES.md](SIGMAOS_DEVELOPMENT_RULES_AND_GUIDELINES.md).

---

## 📜 Core Rules for Contributors

### 1. **Zero External Dependencies Policy**
- SigmaOS strictly adheres to a **zero-dependency `#![no_std]`** design philosophy across kernel, hardware abstractions, and system services.
- **Do NOT add third-party crates** to `Cargo.toml`.
- All abstractions must use core Rust or `alloc::` primitives (`alloc::vec::Vec`, `alloc::string::String`, `alloc::format`).

### 2. **Code Quality, Safety & Testing**
- **Safe Rust First:** Avoid `unsafe` blocks unless interfacing directly with MMIO registers, CPU instructions, or FFI. Always document `// SAFETY:` invariants for any `unsafe` usage.
- **No Panics:** Avoid `unwrap()`, `expect()`, or panicking logic in production paths. Gracefully return `Option` or `Result`.
- **Mandatory Unit Testing:** Every new feature, bug fix, or security enhancement must include comprehensive unit tests (`#[cfg(test)] mod tests`).
- **Full Verification:** All changes must pass `./run_sigma_tests.sh` and standalone test compilation (`rustc --edition=2021 --test <file_path>`).

### 3. **Security & Sandboxing Standards**
- Implement security controls following defense-in-depth principles: OpenBSD `pledge`/`unveil`, Linux Landlock v5, FreeBSD Capsicum descriptors, and SELinux MAC.
- All network packets, input parameters, and package manifests must undergo strict validation against path traversal, octal differential, and CLI option injection attacks.

### 4. **Branch Naming & Commit Workflow**
- Branch names should follow descriptive prefixes (`feat/`, `fix/`, `docs/`, `security/`, `perf/`, `jules-`).
- Keep commits atomic, well-tested, and accompanied by clear commit messages adhering to standard git conventions (50-char subject, blank line, body).

---

## 📋 Pull Request Checklist & Pre-Commit Protocol

Before opening a pull request or submitting code changes:

1. **Verify Standalone Test Compilation:**
   ```bash
   mkdir -p build && rustc --edition=2021 --test <file_path> -o build/test_mod && ./build/test_bin
   ```
2. **Execute Full Test Suite:**
   ```bash
   ./run_sigma_tests.sh
   ```
3. **Check Formatting & Quality:** Ensure `cargo fmt` standards are maintained and no unhandled panics or unwraps exist in production paths.
4. **Follow PR Guidelines:** Follow the detailed steps in [SIGMAOS_DEVELOPMENT_RULES_AND_GUIDELINES.md](SIGMAOS_DEVELOPMENT_RULES_AND_GUIDELINES.md).
