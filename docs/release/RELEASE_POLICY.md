# SigmaOS Release Policy & Quality Readiness Tiers

## Release Tiers & Acceptance Criteria

| Tier | Meaning | Minimum Required Evidence |
| :--- | :--- | :--- |
| **Experimental** | Buildable codebase, unit tests may pass, no boot guarantee | `cargo check --lib`, standalone unit tests passing |
| **Developer Preview** | Boots in QEMU to defined shell marker | QEMU boot smoke log archived (`scripts/ci/qemu_boot_smoke.sh`) |
| **Test Distribution** | Installable in controlled VM / QEMU | Clean install + dry-run + recovery runbook test passing |
| **Candidate** | Suitable for limited external testing | Full regression suite (`./run_sigma_tests.sh`) + zero open security blockers |
| **Stable** | Daily-driver ready | Hardware matrix evidence, atomic update/rollback verified, security review signed off |

---

## Release Script & Workflow
Releases are prepared via `scripts/release/prepare_release.sh` and automated via `.github/workflows/release.yml`.
