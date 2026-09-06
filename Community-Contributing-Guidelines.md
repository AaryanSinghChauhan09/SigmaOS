# 📑 SigmaOS Contributor Guidelines

Thank you for your interest in contributing to SigmaOS! SigmaOS is a sovereign, modular, post-quantum resilient operating system. These guidelines set clear expectations, facilitate onboarding, and provide a structured path for community collaboration.

---

## 🔹 1. Code of Conduct

- **Respectful Communication**: Maintain respectful, professional communication in GitHub issues, pull requests, and discussions.
- **Zero Tolerance**: Discrimination, harassment, or non-constructive behavior will not be tolerated.
- **Collaborative Spirit**: Assume good intent, offer constructive feedback, and focus on collaborative solutions that advance digital sovereignty.

---

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
