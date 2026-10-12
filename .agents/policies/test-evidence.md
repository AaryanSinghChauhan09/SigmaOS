# Test Evidence Policy

## Evidence Standards

1. **Objective Proof Required**: A component claim (`qemu-verified`, `hardware-verified`) requires machine-readable evidence saved under `reports/`.
2. **Standard Evidence Artifacts**:
   - Build/Test: `reports/test/build_manifest.json`
   - Boot: `reports/boot/qemu_x86_64.log` and `reports/boot/boot_report.json`
   - Capability Audit: `reports/capability/truth_table.json` and `reports/capability/verified_today.md`
   - Package Dry-Run: `reports/package/dry_run_report.json`
3. **Flaky Test Handling**: Tests that fail must be re-executed up to 3 times to distinguish genuine regressions from environmental flakiness before flagging as broken.
4. **Attestation & Hash Verification**: Every report bundle must include a SHA256 evidence manifest signed/timestamped by `tools/evidence_signer.py`.
