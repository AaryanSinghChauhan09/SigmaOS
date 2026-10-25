# SigmaOS Release Policy

This document defines the release readiness tiers, evidence requirements, versioning conventions, and release engineering criteria for SigmaOS.

## Release Readiness Tier Matrix

| Release Tier | Classification | Required Evidence & Verification Gates |
|---|---|---|
| **Experimental** | Tier 0 | Builds compile cleanly via `cargo check --lib`, unit tests pass (`cargo test --lib`). |
| **Developer Preview** | Tier 1 | QEMU boot reaches defined shell/diagnostic marker (`SIGMAOS_BOOT_SUCCESS`) in automated smoke harness. |
| **Test Distribution** | Tier 2 | Clean VM installation and installer recovery path verified in monthly rehearsal (`docs/operations/RECOVERY_RUNBOOK.md`). |
| **Release Candidate** | Tier 3 | Full regression test suite pass (`./run_sigma_tests.sh`), security advisory triage complete, known-issue list documented. |
| **Stable** | Tier 4 | Hardware support matrix verified, update/rollback rehearsal passed, supply-chain SBOM generated, LTS support policy active. |

---

## Semantic Versioning Guidelines
SigmaOS follows Semantic Versioning 2.0.0 (`MAJOR.MINOR.PATCH`):
- `MAJOR`: Architecture breaking changes, ABI breaks, kernel capability policy updates.
- `MINOR`: New subsystem additions, major driver expansion, backwards-compatible API additions.
- `PATCH`: Security fixes, bug repairs, documentation updates, dependency refreshes.

---

## Release Script Automation
Official release artifacts (ISO, kernel binaries, sigpkg repos) are built using:

```bash
# Execute release preparation script
./scripts/release/prepare_release.sh --tier test_distribution
```
