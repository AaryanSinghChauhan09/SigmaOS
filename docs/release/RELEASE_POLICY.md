# SigmaOS Release Engineering Policy

## Release Tiers
SigmaOS releases are classified into five strict tiers based on evidence, non-negotiable quality gates, and runtime verification.

### Tier 1: Experimental
- **Definition:** Early feature additions or exploratory subsystem branches.
- **Evidence Requirement:** Code compiles (`cargo check --lib`), unit tests pass (`cargo test --lib`). No boot or installation guarantee.

### Tier 2: Developer Preview
- **Definition:** Functional kernel/userspace builds suitable for QEMU testing.
- **Evidence Requirement:** QEMU x86_64 boot-to-shell smoke test passes (`scripts/ci/qemu_boot_smoke.sh`).

### Tier 3: Test Distribution
- **Definition:** Full system ISO suitable for testing in VM live environments.
- **Evidence Requirement:** Installer dry-run suite passes (`installer/tests/dry_run.rs`) and recovery runbook validated (`docs/operations/RECOVERY_RUNBOOK.md`).

### Tier 4: Candidate
- **Definition:** Near-production candidate for external developers.
- **Evidence Requirement:** Full regression test suite passes (`./run_sigma_tests.sh`), zero unhandled Clippy warnings, security policy verified.

### Tier 5: Stable
- **Definition:** Production daily-driver target.
- **Evidence Requirement:** Verified hardware matrix (`docs/platform/HARDWARE_SUPPORT.md`), atomic update/rollback test, and formal sign-off.
