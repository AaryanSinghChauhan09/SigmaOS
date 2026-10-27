# Runbook: Build and Test Verification

## Agent Role
Build and Test Sentinel (`sentinel`)

## Workflow Steps
1. Perform clean workspace check (`make clean`).
2. Run `cargo check --lib`.
3. Run `cargo test --lib`.
4. Execute `./run_sigma_tests.sh`.
5. Run `make check`, `make test`, and `make format`.
6. Output execution summary manifest to `reports/test/test-manifest.json`.
7. If failure is mechanical (formatting, missing import), generate repair PR.
