# 🤝 Contributing to SigmaOS

Thank you for contributing to **SigmaOS**! This document provides development guidelines, code quality standards, and contribution workflows inspired by Linux kernel maintainers and BSD distribution standards.

---

## 📜 Rules for Contributors

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

### 3. **Testing & Code Review**
- Run standalone tests on modified files (`rustc --edition=2021 --test <file>`).
- Execute `./run_sigma_tests.sh` to ensure all 13 native test stages pass cleanly.
- Pull requests require double maintainer code review before merging.
