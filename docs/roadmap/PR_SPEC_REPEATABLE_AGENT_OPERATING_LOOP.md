# PR Specification: Turn Repository into Repeatable Agent Operating Loop

## Metadata
- **PR Title:** feat(distro): Turn Repository into Repeatable Agent Operating Loop
- **PR Branch:** `feat/agent-operating-loop`
- **Engine Module:** `src/distro/repeatable_agent_operating_loop_pr.rs`
- **Integration Test:** `tests/test_repeatable_agent_operating_loop.rs`
- **Status:** Approved / Implementation Ready

## Subsystem Contract & Architecture
This specification formalizes the transition of SigmaOS into an evidence-gated distro-maintainer operating model:
- 7 Agent Roles (`steward`, `sentinel`, `boot-validator`, `capability-auditor`, `kernel-subsystem`, `package-compat`, `docs-onboarding`).
- 9-Stage Repeatable Task Lifecycle (`Select`, `Read Scope`, `Isolate Workspace`, `Implement Slice`, `Scoped Test`, `Repo Checks`, `Evidence Artifact`, `Update Status`, `Submit PR`).
- 6 Evidence-Gated Status Levels (`unimplemented`, `model-only`, `integration-pending`, `qemu-verified`, `hardware-verified`, `blocked`).
- Least-Privilege Sandboxing Matrix.
- 90-Day Execution Roadmap.
- 10-Point "Definition of Done" Verification.

## Verification Gate
```bash
cargo check --lib
python3 tools/agent_loop_validator.py
python3 tools/capability_audit.py
python3 tools/boot_matrix.py
python3 tools/agent_cli.py --list-open
```
