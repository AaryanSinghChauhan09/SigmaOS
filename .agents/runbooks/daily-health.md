# Daily Health & Status Triage Runbook

## Objective
Maintain continuous build health, unit test integrity, and QEMU boot validity across all SigmaOS system modules.

## Frequency
Daily (Automated / Continuous Integration)

## Steps
1. Execute repository compilation checks:
   ```bash
   cargo check --lib
   ```
2. Run standard unit test suite:
   ```bash
   cargo test --lib
   ```
3. Run native SigmaOS system test runner:
   ```bash
   ./run_sigma_tests.sh
   ```
4. Verify QEMU boot status and log output:
   ```bash
   ./scripts/boot_matrix.py --check
   ```
5. Record health results in `reports/steward/daily-status.md`.
6. Triage any compilation or test regressions immediately.
