# Contributing to SigmaOS & Contributor Charter

Welcome to **SigmaOS**! We welcome contributions from developers, security researchers, accessibility advocates, and institutions building the post-Linux sovereign operating system.

---

## 📜 Contributor Charter & Governance Principles

All contributions to SigmaOS must uphold the core principles defined in the **SigmaOS Contributor Charter** (`docs/GOVERNANCE_CHARTER.md`):

1. **Sovereignty**: Code contributions must remain transparent and independent from proprietary binary blobs or opaque vendor lock-in.
2. **Clarity**: Code structure, Rust traits, and documentation must be explicit, declarative, and self-documenting.
3. **Resilience**: Modifications to kernel shards, VFS storage, or package managers must preserve $O(1)$ state rollback guarantees.
4. **Security**: All code must enforce Safe-Rust memory safety, Post-Quantum Cryptography (Dilithium-5/Kyber) verifications, and least-privilege capability sandboxing (`pledge`/`unveil`).

---

## 🛠️ How to Contribute

### 1. Development Workflow
- Fork the repository and create a feature branch (`jules-<feature-name>`).
- Follow strict `#![no_std]` bare-metal zero-dependency Rust coding standards.
- Run `cargo check --lib` and `./run_sigma_tests.sh` to ensure all tests pass with zero warnings or errors.

### 2. Contributor Roles
- **Core Shard Maintainers**: Oversee the 12 microkernel shards (Media, Networking, Storage, AI, Compositor, Drivers, Security, Virtualization, System, Package, IPC, Hardware).
- **Community Contributors**: Submit bug fixes, documentation updates, translations, and accessibility improvements.
- **Institutional Partners**: Academic labs and sovereign entities deploying cluster-native hardware nodes.

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
