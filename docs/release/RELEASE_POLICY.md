# SigmaOS Release Tier Policy & Engineering Criteria

## Release Tiers Definition

| Tier | Name | Required Verification Evidence |
| :--- | :--- | :--- |
| **Tier 1** | **Experimental** | Successful `cargo check --lib` and passing unit tests. No boot guarantee required. |
| **Tier 2** | **Developer Preview** | Boots cleanly in QEMU `x86_64` to deterministic shell prompt marker. |
| **Tier 3** | **Test Distribution** | Clean installation pass in QEMU VM with dry-run and recovery test verification. |
| **Tier 4** | **Candidate** | Full automated test suite pass (`./run_sigma_tests.sh`) and zero open P0/P1 blockers. |
| **Tier 5** | **Stable** | Hardware evidence matrix filled for target profiles, security policy review signoff, and update rollback verified. |
