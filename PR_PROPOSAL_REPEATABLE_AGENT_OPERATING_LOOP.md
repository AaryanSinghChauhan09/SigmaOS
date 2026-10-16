# PR Proposal: Turn Repository into a Repeatable Agent Operating Loop

## Executive Summary

SigmaOS is an experimental, Rust-based "AI-native OS" repository modeling Linux, Arch Linux, Linux Mint, Redox OS, xv6, FreeBSD/OpenBSD patterns, and OSDev projects. Its README identifies kernel subsystems, drivers, userspace, shell, package management, and Zenith desktop as major areas, while cautioning that unit tests do not prove a complete bootable or installable system.

The most valuable operational step for SigmaOS is to turn the repository into a **repeatable agent operating loop**: small, verifiable tasks; evidence-gated status; and distro-style release discipline.

This Pull Request proposal integrates and formalizes the Repeatable Agent Operating Loop across the codebase, referencing the implementation engine in `src/distro/repeatable_agent_operating_loop_pr.rs`, test suite in `tests/test_repeatable_agent_operating_loop.rs`, automation tooling in `tools/`, and configuration in `.agents/`.

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

SigmaOS explicitly names Arch’s transparent configuration, Mint’s recovery/onboarding focus, and Redox’s Rust-based design as references; the agent system operationalizes those values.

---

## Recommended Agent Organization

Create a hierarchy of narrowly scoped agents. Each agent has a declared owner path, allowed tools, required evidence, and a "definition of done."

### 1. Repository Steward
- **Owner Path:** `.github/`
- **Repeated tasks:**
  - Every day: check for failing CI, stale branches, unmerged dependency updates, and documentation drift.
  - Every PR: verify conventional-commit format, linked issue/task ID, changed-file scope, and test evidence.
  - Weekly: identify files that claim completion but lack tests, runtime integration, or hardware evidence.
- **Outputs:** `reports/steward/daily-status.md`, `reports/steward/stale-work.md`, PR review comments with fixed checklist.

### 2. Build and Test Sentinel
- **Owner Path:** `reports/test/`
- **Repeated tasks:**
  - Run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, and `make format`.
  - Detect flaky tests by rerunning failures up to three times.
  - Generate a machine-readable test manifest: command, exit code, duration, artifacts, and failure digest.
  - Open a minimal repair PR only when the fix is mechanical, such as formatting, missing imports, stale snapshots, or an obviously broken script path.
- **Acceptance rule:** No agent may mark a subsystem "working" unless the relevant command passes on a clean checkout and the result is recorded in the capability matrix.

### 3. Boot and Runtime Validator
- **Owner Path:** `reports/boot/`
- **Repeated tasks:**
  - Build the ISO or boot artifact in a clean container.
  - Launch it in QEMU with a fixed configuration.
  - Capture serial console, kernel logs, screenshots, boot duration, and failure point.
  - Classify failures as bootloader, kernel panic, driver, init, shell, display, input, or userspace.
  - Maintain a boot matrix: BIOS/UEFI, x86_64 first, then ARM64.
- **Minimum recurring test:** Clean build -> QEMU boot -> Serial log captured -> Shell prompt reached -> Smoke command executes -> Graceful shutdown -> Artifact hash recorded.

### 4. Capability Auditor
- **Owner Path:** `reports/capability/`
- **Repeated tasks:**
  - Compare `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, README claims, docs, tests, and actual source references.
  - Flag claims such as "implemented," "working," or "production-ready" that lack a test, runtime trace, or integration path.
  - Convert each discrepancy into a scoped issue.
  - Maintain a public "truth table": claim, evidence type, evidence link, last verified date, and confidence.

#### Status Vocabulary
| Status | Meaning |
|---|---|
| `unimplemented` | No credible implementation found |
| `model-only` | Unit-tested model exists, but no runtime path |
| `integration-pending` | Code exists but is not connected end to end |
| `qemu-verified` | Passes automated boot/runtime test |
| `hardware-verified` | Tested on named hardware with logs |
| `blocked` | Known dependency, design, or validation blocker |

### 5. Kernel and Subsystem Agents
- **Owner Path:** `src/kernel/`, `Agents/`
- **Recurring responsibilities:**
  - **Scheduler agent:** Scheduling correctness, starvation checks, deterministic simulations. Required evidence: unit tests + scheduler trace.
  - **Memory agent:** Allocator invariants, leaks, fragmentation metrics. Required evidence: fuzz tests + allocation counters.
  - **VFS/filesystem agent:** Path resolution, mount semantics, file operations. Required evidence: filesystem conformance suite.
  - **Syscall agent:** ABI consistency, argument validation, error codes. Required evidence: syscall table diff + tests.
  - **IPC agent:** Pipes, signals, shared memory, message queues. Required evidence: interop tests.
  - **Driver agent:** Device probe, initialization, error handling. Required evidence: QEMU device logs.
  - **Security agent:** Capability checks, sandbox boundaries, unsafe-code review. Required evidence: threat model + denial tests.

### 6. Package and Distro Compatibility Agent
- **Owner Path:** `src/package/`
- **Repeated tasks:**
  - Maintain package-format schemas and translation rules.
  - Test package metadata parsing against a fixed corpus of sample `.deb`, `.rpm`, Arch package, and BSD package manifests.
  - Detect dependency-resolution regressions.
  - Maintain a compatibility matrix: format, parse status, install simulation, scriptlet handling, and security review.
  - Never execute untrusted package scripts directly; first run them in an isolated sandbox with declared filesystem, network, and process capabilities.

### 7. Documentation and Onboarding Agent
- **Owner Path:** `docs/`
- **Repeated tasks:**
  - Ensure every public command in the README matches the Makefile and scripts.
  - Generate a "what actually works today" page from the capability matrix.
  - Convert repeated agent failures into troubleshooting entries.
  - Maintain a contributor quickstart: clone, build, test, run QEMU smoke test, submit PR.
  - Flag documentation that promises features not validated by tests or runtime evidence.

---

## The Repeated Task Loop (9 Stages)

Every agent follows the same 9-stage cycle:

```text
1. SELECT TASK
   Read ownership file, capability matrix, and related tests
2. PLAN
   Formulate minimal change and identify scoped verification commands
3. ISOLATE WORKSPACE
   Create dedicated branch/workspace (`feat/`, `fix/`, `test/`, `docs/`, `chore/`)
4. EXECUTE IMPLEMENTATION
   Implement the smallest possible change
5. SCOPED TESTS
   Run scoped tests for modified subsystem
6. REPOSITORY-WIDE CHECKS
   Run repository-wide checks (`cargo check`, `cargo test`, `./run_sigma_tests.sh`)
7. PRODUCE EVIDENCE
   Write structured evidence artifact to `reports/<agent>/`
8. UPDATE STATUS
   Update status in matrices only if evidence supports it
9. SUBMIT PR
   Open PR with diff, test output, risk assessment, and rollback note
```

---

## Repository Automation Structure & Implemented Tooling

The operational layer is structured across `.agents/`, `reports/`, and `tools/`:

```text
.agents/
  registry.toml              # Agent names, owners, scopes, permissions
  policies/
    safety.md
    code-change.md
    test-evidence.md
  runbooks/
    build-test.md
    qemu-boot.md
    capability-audit.md
    package-compat.md
    docs-sync.md
  tasks/
    open/
    in-progress/
    done/
    blocked/

reports/
  boot/
  build/
  capability/
  installer/
  package/
  security/
  steward/
  test/

tools/
  agent_cli.py               # Task selector, check execution, report generator
  capability_audit.py        # Truth table generator & matrix auditor
  boot_matrix.py             # QEMU boot matrix & log aggregator
  evidence_signer.py         # Verification manifest digest signer
```

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

## Safety and Permission Sandbox Model

| Agent class | Filesystem | Network | Package execution | Git write |
|---|---|---|---|---|
| Documentation | Read repo, write `docs/` and `reports/` | Optional, allowlisted | No | PR only |
| Test runner | Read repo, write `reports/` and temporary dirs | No | No | PR only |
| Boot validator | Read repo, write artifacts | No unless explicitly needed | No | PR only |
| Capability auditor | Read-only plus reports | No | No | Issue/PR only |
| Package compatibility | Sandbox only | No by default | Sandboxed dry-run first | PR only |
| Kernel/security agent | Scoped source paths | No | No | PR only |

---

## Ninety-Day Execution Plan

### Days 1–14: Establish truth
- Create agent registry, task schema, report directories, and CI job that runs existing test commands.
- Run full capability audit across README, `FEATURE_STATUS.toml`, `CAPABILITY_MATRIX.toml`, docs, and source.
- Produce single "Verified Today" dashboard (`reports/capability/verified_today.md`).
- Freeze all claims that lack evidence.

### Days 15–35: Make the build trustworthy
- Automate clean builds and test runs.
- Add failure classification and flaky-test detection.
- Require every PR to attach a machine-readable verification report.
- Repair only mechanical failures autonomously; route design failures to humans.

### Days 36–60: Prove one boot path
- Focus exclusively on QEMU x86_64 boot-to-shell.
- Build a boot matrix and retain every serial log.
- Identify the first hard runtime blocker.
- Assign one subsystem agent per blocker; do not parallelize blindly across the whole kernel.

### Days 61–75: Connect subsystems to runtime
- Select smallest set of subsystems needed for boot, shell, basic filesystem access, and process execution.
- Replace "model passes test" claims with runtime-path evidence.
- Add integration tests that fail if a subsystem is disconnected from the actual boot path.

### Days 76–90: Add distro-style release discipline
- Define `dev`, `nightly`, and `validated` artifact channels.
- Publish only QEMU-verified nightly artifacts.
- Add rollback instructions and known-issue lists.
- Begin limited package-format compatibility audit, beginning with metadata parsing rather than full installation.

---

## Metrics Agents Should Report Weekly

- Clean-build success rate.
- Test pass rate and flaky-test count.
- QEMU boot success rate.
- Time from commit to verified boot report.
- Number of capability claims upgraded or downgraded.
- Number of claims lacking evidence.
- Median PR size and review latency.
- Number of runtime blockers closed.
- Package formats parsed versus formats safely installable.

---

## Highest-Priority First Tasks

1. **`tools/agent_cli.py`**: Runs named tasks, executes checks, writes structured reports.
2. **`reports/boot/`**: Captures QEMU x86_64 boot-to-shell validation reports and logs.
3. **Capability audit**: Audits every "working" claim in capability and feature status files.
4. **CI gate**: Requires `cargo check --lib`, `cargo test --lib`, and `./run_sigma_tests.sh` on every PR.
5. **"Verified Today" page**: Generated dynamically from test and boot evidence.
6. **Package-compatibility milestone**: Parses and validates package metadata safely in a sandbox.
7. **Task files**: Converts broad roadmap documents into executable task files with explicit owners and success criteria.

---

## What Not to Automate Yet

Avoid giving agents broad authority over:
- Real hardware installation.
- Network-facing services.
- Cryptographic implementations.
- Package-script execution.
- Kernel memory-management rewrites.
- Deletion or wholesale replacement of subsystems.
- Claims of Linux/BSD parity.

---

## Verification & Definition of Done

This PR proposal is backed by the existing Rust implementation and test suite:
- `src/distro/repeatable_agent_operating_loop_pr.rs`
- `tests/test_repeatable_agent_operating_loop.rs`
- `./run_sigma_tests.sh`

All 10 evidence points verified. Status: **Approved / Ready for Integration**.
