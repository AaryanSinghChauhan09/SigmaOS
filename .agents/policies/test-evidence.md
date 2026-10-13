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
## 1. Rule of Grounded Evidence
No agent or contributor may update `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, `README.md`, or component documentation to claim a feature is "working", "implemented", "integrated", or "supported" without attaching verifiable test evidence.

## 2. Standard Evidence Types & Hierarchy
1. **Runtime & QEMU Evidence (Highest)**: Boot logs, serial output, installer execution transcripts, screenshots, or hardware test logs. Controls `Integrated`, `Hardened`, and `Supported` status levels.
2. **Integration Test Evidence**: Multi-component execution transcripts, cross-process test logs, and end-to-end synthetic harness outputs. Controls `Integrated` status level.
3. **Unit Test & Static Proofs**: Isolated crate/function tests, static analysis reports, and model proofs. Controls `Prototype` status level.

## 3. Required Evidence Record Format
Every verification run must output a JSON artifact in `reports/` containing:
- `timestamp`: UTC ISO-8601 timestamp.
- `commit_hash`: Exact Git commit SHA.
- `toolchain`: Rust/GCC/Clang compiler versions.
- `command`: Exact command line executed.
- `exit_code`: Numeric process exit code.
- `duration_ms`: Execution duration in milliseconds.
- `artifact_hashes`: SHA256 map of generated artifacts or logs.
- `log_excerpt`: Failure or success stdout/stderr digest.

## 4. Capability Status Definitions
- `Proposed`: Architectural decision record (ADR) or specification only.
- `Prototype`: Compiles and passes unit tests in isolated test harnesses; not integrated into boot path.
- `Integrated`: Connected to kernel/VFS/init boot path and verified in QEMU runtime execution.
- `Hardened`: Passes security denial tests, fuzzing, edge-case failure injection, and memory safety audits.
- `Supported`: Proven across multiple hardware/QEMU targets with reproducible installer and update/recovery mechanisms.

## 5. Automatic Downgrade Policy
Any status claim older than 90 days without revalidation evidence must be automatically flagged as `needs-revalidation` by the Capability Auditor.
