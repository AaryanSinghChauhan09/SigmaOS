# Runbook: Capability Auditor (`capability-audit.md`)

## Objective
Prevent aspirational claims in documentation by cross-referencing `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml` against code references and test logs.

## Execution Sequence
1. **Execute Audit Tool:**
   ```bash
   python3 tools/capability_audit.py
   ```
2. **Validate Results:**
   - Confirm all components with claimed status have backing source code.
   - Ensure components without runtime evidence are downgraded to `model-only` or `integration-pending`.
   - Ensure components verified in QEMU are marked `qemu-verified`.
3. **Output Artifacts:**
   - `reports/capability/truth_table.json`: Machine-readable audit table.
   - `reports/capability/verified_today.md`: Markdown dashboard of verified capabilities.
