# Daily Health Runbook — SigmaOS Periodic Improvement Program

## Objective
Maintain clean build, zero unexpected test failures, and healthy QEMU boot status.

## Cadence
Executed daily by CI and automated maintenance agents.

## Workflow Steps
1. Execute `cargo check --lib`
2. Execute `cargo test --lib`
3. Execute `./run_sigma_tests.sh`
4. Run QEMU x86_64 boot-to-shell smoke test
5. Triage any failures and generate daily status log in `research/reports/daily/`
