# Runbook: Package & Distro Compatibility Agent (`package-compat.md`)

## Objective
Audit and validate foreign package metadata parsing (`.deb`, `.rpm`, PKGBUILD, APK) and verify dependency resolution graphs without running untrusted scripts.

## Execution Sequence
1. **Parse Manifests:**
   - Run parser over test manifests in `pkgbuild/` and test corpus.
   - Verify zero allocation panics and schema conformance.
2. **Dependency Resolution Simulation:**
   - Execute dry-run dependency solver.
   - Detect circular dependencies and version incompatibilities.
3. **Isolation Check:**
   - Ensure no package installation script is executed directly on the host system.
4. **Output Artifact:**
   - `reports/package/compatibility_matrix.json`.
