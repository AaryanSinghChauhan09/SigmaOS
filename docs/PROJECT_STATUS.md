# SigmaOS Project Status & Metrics Matrix

## Core Metrics Ledger

| Metric | Target | Current Status | Verification Source |
| :--- | :--- | :--- | :--- |
| **QEMU x86_64 Boot-to-Shell Success** | 100% on CI | 100% Pass | `tests/boot/boot_to_shell_test.py` / `.github/workflows/boot-smoke.yml` |
| **Boot Smoke Test Duration** | Under 5 mins | ~35s | `scripts/ci/qemu_boot_smoke.sh` |
| **Installer Dry-Run Pass Rate** | 100% | 100% Pass | `installer/tests/dry_run.rs` |
| **Installer Recovery Failure Test** | 100% Pass | 100% Pass | `installer/tests/failure_injection.rs` |
| **PRs with Evidence / Test Exemption**| 100% | Enforced | `.github/PULL_REQUEST_TEMPLATE.md` + `COMPONENT_RULES.toml` |
| **Security Review on Sensitive PRs** | 100% | Enforced | `SECURITY_ARCHITECTURE.md` + `docs/security/CAPABILITY_POLICY.md` |
| **Components with Documented Limits**| 100% | Enforced | `docs/PROCESS.md` + `docs/adr/0001-evidence-based-component-status.md` |
