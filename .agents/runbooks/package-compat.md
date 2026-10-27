# Runbook: Package & Distro Compatibility

## Agent Role
Package and Distro Compatibility Agent (`package-compat`)

## Workflow Steps
1. Load sample package metadata manifests (`.deb`, `.rpm`, Arch PKGBUILD, BSD pkg).
2. Validate metadata parsing and dependency tree construction in isolated sandbox.
3. Perform dry-run package dependency resolution simulation.
4. Record parsing and dependency resolution status in `reports/package/compatibility.json`.
5. Do NOT execute untrusted package scriptlets outside isolated sandbox environment.
