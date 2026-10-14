# PR Specification: SigmaOS Repeatable Agent Operating Loop

## Specification Metadata
- **PR Title:** `feat(distro): Turn Repository into Repeatable Agent Operating Loop`
- **PR Branch:** `feat/agent-operating-loop`
- **Status:** Approved / Implementation Ready
- **Implementation Engine:** `src/distro/repeatable_agent_operating_loop_pr.rs`
- **Test Suite:** `tests/test_repeatable_agent_operating_loop.rs`

---

## 1. Distro Maintainer Agent Roles

1. `steward`: **Repository Steward** — Repo hygiene, issue/PR triage, conventional commits, staleness audits (`.github/`).
2. `sentinel`: **Build and Test Sentinel** — Automated clean builds, test execution, flaky test detection, test manifests (`reports/test/`).
3. `boot-validator`: **Boot and Runtime Validator** — QEMU boot-to-shell validation, serial console log capture, boot matrix upkeep (`reports/boot/`).
4. `capability-auditor`: **Capability Auditor** — Evidence-backed truth table cross-referencing status claims (`reports/capability/`).
5. `kernel-subsystem`: **Kernel and Subsystem Agent** — Kernel entry, memory management, VFS, IPC, interrupts, syscall ABI (`src/kernel/`).
6. `package-compat`: **Package and Distro Compatibility Agent** — Package metadata schemas, dependency resolution, sandboxed dry-runs (`src/package/`).
7. `docs-onboarding`: **Documentation and Onboarding Agent** — Command synchronization between README/Makefile/scripts and Verified Today dashboard (`docs/`).

---

## 2. 9-Stage Repeatable Task Loop

1. **Stage 1: Select Task** — Read task specification, runbook, ownership rules, and capability matrix.
2. **Stage 2: Read Ownership & Matrix** — Verify subsystem ownership boundaries and current status claims.
3. **Stage 3: Isolate Branch** — Create dedicated branch/workspace (`feat/`, `fix/`, `test/`, `docs/`, `chore/`).
4. **Stage 4: Implement Minimal Change** — Apply smallest possible verifiable code or documentation change.
5. **Stage 5: Run Scoped Tests** — Execute scoped test suite for modified module.
6. **Stage 6: Run Repository-Wide Checks** — Execute `./run_sigma_tests.sh` and `python3 tools/agent_cli.py --run-checks`.
7. **Stage 7: Produce Evidence Artifact** — Write structured execution report to `reports/<agent>/`.
8. **Stage 8: Update Status With Evidence** — Update capability/status matrices only if supported by passing test evidence.
9. **Stage 9: Submit PR With Rollback Note** — Open pull request with diff, test output, risk assessment, and rollback instructions.

---

## 3. Evidence Status Vocabulary

- `unimplemented`: No credible implementation found in codebase.
- `model-only`: Unit-tested model exists, but no connected runtime path.
- `integration-pending`: Code exists but is not connected end-to-end to kernel boot/runtime.
- `qemu-verified`: Passes automated QEMU boot and runtime execution tests.
- `hardware-verified`: Tested on named physical hardware with serial/kernel logs on record.
- `blocked`: Known dependency, design, or validation blocker prevents progress.

---

## 4. Verification & Validation Commands

Every agent PR must pass:
```bash
python3 tools/agent_cli.py --run-checks
./run_sigma_tests.sh
python3 tools/capability_audit.py
python3 tools/boot_matrix.py
python3 tools/evidence_signer.py
```
