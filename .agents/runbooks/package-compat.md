# Runbook: Package Compatibility Audit

## Purpose
Safely parse package metadata (.deb, .rpm, Arch PKGBUILD/ALPM, sigpkg) and perform dry-run dependency graph resolution without executing arbitrary package scripts.

## Procedure
1. Load sample package manifests from test corpus in `src/package/` and `src/sigpkg/`.
2. Validate metadata parsing against schema definitions.
3. Construct package dependency DAG and check for circular or missing dependencies.
4. Perform dry-run installation simulation capturing target file locations.
5. Record parsing accuracy, resolution time, and safety review flags in `reports/package/dry_run_report.json`.
# Runbook: Package and Distro Compatibility Agent (`package-compat`)
Safely evaluate Linux and BSD package metadata parsing, dependency graph resolution, and sandboxed package execution without exposing the host system.
1. Fixed Package Corpus Parsing:
   - Parse sample metadata from `.deb`, `.rpm`, Arch `PKGBUILD`/`desc`, and FreeBSD `pkg-plist` manifests.
   - Verify zero-panic parsing against malformed inputs.
2. Dependency Resolution Validation:
   - Construct directed acyclic graph (DAG) of package dependencies.
   - Test resolution algorithms against missing, circular, or conflicting dependency trees.
3. Sandboxed Dry-Run Execution:
   - Execute package scriptlets inside a sandboxed container/chroot with restricted VFS (`unveil`) and pledge promises.
   - Log all attempted syscalls and VFS modifications.
4. Report Generation:
   - Output results to `reports/package/compatibility_matrix.json`.
