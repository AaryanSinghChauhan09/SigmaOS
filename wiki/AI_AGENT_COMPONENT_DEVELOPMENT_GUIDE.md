# 🤖 AI Agent Component Development Guide for SigmaOS

This guide defines the comprehensive execution protocol, architectural standards, and workflow directives for AI agents developing OS components for **SigmaOS**, drawing inspiration from world-class Linux and BSD distribution standards (Arch Linux, FreeBSD, OpenBSD, NixOS, Fedora, Alpine, Void, Omarchy).

---

## 🏛️ Core Architectural Standards

### 1. 100% Zero-Dependency & `#![no_std]` Rust Architecture
- **Self-Sufficiency**: Core kernel, system services, and userspace components must maintain zero external third-party crates in `Cargo.toml`.
- **Memory Primitives**: Use standard `alloc::` types (`Vec`, `String`, `BTreeMap`, `format!`) with custom memory allocators.
- **Safety First**: Prohibit `unsafe` blocks except for low-level hardware MMIO, PCI registers, or context switching, with documented safety invariants.

### 2. Linux & BSD Distro Component Inspirations
- **Arch Linux & ALPM**: Cleanroom package recipes, topological dependency resolution, and PKGBUILD purity.
- **FreeBSD Capsicum & OpenBSD Pledge/Unveil**: Granular file descriptor capability limits and strict path visibility sandboxing.
- **NixOS & Guix Hermetic CAS Store**: Content-addressed `/nix/store` closure verification and atomic generation rollbacks.
- **Omarchy & Quickshell**: Keyboard-driven tiling navigation, Quickshell plugin system (`omarchy.*` namespace), `cidata` unattended cloud-init installations, and single-pass system theme studio.

---

## 🛠️ AI Agent Workflow & Verification Protocol

### Step 1: Feature Implementation
1. Identify missing subsystem or component gaps based on `.md` plans or Wiki specifications.
2. Implement native Rust modules using zero-dependency OOP design patterns.
3. Export new public structs and traits in module `mod.rs` and `src/lib.rs`.

### Step 2: Autonomous Verification
1. **Standalone File Testing**: Every modified or created Rust file in `src/` must pass standalone compilation and test execution:
   ```bash
   mkdir -p build
   rustc --test --edition=2021 <filepath> -o build/test_bin && ./build/test_bin
   ```
2. **Master Test Suite Verification**:
   ```bash
   ./run_sigma_tests.sh
   ```

### Step 3: GitHub Wiki Transfer & Pull Request Formatting
- **Fully Implemented Features**: When a feature defined in a proposal `.md` file or Wiki specification is 100% completed and verified, transfer its documentation and status into the official GitHub Wiki (`wiki/`).
- **In-Progress Features**: Submit clean PR-formatted commits using the mandatory `jules-*` branch naming convention.

---

## 📜 Task Guidelines Summary
- **Branch Prefix**: `jules-<feature-name>`
- **Read-Only System Guard**: `/usr/share/omarchy/` is strictly read-only for end-user customization tasks; edit `~/.config/` instead.
- **Privilege Mode**: Use `sudo` for interactive terminal commands, `pkexec` for background tasks, and `--no-sudo --print` for debug output.
- **Zero Regression Policy**: All unit tests must pass before submitting PRs.
