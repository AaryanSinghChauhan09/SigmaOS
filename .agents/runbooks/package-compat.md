# Runbook: Package and Distro Compatibility Agent (`package-compat`)

## Purpose
Safely evaluate Linux and BSD package metadata parsing, dependency graph resolution, and sandboxed package execution without exposing the host system.

## Procedure
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
