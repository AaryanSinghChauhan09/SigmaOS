# SigmaOS Release Engineering & Tiering Policy

SigmaOS defines strict, evidence-based release tiers to govern quality, feature inclusion, and distribution readiness.

---

## Release Quality Tiers

| Tier Name | Quality Level | Required Evidence Gate |
|---|---|---|
| **Experimental** | Pre-alpha / Active Development | `cargo check --lib` and standalone unit tests pass cleanly. |
| **Developer Preview** | Functional Subsystem Testing | QEMU boot smoke test reaches defined Zenith shell state (`scripts/ci/qemu_boot_smoke.sh`). |
| **Test Distribution** | VM / Live ISO Validation | Clean VM installer dry-run, failure-injection rollback, and recovery runbook verified. |
| **Release Candidate (RC)** | Full Regression & Security Audit | Zero open high-severity security advisories, fuzzing triage clean, documentation updated. |
| **Stable** | Production / Hardware Distribution | Verified hardware matrix evidence, atomic update/rollback rehearsal, SBOM provenance generated. |

---

## Release Process Execution

Releases are executed quarterly using `scripts/release/prepare_release.sh` following the checklist in `docs/release/RELEASE_CHECKLIST.md`.
