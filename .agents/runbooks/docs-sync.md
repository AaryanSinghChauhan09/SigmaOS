# Runbook: Documentation & Onboarding Synchronization

## Agent Role
Documentation and Onboarding Agent (`docs-onboarding`)

## Workflow Steps
1. Parse all public CLI commands and make targets in `README.md`.
2. Cross-reference commands against `Makefile` targets and `scripts/` executables.
3. Verify contributor quickstart sequence (`clone`, `check`, `test`, `qemu`, `submit`).
4. Read latest `reports/capability/truth_table.json` and refresh "Verified Today" status documentation.
5. Flag documentation claims that promise unverified capabilities.
