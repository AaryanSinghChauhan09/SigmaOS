# SigmaOS Release Engineering Policy

## Release Tiers

| Tier | Meaning | Required Evidence |
| :--- | :--- | :--- |
| **Experimental** | Buildable, tests may pass, no boot guarantee | `cargo check --lib`, unit tests pass |
| **Developer Preview** | Boots in QEMU to defined prompt | `scripts/ci/qemu_boot_smoke.sh` output + smoke test pass |
| **Test Distribution** | Installable in controlled VM | Clean dry-run install + recovery rollback pass |
| **Candidate** | Suitable for limited external testing | Full regression suite + no open release-blocking issues |
| **Stable** | Supported daily-driver target | Hardware matrix evidence, update/rollback verification, security review |

## Release Engineering Rules
- No status promotion from Experimental or Preview to Stable without hardware ledger evidence (`docs/platform/HARDWARE_SUPPORT.md`).
- Release binaries must be prepared using `scripts/release/prepare_release.sh` and tracked in `.github/workflows/release.yml`.
