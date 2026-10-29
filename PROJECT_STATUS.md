# SigmaOS Project Status & Evidence Metrics

This document tracks operational success metrics for SigmaOS components, enforcing evidence-based status declarations.

## Success Metrics

| Metric | Target | Current Status | Verification Source |
| :--- | :--- | :--- | :--- |
| **QEMU x86_64 boot-to-shell success** | 100% on CI | 100% Verified | `scripts/ci/qemu_boot_smoke.sh`, `tests/boot/boot_to_shell_test.py` |
| **Boot smoke-test duration** | Under 5 minutes | ~15 seconds | `.github/workflows/boot-smoke.yml` |
| **PRs with tests or documented exemption** | 100% | Enforced | `.github/PULL_REQUEST_TEMPLATE.md` |
| **Components with documented limitations** | 100% | Enforced | `COMPONENT_RULES.toml`, `docs/` |
| **Security-sensitive PRs with security review** | 100% | Enforced | `docs/security/CAPABILITY_POLICY.md` |
| **Installer dry-run pass rate** | 100% | 100% Verified | `installer/tests/install_dry_run.rs` |
| **Driver claims backed by evidence** | 100% | 100% Verified | `docs/platform/HARDWARE_SUPPORT.md`, `tests/hardware/` |
| **Open gaps with owner and acceptance criteria** | 100% | Tracked | `.github/ISSUE_TEMPLATE/gap.md` |

---

## Release Readiness Tiers

- **Experimental**: Buildable, unit tests pass, no boot guarantee (`cargo check`, `cargo test`).
- **Developer Preview**: Boots in QEMU to defined prompt (`scripts/ci/qemu_boot_smoke.sh`).
- **Test Distribution**: Installable in controlled VM (`installer/tests/install_dry_run.rs`).
- **Candidate**: Suitable for limited external testing (Full regression + known-issue list).
- **Stable**: Supported daily-driver target (Hardware matrix, update/rollback, security policy).
