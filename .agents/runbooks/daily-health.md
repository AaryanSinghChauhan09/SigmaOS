# Daily Health Runbook

## Purpose
Keep build, tests, and boot status healthy across SigmaOS components.

## Steps
1. Execute clean build check: `cargo check --lib`.
2. Run core unit tests: `cargo test --lib`.
3. Run native test runner: `./run_sigma_tests.sh`.
4. Run QEMU boot test and record serial log in `reports/boot/qemu-x86_64.log`.
5. Triage any test failures or boot panics and record in daily status report under `research/reports/daily/`.
