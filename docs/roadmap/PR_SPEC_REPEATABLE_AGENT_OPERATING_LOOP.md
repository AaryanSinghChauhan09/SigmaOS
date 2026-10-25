# PR Spec: Turn SigmaOS into a Repeatable Agent Operating Loop

**PR Title**: `feat(distro): Turn Repository into Repeatable Agent Operating Loop`
**Specification ID**: `PR-SPEC-REPEATABLE-AGENT-OPERATING-LOOP-001`
**Module Owner**: Distro Maintainer & Agent Operating Engine
**Engine Reference**: `src/distro/repeatable_agent_operating_loop_pr.rs`
**Test Reference**: `tests/test_repeatable_agent_operating_loop.rs`

---

## 1. Specification Overview

This document specifies the operational contract and verification rules for transforming SigmaOS into a **repeatable agent operating loop**. It bridges high-level OS planning with automated, evidence-backed software engineering procedures.

---

## 2. Agent Roles and Scopes

```text
+-----------------------------------------------------------------------------------+
|                            REPOSITORY STEWARD                                     |
|    (.github/, reports/steward/ - Hygiene, Conventional Commits, Staleness)       |
+-----------------------------------------------------------------------------------+
       |                    |                    |                   |
       v                    v                    v                   v
+--------------+   +-----------------+   +---------------+   +-------------------+
| BUILD/TEST   |   | BOOT/RUNTIME    |   | CAPABILITY    |   | KERNEL & SUBSYSTEM|
| SENTINEL     |   | VALIDATOR       |   | AUDITOR       |   | AGENTS            |
| (reports/    |   | (reports/boot/  |   | (reports/     |   | (src/kernel/,     |
| test/)       |   | QEMU boot)      |   | capability/)  |   | src/fs/, drivers/)|
+--------------+   +-----------------+   +---------------+   +-------------------+
                                                                     |
                                                                     v
                                                             +-------------------+
                                                             | PACKAGE & COMPAT  |
                                                             | (src/package/,    |
                                                             | sandboxed dry-run)|
                                                             +-------------------+
```

1. **Repository Steward** (`steward`):
   - Scope: `.github/`, `reports/steward/`
   - Actions: Triage stale work, enforce conventional commit format, check issue links.
2. **Build and Test Sentinel** (`sentinel`):
   - Scope: `reports/test/`, build targets
   - Actions: Continuous test runs (`cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, `make format`), flaky test detection (up to 3 retries).
3. **Boot and Runtime Validator** (`boot-validator`):
   - Scope: `reports/boot/`, `installer/`, `scripts/`
   - Actions: QEMU x86_64 boot-to-shell verification, serial log capture, screenshot generation, boot duration metrics.
4. **Capability Auditor** (`capability-auditor`):
   - Scope: `reports/capability/`, `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`
   - Actions: Audit documentation claims vs runtime/test evidence. Publish public truth table (`truth_table.json`).
5. **Kernel and Subsystem Agents** (`kernel-subsystem`):
   - Scope: `src/kernel/`, `src/fs/`, `src/vfs/`, `src/drivers/`, `src/network/`
   - Actions: Scheduler, Memory, VFS, Syscall, IPC, Driver, and Security subsystem maintenance.
6. **Package and Distro Compatibility Agent** (`package-compat`):
   - Scope: `src/package/`, `src/compatibility/`
   - Actions: Sandboxed package manifest parsing, dependency resolution, dry-run package installations.
7. **Documentation and Onboarding Agent** (`docs-onboarding`):
   - Scope: `docs/`, `README.md`, `reports/`
   - Actions: README/Makefile sync, "Verified Today" page updates.

---

## 3. 9-Stage Repeatable Lifecycle Verification

Each agent operation must execute the following 9 steps:
1. `SELECT TASK`: Read task declaration from `.agents/tasks/open/`.
2. `READ OWNERSHIP & CAPABILITY MATRIX`: Check allowed scope in `.agents/registry.toml` and current feature status.
3. `ISOLATE BRANCH`: Create dedicated git branch `feat/<agent>-<task_id>`.
4. `IMPLEMENT MINIMAL CHANGE`: Scope code modification strictly to task bounds.
5. `RUN SCOPED TESTS`: Execute module-level tests.
6. `RUN REPO-WIDE CHECKS`: Execute `cargo check --lib`, `cargo test --lib`, and `./run_sigma_tests.sh`.
7. `PRODUCE EVIDENCE ARTIFACT`: Write JSON/Markdown log artifacts in `reports/`.
8. `UPDATE STATUS WITH EVIDENCE`: Adjust capability level in status files only when evidence is generated.
9. `SUBMIT PR WITH ROLLBACK`: Submit PR with conventional commit, diff summary, risk assessment, and rollback instructions.

---

## 4. Evidence Status Level Matrix

| Level | Name | Rule for Promotion |
|---|---|---|
| Level 0 | `unimplemented` | Default status for unwritten code. |
| Level 1 | `model-only` | Unit tests pass for standalone struct/function. |
| Level 2 | `integration-pending` | Code compiled into system binary but unverified at boot. |
| Level 3 | `qemu-verified` | Passes automated QEMU boot and execution test with serial log evidence. |
| Level 4 | `hardware-verified` | Tested on physical hardware with recorded kernel logs. |
| Blocked | `blocked` | Progression halted by dependency or design blocker. |

---

## 5. Definition of Done Checklist

A task is marked `Done` if and only if all 10 evidence points are satisfied:
- [x] Machine-readable task file exists in `.agents/tasks/`.
- [x] Isolated branch created and clean working tree confirmed.
- [x] Scoped unit tests written and passing.
- [x] System-wide build (`cargo check --lib`) exits 0.
- [x] Full test suite (`./run_sigma_tests.sh`) exits 0.
- [x] Evidence log artifact generated in `reports/`.
- [x] Capability status updated in `CAPABILITY_MATRIX.toml` / `FEATURE_STATUS.toml`.
- [x] Documentation synchronized with code changes.
- [x] PR proposal formatted with Conventional Commits and rollback notes.
- [x] Rust engine verification (`src/distro/repeatable_agent_operating_loop_pr.rs`) succeeds.
