# SigmaOS Quarterly Release Readiness Checklist

## Checklist Overview

Before any version tag is cut or published, the Release Manager must verify and sign off on all items in this checklist.

## Verification Checklist

### 1. Build & Continuous Integration
- [ ] Main branch CI status is 100% green (`.github/workflows/sigma_multiarch_ci.yml`).
- [ ] `./run_sigma_tests.sh` executes with 0 failures across all test suites.
- [ ] No compilation warnings on `no_std` kernel targets.

### 2. Platform Boot & Installer Safety
- [ ] QEMU boot-to-shell smoke test passes (`scripts/ci/qemu_boot_smoke.sh`).
- [ ] Installer dry-run safety test passes (`installer/tests/dry_run.rs`).
- [ ] Installer failure injection and rollback test passes (`installer/tests/failure_injection.rs`).

### 3. Security & Governance
- [ ] Security advisory review completed without unresolved critical CVEs.
- [ ] `COMPONENT_RULES.toml` compliance audit verified.
- [ ] Hardware evidence matrix updated in `docs/platform/HARDWARE_SUPPORT.md`.
- [ ] Documentation and feature matrix (`FEATURE_STATUS.toml`) synchronized with codebase reality.
