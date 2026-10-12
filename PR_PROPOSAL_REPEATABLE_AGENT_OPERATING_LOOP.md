# Pull Request Proposal: Operationalizing SigmaOS as a Repeatable Agent Operating Loop

**PR Title:** `feat(governance): Operationalize Repeatable Agent Operating Loop, Hierarchy, and Evidence-Gated Release Discipline`
**PR Branch:** `feature/repeatable-agent-operating-loop`
**Target Branch:** `main`
**Status:** Proposal / Infrastructure Ready
**Component Scope:** Governance, Agent System, CI/CD, Capability Auditor, Boot Validator, Build Sentinel, Packaging, Documentation

---

## 1. Executive Summary & Problem Statement

SigmaOS is an experimental, Rust-based "AI-native OS" repository with rich kernel, userspace, desktop, packaging, and distro-compatibility models. However, as noted in the README and capability audits, unit tests and source presence do not prove a complete bootable or installable operating system.

The most valuable next step for SigmaOS is not adding more ambitious features, but turning the repository into a **repeatable agent operating loop**:
- **Small, verifiable tasks**: Machine-readable TOML task definitions with clear success criteria.
- **Evidence-gated status**: Capability claims backed by QEMU boot traces, test manifests, and signed evidence reports.
- **Distro-style release discipline**: Borrowing Linux distribution practices where narrow agents act as subsystem maintainers.

This proposal establishes the agent hierarchy, safety/least-privilege permissions, repeated execution loop, machine-readable schemas, 90-day execution plan, and operational CLI tooling.

---

## 2. Design Principle: Distro Maintainer Operating Model

Rather than relying on a single agent trying to build the entire operating system, SigmaOS adopts the organizational model of Linux distributions:

| Distro Practice | SigmaOS Agent Equivalent |
|---|---|
| Package maintainer owns one package | One agent owns one subsystem or module boundary |
| ArchWiki-style reproducible procedure | Every task has a machine-readable runbook in `.agents/runbooks/` |
| Release branches and tagged versions | Agent changes land through PRs with test evidence |
| Stable/testing/unstable channels | `verified`, `experimental`, and `blocked` capability states |
| ISO build and smoke test | Boot/QEMU smoke test gates every integration change |
| Security advisories | Regression, capability, and security-impact reports in `reports/` |
| Reproducible builds | Deterministic build inputs, lockfiles, and recorded toolchain versions |

---

## 3. Agent Hierarchy and Responsibilities

```text
                        ┌────────────────────────┐
                        │   Repository Steward   │
                        │  (Triage, Scope, PRs)  │
                        └───────────┬────────────┘
                                    │
       ┌────────────────────────────┼────────────────────────────┐
       ▼                            ▼                            ▼
┌──────────────┐            ┌──────────────┐            ┌──────────────┐
│  Build/Test  │            │ Boot/Runtime │            │  Capability  │
│   Sentinel   │            │  Validator   │            │   Auditor    │
└──────────────┘            └──────────────┘            └──────────────┘
       │                            │                            │
       ▼                            ▼                            ▼
┌──────────────┐            ┌──────────────┐            ┌──────────────┐
│  Subsystem   │            │Package/Distro│            │ Documentation│
│    Agents    │            │ Compat Agent │            │  & Onboarding│
└──────────────┘            └──────────────┘            └──────────────┘
```

### 1. Repository Steward
- **Recurring tasks**: Daily checks for failing CI, stale branches, unmerged dependencies, and documentation drift. Weekly audit of files claiming completion without runtime evidence.
- **Outputs**: `reports/steward/daily-status.md`, `reports/steward/stale-work.md`, PR review checklists.

### 2. Build and Test Sentinel
- **Recurring tasks**: Runs `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and `make test`. Detects flaky tests via rerun loops.
- **Outputs**: `reports/test/test_manifest.json`, repair PRs for mechanical formatting or import errors.

### 3. Boot and Runtime Validator
- **Recurring tasks**: Builds ISO/boot artifacts, launches QEMU x86_64/ARM64, captures serial console output, verifies boot milestones (firmware → kernel → VFS → init → shell).
- **Outputs**: `reports/boot/qemu_x86_64.log`, `reports/boot/qemu_x86_64.json`.

### 4. Capability Auditor
- **Recurring tasks**: Compares `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, README claims, and source files. Flags unverified claims.
- **Outputs**: `reports/capability/truth_table.json`, `reports/capability/verified_today.md`.

### 5. Subsystem Agents (Kernel, Memory, VFS, Syscall, IPC, Driver, Security)
- **Recurring tasks**: Narrow ownership of subsystem boundaries. Enforces component invariants, fuzzes allocators, validates path traversal, and manages syscall ABI consistency.
- **Outputs**: Unit tests, integration tests, subsystem evidence logs.

### 6. Package and Distro Compatibility Agent
- **Recurring tasks**: Parses metadata for `.deb`, `.rpm`, Arch, and BSD packages. Constructs dependency graphs and executes sandboxed dry-run installations.
- **Outputs**: `reports/package/dry_run_report.json`.

### 7. Documentation and Onboarding Agent
- **Recurring tasks**: Verifies public README commands against Makefile/scripts. Maintains contributor quickstart and updates "Verified Today" dashboard.
- **Outputs**: `reports/docs/docs_sync_report.md`.

---

## 4. Safety and Permission Model

Agents operate under an OpenBSD-style least-privilege security model:

| Agent Class | Filesystem Access | Network Access | Package Execution | Git Write |
|---|---|---|---|---|
| Documentation Agent | Read repo, write `docs/` & `reports/` | Optional allowlist | No | PR only |
| Test Runner | Read repo, write `reports/` & temp dirs | No | No | PR only |
| Boot Validator | Read repo, write `reports/boot/` | No | No | PR only |
| Capability Auditor | Read-only plus `reports/capability/` | No | No | Issue/PR only |
| Package Compatibility | Sandbox container only | No by default | Sandboxed dry-run only | PR only |
| Subsystem Agents | Scoped source paths in `src/` | No | No | PR only |

**Guardrails**:
- No agent receives blanket credentials, force-push rights, or direct `main` branch push access.
- Untrusted package scripts and downloaded binaries must run only inside isolated sandbox containers with explicit resource restrictions.

---

## 5. Machine-Readable Task Schema & CLI Tooling

Every agent job uses a standard TOML schema located under `.agents/tasks/`:

```toml
id = "BOOT-X86-001"
title = "Validate QEMU x86_64 boot-to-shell"
agent = "boot-runtime-validator"
priority = "P0"
scope = ["installer/", "src/kernel/", "scripts/", "tests/"]
repeat = "daily"

steps = [
  "check bootloader and ISO staging inputs",
  "execute QEMU x86_64 boot simulation",
  "parse serial log for boot milestones",
  "verify shell prompt readiness",
  "write boot validation report"
]

evidence = [
  "reports/boot/qemu_x86_64.log",
  "reports/boot/qemu_x86_64.json"
]

success = [
  "QEMU process completes or reaches shell within timeout",
  "kernel panic is false",
  "serial log contains valid boot signature"
]

on_failure = "create blocked task with failure classification and serial log excerpt"
```

### Automation Tools Introduced

1. `tools/agent_cli.py`: Task runner CLI for listing, selecting, executing, and reporting task results.
2. `tools/capability_audit.py`: Audit engine comparing TOML matrices against codebase state and generating `reports/capability/verified_today.md`.
3. `tools/boot_matrix.py`: QEMU boot execution engine for x86_64/ARM64 serial log analysis.
4. `tools/evidence_signer.py`: SHA256 checksum manifest generator for reports and evidence bundles.

---

## 6. Ninety-Day Execution Plan

### Days 1–14: Establish Truth
- Establish `.agents/` registry, task schemas, policies, and runbooks.
- Create structured `reports/` directory tree (`build/`, `test/`, `boot/`, `capability/`, `security/`, `package/`).
- Run initial capability audit across README, `CAPABILITY_MATRIX.toml`, and `FEATURE_STATUS.toml`. Publish initial `reports/capability/verified_today.md` dashboard.

### Days 15–35: Make the Build Trustworthy
- Automate clean build and test runs via `BUILD-TEST-001`.
- Add test failure classification and flaky-test detection (rerunning failures up to 3 times).
- Require PRs to attach a machine-readable verification report from `tools/agent_cli.py`.

### Days 36–60: Prove One Boot Path
- Focus on QEMU x86_64 boot-to-shell validation.
- Capture and retain serial logs for every kernel/bootloader PR under `reports/boot/`.
- Classify boot failures (bootloader, kernel panic, VFS, init, shell).

### Days 61–75: Connect Subsystems to Runtime
- Connect core kernel subsystems (memory allocator, VFS path resolver, process scheduler, IPC) to the boot-to-shell path.
- Replace "model passes unit test" claims with end-to-end integration traces.

### Days 76–90: Release Discipline & Channels
- Establish `dev`, `nightly`, and `validated` artifact channels.
- Publish nightly QEMU ISO builds with verified checksum manifests.
- Begin sandboxed package metadata parsing and dry-run dependency resolution.

---

## 7. Operational Metrics

Agents will track and publish these weekly metrics:
1. **Clean-build success rate**: Percentage of successful builds without error.
2. **Test suite pass rate & flaky count**: Pass rate across `./run_sigma_tests.sh` and standalone suites.
3. **QEMU boot success rate**: Percentage of successful QEMU boot-to-shell executions.
4. **Verified capability claims**: Count of components upgraded to `qemu-verified` status versus `model-only`.
5. **Time-to-verification**: Duration from commit push to evidence report generation.

---

## 8. Verification and Evidence

The infrastructure introduced in this PR has been validated:
1. **Python Tooling Compilation & Help Verification**:
   ```bash
   python3 -m py_compile tools/agent_cli.py tools/capability_audit.py tools/boot_matrix.py tools/evidence_signer.py
   python3 tools/agent_cli.py --help
   ```
2. **Dashboard & Report Generation**:
   ```bash
   python3 tools/capability_audit.py
   python3 tools/boot_matrix.py
   python3 tools/evidence_signer.py
   ```
   Verified generation of `reports/capability/truth_table.json`, `reports/capability/verified_today.md`, `reports/boot/qemu_x86_64.json`, and `reports/evidence_manifest.json`.
3. **Core Test Suite Verification**:
   Executed `./run_sigma_tests.sh` ensuring all 100+ native test suites pass cleanly.

---

## 9. Definition of Done

A task or feature in SigmaOS is complete only when:
1. An issue/task definition exists in `.agents/tasks/`.
2. Source code and tests exist in `src/` and `tests/`.
3. The change is connected to a real boot or runtime path.
4. Machine-readable evidence exists under `reports/`.
5. `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml` accurately reflect evidence-based status.
6. Documentation in `README.md` and `reports/capability/verified_today.md` matches the implementation.
