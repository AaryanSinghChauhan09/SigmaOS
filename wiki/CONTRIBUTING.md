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

## 🔹 2. Getting Started

- **Fork & Clone**: Fork the SigmaOS repository and clone your local copy.
  ```bash
  git clone https://github.com/your-username/SigmaOS.git
  cd SigmaOS
  ```
- **Rust Toolchain**: Ensure you have the Rust toolchain installed.
  ```bash
  rustup toolchain install stable
  ```
- **Build & Test**: Verify your setup by compiling and executing tests before making changes:
  ```bash
  cargo build
  ./run_sigma_tests.sh
  ```
- **Explore Ecosystem Modules**: Familiarize yourself with core modules including the **Zenith Desktop Compositor** (`src/desktop/`), **Shards Application Ecosystem** (`src/package/`), and **Kernel/Driver Architecture** (`src/kernel/`, `src/driver/`).

---

## 🔹 3. Contribution Workflow

- **Issues First**: Open an issue or join an existing discussion before commencing major architectural changes or new feature implementations.
- **Feature Branches**: Use descriptive feature branch names starting with a valid prefix (`feat/`, `fix/`, `docs/`, `refactor/`, `kernel/`, `pkg/`, `arch/`):
  ```bash
  git checkout -b feat/zenith-gesture-support
  ```
- **Pull Requests**: Submit pull requests against `main` with clear descriptions, linked issues, and detailed context.
- **Verification**: Ensure all unit tests pass, documentation is updated, and quality checks (`./scripts/sigma_quality_check.sh`) succeed.

---

## 🔹 4. Coding Standards & Mandatory Rules

### 4.1 Rules for Human Contributors
- **Zero External Dependencies Policy**: The core microkernel and 12 shards operate under `#![no_std]`. Never add third-party crates to `[dependencies]` in `Cargo.toml`.
- **Safe Rust & `unsafe` Documentation**: Memory safety is non-negotiable. Every `unsafe` block must include a `// SAFETY:` explanation.
- **PQC Security**: Cryptographic signing and verification must use Dilithium-5 or Kyber-1024.
- **Formatting & Linting**: Format code with `rustfmt` (`cargo fmt`) and check with Clippy (`cargo clippy`).
- **API Documentation**: Document all public modules, structs, traits, and functions using Rustdoc comments (`///`).

### 4.2 Rules for AI Agents
- **Planning & Review**: AI agents must propose plans via `request_plan_review` before calling `set_plan`.
- **Mandatory Pre-Commit Step**: Plans must include a step with the exact text: `Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.`
- **Post-Modification Verification**: Every file modification must be confirmed using a read-only tool (`read_file` / `list_files`).
- **Code Review**: AI agents must call `request_code_review` prior to submitting changes.
- **Secret Scanning Safeguards**: Mock keys and test tokens must be prefixed with `mock_` or `test_`.

---

## 🔹 5. Areas to Contribute

- **Kernel Subsystems**: Microkernel hybrid primitives, EEVDF scheduler, eBPF tracing, device drivers.
- **Userland & Utilities**: Universal shell compatibility, init system services, package manager adapters (`.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.nix`, `.xbps`, `.moss`).
- **Zenith Desktop**: Compositor layout engines, GTK3/GTK4 native UI toolkit adapters, display management.
- **Shards Ecosystem**: Sandboxed applications, productivity suites, multimedia tools, and security auditing stacks.
- **Documentation & Wiki**: Architecture guides, API reference docs, installation manuals, and tutorial examples.

---

## 🔹 6. Community Engagement

- **GitHub Discussions**: Participate in roadmap discussions, strategic planning, and architectural reviews.
- **Sprints & Sagas**: Join community hackathons, bug triages, and release sprints.
- **Future Feature Ideas**: Share and collaborate on visionary features such as temporal filesystems, clustered peripheral virtualization, and autonomous AI system governors.

---

*Thank you for helping build a sovereign, resilient, and ultra-modular operating system with SigmaOS!*

---

## 📜 Contributor Charter & Governance Principles
All contributions to SigmaOS must uphold the core principles defined in the **SigmaOS Contributor Charter** (`docs/GOVERNANCE_CHARTER.md`):
- **Sovereignty**: Code contributions must remain transparent and free of closed proprietary vendor lock-in.
- **Clarity**: Architecture, code structure, and documentation must be explicit and self-documenting.
- **Resilience**: Every modification must preserve $O(1)$ state rollback safety and fault tolerance.
- **Security**: Non-negotiable Safe-Rust memory safety, Post-Quantum Cryptography (Dilithium-5/Kyber), and capability sandboxing (`pledge`/`unveil`).
