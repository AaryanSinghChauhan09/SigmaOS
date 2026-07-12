# Phase 4: CI/CD & Testing

## Overview

SigmaOS uses a multi-dimensional CI matrix to validate every push to `main` across all build profiles and OS targets. Testing is inspired by Ubuntu's autopkgtest framework but adapted for the SigmaOS microkernel.

---

## CI Matrix

Every push to `main` or `lattice-dev` triggers:

| Dimension | Values |
|-----------|--------|
| `profile` | `standalone`, `microkernel`, `cloud` |
| `target_os` | `sigma`, `ubuntu`, `bsd` |
| **Total jobs** | **9 parallel jobs** |

### Workflow Files

| File | Purpose |
|------|---------|
| [`.github/workflows/ci.yml`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/.github/workflows/ci.yml) | Core build + Rust static analysis per `target_os` |
| [`.github/workflows/sigma_ci.yml`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/.github/workflows/sigma_ci.yml) | Full 3×3 matrix: `profile` × `target_os` |

---

## Test Suites

### Unit Tests (Rust no_std)

```bash
cargo test --target x86_64-unknown-linux-gnu
```

### Current status: Full Rust migration

| Suite | Tests |
|-------|-------|
| `tests/sovereign/format_tests.rs` | Universal OS Format validation |
| `tests/sovereign/kernel_tests.rs` | Core kernel module validation |

### Format Validation Tests (Rust)

The test file enforces:

1. **Compile-time guard**: Fails immediately if no `TARGET_OS_*` define is set via `#[cfg(feature = "...")]`

2. **Mutual-exclusivity**: Fails if more than one `TARGET_OS_*` define is active.

3. **Profile assertions**: Validates driver availability, POSIX state, and key feature flags per profile.

### Static Analysis (Clippy)

Runs on every CI push:

```bash
cargo clippy --target x86_64-unknown-none -- -D warnings
```

---

## Hardware Validation Matrix

| Architecture | Target | Status |
|-------------|--------|--------|
| x86_64 | All | ✅ CI Validated |
| ARM64 | sigma, ubuntu | 🔄 Planned Phase 4B |
| RISC-V 64 | sigma | 🔄 Planned Phase 4C |
| AI NPU/TPU | sigma-cloud | 🔄 Planned Phase 6 |

---

## Release Cadence

| Channel | Cadence | LTS? |
|---------|---------|------|
| `nightly` | Every push to `main` | No |
| `testing` | Weekly snapshot | No |
| `stable` | Quarterly | Yes (2-yr support) |
| `lts` | Annual | Yes (5-yr support) |

---

## 🔗 Related Pages

- [Phase 3: Package & Update System](Phase-3-Package-And-Update-System)

- [Phase 5: Ecosystem & Developer Tools](Phase-5-Ecosystem-And-Developer-Tools)

- [Testing Guide](Testing-Guide)

- [CI Pipeline](CI-Pipeline)
