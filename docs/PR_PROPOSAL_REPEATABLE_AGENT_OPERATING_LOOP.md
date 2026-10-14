# PR Proposal: SigmaOS Repeatable Agent Operating Loop

## Executive Summary

SigmaOS is an experimental, Rust-based "AI-native OS" repository modeling Linux, Arch Linux, Linux Mint, Redox OS, xv6, FreeBSD/OpenBSD patterns, and OSDev projects. The most valuable operational step for SigmaOS is to turn the repository into a **repeatable agent operating loop**: small, verifiable tasks; evidence-gated status; and distro-style release discipline.

---

## Design Principle: Treat Agents Like Distro Maintainers

Borrow the operating model of Linux distributions rather than attempting to have one agent build the entire operating system:

| Distro practice | SigmaOS agent equivalent |
|---|---|
| Package maintainer owns one package | One agent owns one subsystem or module boundary |
| ArchWiki-style reproducible procedure | Every task has a machine-readable runbook |
| Release branches and tagged versions | Agent changes land through PRs with test evidence |
| Stable/testing/unstable channels | `verified`, `experimental`, and `blocked` capability states |
| ISO build and smoke test | Boot/QEMU smoke test gates every integration change |
| Security advisories | Regression, capability, and security-impact reports |
| Reproducible builds | Deterministic build inputs, lockfiles, and recorded toolchain versions |

---

## Recommended Agent Organization

### 1. Repository Steward
- **Recurring Tasks:** Daily CI checks, stale branch auditing, conventional commit format verification, weekly staleness audits.
- **Outputs:** `reports/steward/daily-status.md`, `reports/steward/stale-work.md`.
- **Definition of Done:** All open PRs checked for commit rules, linked issue, changed-file scope, and test evidence.

### 2. Build and Test Sentinel
- **Recurring Tasks:** Runs `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, `make format`. Detects flaky tests and generates test manifests.
- **Outputs:** `reports/build/latest.json`, `reports/test/latest.json`.
- **Acceptance Rule:** No agent may mark a subsystem "working" unless the test command passes on a clean checkout and the result is recorded.

### 3. Boot and Runtime Validator
- **Recurring Tasks:** Builds ISO/boot artifacts, launches QEMU x86_64/ARM64, captures serial console logs and boot durations, maintains boot matrix.
- **Outputs:** `reports/boot/qemu-x86_64.log`, `reports/boot/qemu-x86_64.json`, `reports/boot/boot_matrix.json`.
- **Recurring Gate:** Clean build -> QEMU boot -> Serial log captured -> Shell prompt reached -> Smoke command executes -> Graceful shutdown -> Artifact hash recorded.

### 4. Capability Auditor
- **Recurring Tasks:** Audits `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, README claims, docs, and source code to maintain an evidence-driven truth table.
- **Outputs:** `reports/capability/truth_table.json`, `reports/capability/verified_today.md`.
- **Status Vocabulary:** `unimplemented`, `model-only`, `integration-pending`, `qemu-verified`, `hardware-verified`, `blocked`.

### 5. Kernel and Subsystem Agents
- **Recurring Tasks:** Ownership boundaries over Scheduler, Memory, VFS/Filesystem, Syscall, IPC, Driver, and Security subsystems.
- **Evidence Required:** Unit/integration test outputs, scheduler traces, allocation metrics, filesystem conformance reports, threat models.

### 6. Package and Distro Compatibility Agent
- **Recurring Tasks:** Maintains package-format schemas, parses `.deb`, `.rpm`, Arch, and BSD manifests, resolves dependency graphs, runs sandboxed dry-runs.
- **Outputs:** `reports/package/compatibility_matrix.json`.
- **Guardrail:** Never execute untrusted package scripts directly; execute only in isolated sandboxes with declared filesystem, network, and process capabilities.

### 7. Documentation and Onboarding Agent
- **Recurring Tasks:** Ensures public commands match scripts/Makefile, generates "what actually works today" page, converts repeated agent failures into troubleshooting entries.
- **Outputs:** `reports/capability/verified_today.md`.

---

## The 9-Stage Repeatable Task Loop

Every agent task follows a strict 9-stage lifecycle:

```text
SELECT TASK
  1. SELECT: Read task specification, runbook, ownership rules, and capability matrix
  2. PLAN: Formulate minimal change and identify scoped verification commands
  3. ISOLATE: Create dedicated branch/workspace (`feat/`, `fix/`, `test/`, `docs/`, `chore/`)
  4. EXECUTE: Implement the smallest possible change
  5. VERIFY SCOPED: Run scoped tests for modified subsystem
  6. VERIFY REPO: Run repository-wide verification (`./run_sigma_tests.sh`, `python3 tools/agent_cli.py --run-checks`)
  7. RECORD EVIDENCE: Write structured report to `reports/<agent>/`
  8. UPDATE STATUS: Update capability/status matrices only if supported by evidence
  9. SUBMIT PR: Open pull request with diff, test output, risk assessment, and rollback note
```

---

## Safety and Permission Sandbox Model

| Agent Class | Filesystem Permissions | Network Permissions | Package Execution | Git Write |
|---|---|---|---|---|
| Documentation | Read repo, write `docs/` and `reports/` | Optional, allowlisted | No | PR only |
| Test Sentinel | Read repo, write `reports/` and temp dirs | No | No | PR only |
| Boot Validator | Read repo, write `reports/boot/` | No | No | PR only |
| Capability Auditor | Read-only plus `reports/capability/` | No | No | Issue / PR only |
| Package Compat | Sandbox only | No default | Sandboxed dry-run first | PR only |
| Kernel Subsystem | Scoped source paths | No | No | PR only |
| Repository Steward | Read repo, write `reports/steward/` | No | No | PR only |

---

## 90-Day Execution Plan

1. **Days 1–14 (Establish Truth):** Agent registry, task schema, report directories, CI gates, capability audit, "Verified Today" dashboard, freeze unverified claims.
2. **Days 15–35 (Make Build Trustworthy):** Automate clean builds and test runs, failure classification, flaky-test detection, machine-readable verification reports on every PR.
3. **Days 36–60 (Prove One Boot Path):** Focus exclusively on QEMU x86_64 boot-to-shell, maintain boot matrix, retain serial logs, assign subsystem agents to hard blockers.
4. **Days 61–75 (Connect Subsystems to Runtime):** Connect VFS, init, shell, memory management, and process lifecycle to actual boot path. Replace "model passes test" claims with runtime trace evidence.
5. **Days 76–90 (Distro Release Discipline):** Define `dev`, `nightly`, and `validated` release channels. Publish only QEMU-verified nightly artifacts. Implement rollback safety and package metadata compatibility audits.

---

## Highest-Priority Automation Tooling

1. `tools/agent_cli.py`: Task runner executing steps, validating success criteria, and generating evidence JSON reports.
2. `tools/capability_audit.py`: Generates `truth_table.json` and `verified_today.md`.
3. `tools/boot_matrix.py`: Tracks QEMU boot targets, captures logs, and updates `boot_matrix.json`.
4. `tools/evidence_signer.py`: Generates SHA256 cryptographic verification manifests in `reports/evidence_signatures.json`.
