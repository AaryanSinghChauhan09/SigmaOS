# Runbook: Package & Distro Compatibility Agent Execution (.agents/runbooks/package-compat.md)

1. Load test corpus of `.deb`, `.rpm`, Arch, and BSD package manifests.
2. Validate metadata parsing, version comparisons, and dependency tree generation.
3. Perform dry-run package installations in isolated sandbox container.
4. Export matrix to `reports/package/compatibility_matrix.json`.
