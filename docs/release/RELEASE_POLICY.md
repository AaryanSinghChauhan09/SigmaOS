# SigmaOS Release Policy and Stability Tiers

## Stability Tiers

| Release Tier | Required Evidence Gate | Primary Deliverable |
|---|---|---|
| **Experimental** | `cargo check --lib` and `cargo test --lib` pass | Source code commit |
| **Developer Preview** | QEMU boot reaches defined shell or diagnostic prompt | Boot ISO image |
| **Test Distribution** | Clean VM installation and recovery path rehearsal verified | Installer ISO / USB image |
| **Release Candidate** | Full regression pass, security audit, hardware evidence matrix | Version-tagged ISO artifact |
| **Stable** | Hardware matrix, update/rollback, support policy, and security process verified | Signed release ISO & packages |

---

## Release Pipeline Script

Run `scripts/release/prepare_release.sh` to execute all pre-release validation checks.
