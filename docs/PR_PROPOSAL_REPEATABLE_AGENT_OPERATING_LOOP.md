# Pull Request Proposal: Turn Repository into a Repeatable Agent Operating Loop

**Branch Name:** `feat/agent-operating-loop`
**Target Subsystem:** Distro Architecture, Tooling, Governance, Agent Automation
**Status:** Approved / Implementation Ready

---

## 1. Executive Summary

SigmaOS is an experimental, Rust-based "AI-native OS" repository with kernel, userspace, desktop, packaging, and tooling modules, plus dedicated agent documentation for each subsystem. Rather than continually adding speculative, unverified feature models, this proposal establishes a **repeatable agent operating loop** modeled directly after modern Linux distribution maintenance (Arch, Linux Mint, Debian, and Redox OS):
- Small, verifiable tasks with strict ownership boundaries.
- Evidence-gated capability status vocabulary (`unimplemented`, `model-only`, `integration-pending`, `qemu-verified`, `hardware-verified`, `blocked`).
- Distro-style release discipline (`dev`, `nightly`, `validated`, `experimental`).
- Automated tooling (`tools/agent_cli.py`, `tools/agent_loop_validator.py`, `tools/capability_audit.py`, `tools/boot_matrix.py`, `tools/evidence_signer.py`).
- Clear separation of duties across 7 distro-maintainer agent roles with least-privilege sandboxing.

---

## 2. Design Principles: Distro Maintainers vs. Monolithic Agent

Borrow the battle-tested operating model of Linux distributions:

| Distro Practice | SigmaOS Agent Equivalent |
|---|---|
| Package maintainer owns one package | One agent owns one subsystem or module boundary |
| ArchWiki-style reproducible procedure | Every task has a machine-readable runbook |
| Release branches and tagged versions | Agent changes land through PRs with test evidence |
| Stable / testing / unstable channels | `verified`, `experimental`, and `blocked` capability states |
| ISO build and smoke test | Boot / QEMU smoke test gates every integration change |
| Security advisories | Regression, capability, and security-impact reports |
| Reproducible builds | Deterministic build inputs, lockfiles, and recorded toolchain versions |

---

## 3. Seven Distro-Maintainer Agent Roles

1. **Repository Steward (`steward`)**
   - *Scope:* `.github/`, `docs/`, `.agents/`, `reports/steward/`.
   - *Responsibilities:* Check CI health, stale branches, dependency updates, and documentation drift. Verify conventional-commit formatting and task links.
   - *Outputs:* `reports/steward/daily-status.md`, `reports/steward/stale-work.md`.

2. **Build and Test Sentinel (`sentinel`)**
   - *Scope:* `src/`, `tests/`, `tools/`, `Makefile`, `run_sigma_tests.sh`, `reports/build/`, `reports/test/`.
   - *Responsibilities:* Run clean builds (`cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, `make format`). Detect flaky tests (up to 3 retries) and open minimal mechanical repair PRs only.
   - *Outputs:* `reports/build/latest.json`, `reports/test/summary.json`.

3. **Boot and Runtime Validator (`boot-validator`)**
   - *Scope:* `boot/`, `kernel/`, `installer/`, `iso_root/`, `scripts/`, `reports/boot/`.
   - *Responsibilities:* Build boot artifact in clean environment; boot QEMU x86_64; capture serial console logs; classify failure modes (`bootloader`, `kernel_panic`, `driver`, `init`, `shell`); maintain boot matrix.
   - *Outputs:* `reports/boot/qemu_x86_64.log`, `reports/boot/qemu_x86_64.json`, `reports/boot/boot_matrix.json`.

4. **Capability Auditor (`capability-auditor`)**
   - *Scope:* `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, `README.md`, `reports/capability/`.
   - *Responsibilities:* Cross-reference claims against source files and test evidence; prevent aspirational status; maintain public truth table and "Verified Today" dashboard.
   - *Outputs:* `reports/capability/truth_table.json`, `reports/capability/verified_today.md`.

5. **Kernel and Subsystem Agents (`kernel-subsystem`)**
   - *Scope:* `src/kernel/`, `src/fs/`, `src/vfs/`, `src/driver/`, `src/network/`.
   - *Responsibilities:* Subsystem-specific invariants (scheduler starvation, allocator fragmentation, VFS path traversal, syscall ABI consistency, driver probe); generate subsystem trace evidence.
   - *Outputs:* Subsystem unit tests and trace logs.

6. **Package and Distro Compatibility Agent (`package-compat`)**
   - *Scope:* `pkgbuild/`, `src/compatibility/`, `tools/sigma_pacman_compat.rs`, `reports/package/`.
   - *Responsibilities:* Maintain package manifest schemas (`.deb`, `.rpm`, PKGBUILD, APK); test parser against corpus; simulate dependency graphs; sandbox untrusted scriptlets.
   - *Outputs:* `reports/package/compatibility_matrix.json`.

7. **Documentation and Onboarding Agent (`docs-onboarding`)**
   - *Scope:* `README.md`, `docs/`, `CONTRIBUTING.md`, `reports/capability/verified_today.md`.
   - *Responsibilities:* Validate all README commands against Makefile/scripts; maintain contributor quickstarts; convert failure logs into troubleshooting runbooks.
   - *Outputs:* Updated developer documentation and verified quickstart guides.

---

## 4. Nine-Stage Repeatable Task Lifecycle

Every recurring agent task executes strictly through these 9 stages:
1. **Select Task:** Pick open task from `.agents/tasks/open/` based on priority and role.
2. **Read Scope & Invariants:** Review ownership paths, capability status, and runbooks.
3. **Isolate Workspace:** Work on an isolated branch with clean target directories.
4. **Implement Minimal Slice:** Apply the smallest viable change addressing the task.
5. **Scoped Verification:** Run subsystem-targeted tests and linters.
6. **Repository Verification:** Run `cargo check --lib`, unit tests, and integration scripts.
7. **Produce Evidence Artifact:** Generate machine-readable logs/JSON in `reports/`.
8. **Update Capability Status:** Upgrade/downgrade status in matrix only if supported by evidence.
9. **Submit PR with Rollback Note:** Submit pull request with diff, log excerpt, risk assessment, and rollback instructions.

---

## 5. Evidence-Gated Status Vocabulary

| Status Level | Definition | Required Evidence |
|---|---|---|
| `unimplemented` | No credible implementation found in tree | None |
| `model-only` | Unit-tested model exists, no live kernel path | Unit test results |
| `integration-pending` | Code exists but disconnected from boot path | Module compile check |
| `qemu-verified` | Passes automated QEMU boot and shell smoke tests | Serial console log & JSON report |
| `hardware-verified` | Tested on physical bare-metal hardware | Signed dmesg/serial hardware log |
| `blocked` | Known design flaw or dependency blocker | Blocker issue reference & failure log |

---

## 6. Least-Privilege Sandboxing Matrix

| Agent Class | Filesystem Scope | Network Access | Package Execution | Git Privilege |
|---|---|---|---|---|
| Documentation / Onboarding | Read repo, write `docs/` & `reports/` | Optional, allowlisted | No | PR only |
| Build & Test Sentinel | Read repo, write `reports/` & tmp | No | No | PR only (mechanical) |
| Boot & Runtime Validator | Read repo, write build artifacts | No unless needed | No | PR only |
| Capability Auditor | Read-only repo & reports | No | No | Issue / PR only |
| Package Compatibility | Sandboxed virtual workspace | No by default | Sandboxed dry-run only | PR only |
| Kernel & Subsystem Agents | Scoped subsystem directory only | No | No | PR only |

---

## 7. Ninety-Day Execution Plan

- **Days 1–14 (Establish Truth):** Create registry, task schema, report directories, capability audit truth table, and "Verified Today" dashboard. Freeze unverified claims.
- **Days 15–35 (Make Build Trustworthy):** Automate clean builds and test runs with failure classification. Flaky test retry logic. Mechanical PRs only.
- **Days 36–60 (Prove One Boot Path):** Focus on QEMU x86_64 boot-to-shell. Retain serial logs and maintain boot matrix. Identify runtime blockers.
- **Days 61–75 (Connect Subsystems to Runtime):** Connect core subsystems (memory, VFS, init, shell) to real boot path. Replace model claims with runtime traces.
- **Days 76–90 (Add Distro-Style Release Discipline):** Define `dev`, `nightly`, and `validated` channels. Publish only QEMU-verified nightlies. Package metadata compatibility audit.

---

## 8. Definition of "Task Done" Checklist (10 Points)

1. [x] Zero-dependency Rust compilation verified (`cargo check --lib`).
2. [x] Subsystem unit tests pass cleanly (`cargo test --lib`).
3. [x] Master test script executes without regressions (`./run_sigma_tests.sh`).
4. [x] Standard Makefile targets verified (`make check`, `make test`, `make format`).
5. [x] Structured evidence JSON/log stored in `reports/`.
6. [x] QEMU x86_64 boot simulation passes without kernel panic.
7. [x] Capability matrix status matches verifiable empirical evidence.
8. [x] Runbooks and policies updated in `.agents/`.
9. [x] Conventional commit format with linked task identifier.
10. [x] Rollback procedure documented in PR description.
