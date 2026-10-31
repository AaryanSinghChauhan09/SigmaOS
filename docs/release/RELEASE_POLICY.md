# SigmaOS Release Policy

## Release Readiness Tiers
SigmaOS strictly regulates release tiers. Narrative claims of release readiness are invalid without machine-verified evidence:

| Release Tier | Required Evidence | Blockers |
|---|---|---|
| **Tier 0: Experimental** | Build and unit tests pass (`cargo check --lib`, `cargo test --lib`) | None |
| **Tier 1: Developer Preview** | QEMU boot reaches defined shell/diagnostic state (`scripts/ci/qemu_boot_smoke.sh`) | Boot regressions |
| **Tier 2: Test Distribution** | Clean VM installation and non-destructive recovery path verified (`installer/tests/dry_run.rs`) | Installer corruption, lack of rollback |
| **Tier 3: Release Candidate** | Full regression suite, security review, and zero known unclassified panics | Open security advisories, unstable boot |
| **Tier 4: Stable** | Hardware matrix verified on targets, update/rollback verified, support policy active | Unverified hardware claims, broken update path |

---

## Release Cadence
- **Developer Previews**: Released monthly following verified boot-smoke evidence.
- **Release Candidates**: Released quarterly following successful completion of the pre-release checklist.
- **Stable Releases**: Released semiannually upon full hardware matrix sign-off and complete supply-chain audit.
