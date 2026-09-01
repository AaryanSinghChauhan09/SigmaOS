# 📖 SigmaOS Next Steps Guidelines & Developer Handbook

## Overview
This document serves as the operational guide for developers, contributors, and multi-agent systems contributing to **SigmaOS**. It outlines contribution workflows, code standards, security practices, performance expectations, and release governance.

---

## 1. Developer Onboarding & Environment Setup

### 1.1 Prerequisites & Toolchain
- **Rust Toolchain**: Rust 2021 Edition (`rustc 1.80+` recommended).
- **Python**: Python 3.10+ with `pytest` installed.
- **Node.js**: Node.js v18+ with `npm` / `pnpm` for frontend components.
- **System Dependencies**: Standard C build tools (`gcc`, `make`, `cmake`).

### 1.2 Initializing the Workspace
```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Run core Python test suite
pytest

# Verify Rust library compilation
cargo check --lib
```

---

## 2. Contribution Workflow & Code Standards

### 2.1 Branching Strategy
- **`main`**: Production-ready code base. All commits to `main` must pass CI checks.
- **Feature Branches**: Format as `feat/<short-description>` or `fix/<short-description>`.
- **Agent Branches**: Bolt (`bolt/<optimization>`), Palette (`palette/<ux-fix>`), Sentinel (`sentinel/<security-fix>`).

### 2.2 Code Style & Quality Guidelines
- **Rust**:
  - Run `cargo fmt --check` before opening pull requests.
  - Fix warnings using `cargo clippy`.
  - Guard `no_std` environments properly when working in core kernel modules (`src/klib/`, `src/kernel/`).
- **Python**:
  - Follow PEP 8 guidelines.
  - Ensure tests are placed in `tests/` and pass with `pytest`.
- **TypeScript / Web UI**:
  - Use semantic HTML tags and explicit ARIA labels (`aria-label`, `role`).
  - Ensure focus rings are visible (`focus-visible:ring-2`).

---

## 3. Agent Operating Guidelines

### ⚡ Bolt Agent Guidelines (Performance)
- **Focus**: Speed, allocation reduction, execution profiling.
- **Rule**: Always measure before and after optimization. Keep changes targeted (<50 lines).
- **Journal**: Document learnings in `.jules/bolt.md`.

### 🎨 Palette Agent Guidelines (UX & Accessibility)
- **Focus**: Visual polish, screen-reader accessibility, keyboard navigation, helpful empty states.
- **Rule**: Ensure ARIA tags, contrast ratios, and keyboard focus indicators are maintained.
- **Journal**: Document UX patterns in `.jules/palette.md`.

### 🛡️ Sentinel Agent Guidelines (Security & Compliance)
- **Focus**: Privilege separation, input sanitization, path traversal prevention, crypto validation.
- **Rule**: Fail safely, sanitize all external inputs, avoid hardcoded keys/secrets.
- **Journal**: Document security findings in `.jules/sentinel.md`.

---

## 4. Testing & CI Pipeline Guidelines

### 4.1 Running Tests Locally
```bash
# Python unit & integration tests
pytest

# Standalone standalone module test execution pattern (e.g. data engine)
rustc --test src/tools/data_engine.rs --edition=2021 -o /tmp/test_data_engine && /tmp/test_data_engine

# Run complete CI test script
./run_sigma_tests.sh
```

### 4.2 GitHub Actions Workflows
- **`sigma-ci.yml`**: Triggers on push and pull requests to `main`. Executes test suites, formatting checks, and documentation generation.
- **`pages-distro-wiki.yml`**: Deploys documentation to GitHub Pages.

---

## 5. Architectural Roadmap & Future Development

### Phase 1: Hardening & Parity Stabilization
- Complete compiler warning resolution across all package management sub-modules.
- Expand FreeBSD jail hierarchy validation and OpenBSD pledge enforcement.

### Phase 2: Observability & Diagnostics
- Integrate eBPF real-time tracepoints into `SovereignFastBootServicePipeline`.
- Enhance NVIDIA PRIME hybrid graphics auto-switching diagnostics.

### Phase 3: Cloud & Container Integration
- Expand Nix derivation engine closures and Alpine APK repository mirror syncing.
- Implement live snapshot rollback triggers for system service updates.

---

## 6. Release Governance & Versioning
- **Semantic Versioning**: Follow `MAJOR.MINOR.PATCH` format (e.g., `v15.0.0`).
- **Release Verification**:
  1. All test suites (`pytest`, `cargo test`) must pass cleanly.
  2. Release notes draft generated using `./scripts/gen_changelog.sh`.
  3. Artifacts signed using GPG key verification via `./scripts/sign_release.sh`.
