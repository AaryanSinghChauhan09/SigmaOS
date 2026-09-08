# 🤝 Contributing to SigmaOS

Thank you for contributing to **SigmaOS**! This document provides task guidelines inspired by Linux & BSD distribution contributor standards.

---

## Task Guidelines & Contribution Rules for Human Contributors and AI Agents

### 1. **Branch Naming & Commit Workflow**
- All git branches MUST start with `jules-` (e.g. `jules-feature-scheduler`, `jules-fix-pam`).
- Commit messages must follow standard conventions: short subject line (50 chars max), blank line, and descriptive body detailing changes.
- Keep commits granular, logical, and focused on single task objectives.

### 2. **Zero Dependencies & Distro Parity Standards**
- All kernel and userspace components must maintain strict `#![no_std]` zero external dependency design.
- Contributions taking inspiration from Linux & BSD distributions (Arch Linux ALPM, Debian sbuild, Fedora DNF, FreeBSD Ports, OpenBSD Pledge/Unveil, NixOS Flakes) must include unit tests.
- Do not add unverified crates under `[dependencies]` in `Cargo.toml`.

### 3. **AI Agent Development & Verification Directives**
- AI agents working on SigmaOS must follow the planning, execution, and verification workflow defined in `AGENTS.md` and `DEVELOPER_RULES.md`.
- AI agents MUST request plan reviews before initiating multi-file modifications and verify all code changes using read-only tools or test execution scripts (`./run_sigma_tests.sh`).
- AI agents must record critical learnings using `initiate_memory_recording` upon task completion.

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

## 🔹 4. Coding Standards

- **Formatting & Linting**: Format code with `rustfmt` (`cargo fmt`) and check with Clippy (`cargo clippy`).
- **Modular Architecture**: Keep components small, composable, and loosely coupled under `#![no_std]` / `alloc` capability bounds.
- **API Documentation**: Document all public modules, structs, traits, and functions using Rustdoc comments (`///`).
- **Security & Safety First**: Prefer safe Rust. Avoid `unsafe` blocks unless explicitly required for low-level driver/hardware interface interop, and document all safety invariants.
- **PQC Cryptographic Verification**: Ensure driver modules and package recipes adhere to post-quantum signature verification models.

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
