# Runbook: Package Compatibility Audit

## Purpose
Safely parse package metadata (.deb, .rpm, Arch PKGBUILD/ALPM, sigpkg) and perform dry-run dependency graph resolution without executing arbitrary package scripts.

## Procedure
1. Load sample package manifests from test corpus in `src/package/` and `src/sigpkg/`.
2. Validate metadata parsing against schema definitions.
3. Construct package dependency DAG and check for circular or missing dependencies.
4. Perform dry-run installation simulation capturing target file locations.
5. Record parsing accuracy, resolution time, and safety review flags in `reports/package/dry_run_report.json`.
