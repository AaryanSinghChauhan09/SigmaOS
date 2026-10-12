# Runbook: Documentation and Onboarding Agent (`docs-sync`)

## Purpose
Ensure all documentation, onboarding guides, command references, and troubleshooting docs reflect the true, verified state of the repository.

## Procedure
1. Command Alignment Check:
   - Extract all code blocks and shell commands from `README.md` and `CONTRIBUTING.md`.
   - Verify commands match targets in `Makefile`, `Cargo.toml`, and `scripts/`.
2. Generate "What Works Today" Section:
   - Read `reports/capability/truth_table.json`.
   - Update `WHAT_IS_WORKING_AND_NOT_WORKING.md` or `README.md` dynamically with verified features only.
3. Identify Unsupported Promises:
   - Flag any statement in documentation promising unverified parity or production readiness without test logs.
4. Update Contributor Quickstart:
   - Confirm clone, build, unit test, and QEMU smoke test steps function cleanly on fresh environments.
