# PR Proposal: Turn SigmaOS into a Repeatable Agent Operating Loop

**PR Title**: `feat(distro): Turn Repository into Repeatable Agent Operating Loop`
**Status**: `Approved / Implementation Ready`
**Target Branch**: `main`
**Source Branch**: `feat/agent-operating-loop`

---

## Executive Summary

SigmaOS is an experimental, Rust-based "AI-native OS" repository with kernel, userspace, desktop, packaging, and tooling modules, supported by dedicated agent documentation. To transform SigmaOS from broad planning ambitions into a verifiably integrated operating system, this proposal establishes a **repeatable agent operating loop**:
1. Small, verifiable tasks with machine-readable task declarations.
2. Evidence-gated capability status linked to clean test runs and boot traces.
3. Distro-style release discipline modeling agents as specialized Linux/BSD distro maintainers.

Rather than expecting a single agent to "build the whole OS," SigmaOS establishes a hierarchy of narrowly scoped agents, explicit least-privilege permission sandboxes, evidence-backed capabilities, and a 90-day execution plan.

---

## Design Principle: Treat Agents Like Distro Maintainers

| Distro Practice | SigmaOS Agent Equivalent |
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

### 1. Repository Steward (`steward`)
- **Owner Scope**: `.github/`, `reports/steward/`
- **Daily Tasks**: Check for failing CI, stale branches, unmerged dependency updates, and documentation drift.
- **PR Checks**: Verify conventional commit format, linked task ID, changed-file scope, and test evidence.
- **Weekly Tasks**: Audit files that claim completion but lack test or runtime evidence.
- **Outputs**: `reports/steward/daily-status.md`, `reports/steward/stale-work.md`.

### 2. Build and Test Sentinel (`sentinel`)
- **Owner Scope**: `reports/test/`, build scripts, CI workflows
- **Tasks**: Execute `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, and `make format`.
- **Flaky Test Detection**: Rerun failed tests up to 3 times and classify failures.
- **Outputs**: Machine-readable test manifest (`reports/test/manifest.json`). Open minimal repair PRs for mechanical fixes only.

### 3. Boot and Runtime Validator (`boot-validator`)
- **Owner Scope**: `reports/boot/`, `installer/`, `scripts/`
- **Tasks**: Clean build ISO/boot artifacts, launch in QEMU (x86_64 primary target, ARM64 secondary), capture serial console logs, screenshots, and boot metrics.
- **Minimum Recurring Test**: Clean build -> QEMU boot -> Serial log captured -> Shell prompt reached -> Execute smoke commands -> Graceful shutdown -> Record hash & QEMU version.

### 4. Capability Auditor (`capability-auditor`)
- **Owner Scope**: `reports/capability/`, `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`
- **Tasks**: Cross-reference documentation claims against passing tests and boot traces. Flag unverified claims and maintain a public truth table (`reports/capability/truth_table.json`).

### 5. Kernel and Subsystem Agents (`kernel-subsystem`)
- **Owner Scope**: `src/kernel/`, `src/fs/`, `src/vfs/`, `src/drivers/`, `src/network/`
- **Agents**: Scheduler Agent, Memory Agent, VFS/Filesystem Agent, Syscall Agent, IPC Agent, Driver Agent, Security Agent.
- **Rule**: Require integration review before altering another subsystem's public interface.

### 6. Package and Distro Compatibility Agent (`package-compat`)
- **Owner Scope**: `src/package/`, `src/compatibility/`
- **Tasks**: Package format schema validation, metadata parsing against sample `.deb`, `.rpm`, Arch, and BSD manifests, dependency resolution tests.
- **Guardrail**: Never execute untrusted package scripts directly; execute only in sandboxes with declared filesystem and process capabilities.

### 7. Documentation and Onboarding Agent (`docs-onboarding`)
- **Owner Scope**: `docs/`, `README.md`, `reports/`
- **Tasks**: Ensure all public commands match Makefile/scripts. Generate a "Verified Today" page (`reports/capability/verified_today.md`). Convert repeated agent failures into troubleshooting entries.

---

## 9-Stage Repeatable Task Lifecycle

```text
SELECT TASK (1)
  → Read Ownership & Capability Matrix (2)
  → Isolate Branch (3)
  → Implement Minimal Change (4)
  → Run Scoped Tests (5)
  → Run Repo-Wide Checks (6)
  → Produce Evidence Artifact (7)
  → Update Status With Evidence (8)
  → Submit PR With Rollback Note (9)
```

A task is complete only when its code, tests, documentation, and capability status agree.

---

## Evidence Status Vocabulary

| Status Level | Definition |
|---|---|
| `unimplemented` | No credible implementation found in codebase. |
| `model-only` | Unit-tested model exists, but no connected runtime path. |
| `integration-pending` | Code exists but is not connected end-to-end to kernel boot/runtime. |
| `qemu-verified` | Passes automated QEMU boot and runtime execution tests. |
| `hardware-verified` | Tested on named physical hardware with serial/kernel logs on record. |
| `blocked` | Known dependency, design, or validation blocker prevents progress. |

---

## Safety and Permission Sandbox Matrix

| Agent Class | Filesystem | Network | Package Execution | Git Write |
|---|---|---|---|---|
| Documentation | Read repo, write `docs/` & `reports/` | Optional, allowlisted | No | PR only |
| Test Runner | Read repo, write `reports/` & temp dirs | No | No | PR only |
| Boot Validator | Read repo, write `reports/boot/` | No | No | PR only |
| Capability Auditor | Read-only plus `reports/` | No | No | Issue / PR only |
| Package Compatibility | Sandbox only | No default | Sandboxed dry-run first | PR only |
| Kernel/Subsystem Agent | Scoped source paths | No | No | PR only |

---

## 90-Day Execution Plan

1. **Days 1–14 (Establish Truth)**: Create agent registry, task schema, report directories, and CI verification jobs. Audit README, `FEATURE_STATUS.toml`, and `CAPABILITY_MATRIX.toml`. Publish "Verified Today" dashboard and freeze unverified claims.
2. **Days 15–35 (Make Build Trustworthy)**: Automate clean builds and test runs with failure classification. Require machine-readable test reports on every PR.
3. **Days 36–60 (Prove One Boot Path)**: Focus on QEMU x86_64 boot-to-shell. Retain serial console logs and assign subsystem agents to hard runtime blockers.
4. **Days 61–75 (Connect Subsystems to Runtime)**: Connect minimum set of subsystems (memory, VFS, init, shell) to real boot path. Replace "model passes test" claims with runtime traces.
5. **Days 76–90 (Add Distro Release Discipline)**: Establish `dev`, `nightly`, and `validated` channels. Publish QEMU-verified nightly artifacts with rollback notes.

---

## Machine-Readable Task Format (TOML)

```toml
id = "BOOT-X86-001"
title = "Validate QEMU x86_64 boot-to-shell"
agent = "boot-runtime-validator"
priority = "P0"
scope = ["installer/", "src/kernel/", "scripts/", "tests/"]
repeat = "daily"

steps = [
  "make clean",
  "make check",
  "make test",
  "build ISO artifact",
  "boot QEMU with serial logging",
  "wait for shell prompt",
  "execute smoke commands",
  "capture logs and screenshot",
  "write boot report"
]

evidence = [
  "reports/boot/qemu-x86_64.log",
  "reports/boot/qemu-x86_64.json",
  "reports/boot/screenshot.png"
]

success = [
  "build exits 0",
  "QEMU reaches shell within timeout",
  "smoke commands exit 0",
  "no kernel panic in serial log"
]

on_failure = "create blocked task with first failing stage and log excerpt"
```

---

## 7 Highest-Priority First Tasks

1. **Create `tools/agent_cli.py`**: Command-line tool to run named tasks, execute checks, and produce structured evidence reports.
2. **Create `reports/boot/`**: Establish QEMU x86_64 boot-to-shell validation artifacts and logger.
3. **Audit "Working" Claims**: Audit claims in `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml` against test evidence.
4. **Add CI Gate**: Require `cargo check --lib`, `cargo test --lib`, and `./run_sigma_tests.sh` on every PR.
5. **Create "Verified Today" Page**: Generate `reports/capability/verified_today.md` from test and boot results.
6. **Package Compatibility Milestone**: Implement sandboxed metadata parsing and dry-run dependency validation for `.deb`, `.rpm`, and Arch packages.
7. **Convert Strategy Specs to TOML Tasks**: Populate `.agents/tasks/open/` with executable TOML task files.
