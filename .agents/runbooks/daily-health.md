# Daily Health Runbook

## Objective
Keep build, tests, and QEMU boot status healthy.

## Cadence
Daily execution by agent/maintainer.

## Procedure
1. Run static check: `cargo check --lib`
2. Run unit tests: `cargo test --lib`
3. Run full SigmaOS test suite: `./run_sigma_tests.sh`
4. Run formatting check: `make format`
5. Triage any failures and generate daily status log in `research/reports/daily/`.
