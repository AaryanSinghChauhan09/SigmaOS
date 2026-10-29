# PR Proposal: Adopt Distro-Maintainer Operating Model and Evidence-Gated Agent Framework

- **PR Title**: `feat(distro): Adopt Distro-Maintainer Operating Model and Evidence-Gated Agent Framework`
- **PR Branch**: `feature/distro-maintainer-operating-model`
- **Status**: Approved / Specification Ready

## Executive Summary

SigmaOS should adopt a distro-maintainer operating model: autonomous agents perform small, repeatable, evidence-producing tasks; humans approve high-impact changes; and no feature is called complete until it is built, tested, booted, and documented. The repository already provides the right raw materials—AGENTS.md, subsystem agent files, capability/feature-status files, CI, scripts, tests, installer assets, and Make targets—but it needs a strict recurring execution system.

Treat SigmaOS like a small Linux/BSD distribution project, not a single monolithic AI project.

## Core Operating Model Matrix

| Open-source OS practice | SigmaOS agent practice |
|---|---|
| **Package maintainer owns a package** | One agent owns one subsystem or module boundary |
| **Debian/Arch release process** | Changes enter through PRs with test evidence |
| **ArchWiki reproducibility** | Every recurring task has a machine-readable runbook |
| **FreeBSD/OpenBSD security discipline** | Least privilege, sandboxing, audit trails |
| **ISO build and smoke testing** | Every integration change must pass a boot/runtime gate |
| **Stable, testing, unstable channels** | verified, experimental, and blocked states |
| **Regression tracking** | Every failed test or boot becomes a tracked task |
| **Reproducible builds** | Fixed toolchain, lockfiles, recorded commands and hashes |

SigmaOS explicitly draws on Linux, Arch, Mint, Redox, xv6, and BSD traditions, while its README stresses that unit tests and source presence do not establish a bootable, installable, or hardware-supported system. The agent plan must therefore optimize for verified reality, not documentation volume.

## Agent Roles

### 1. Repository Steward
- **Purpose**: Keep the repository clean, coherent, and reviewable.
- **Repeated Tasks**:
  - *Daily*: inspect CI failures, stale branches, unmerged dependency updates, and broken documentation links.
  - *Per PR*: enforce conventional commits, issue linkage, scope limits, and required test evidence.
  - *Weekly*: identify orphaned files, duplicate plans, contradictory status claims, and abandoned work.
- **Deliverables**: `reports/steward/daily-status.md`, `reports/steward/stale-work.md`, PR review checklist output.

### 2. Build and Test Sentinel
- **Purpose**: Make the existing build and test workflow trustworthy.
- **Repeated Tasks**:
  - Run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, and `make format` on a clean checkout.
  - Rerun failures up to three times to distinguish real regressions from flaky tests.
  - Produce a JSON/Markdown report containing command, exit code, duration, changed files, artifacts, and failure digest.
  - Open mechanical repair PRs for formatting, imports, stale snapshots, or broken script paths.
- **Rule**: The agent may mark a build “green” only after a clean checkout and recorded command output.

### 3. Boot and Runtime Validator
- **Purpose**: Close SigmaOS’s most important gap: proving the firmware-to-desktop path. The README identifies boot-to-shell, installer/update recovery, and runtime integration as near-term priorities.
- **Repeated Tasks**:
  - Build the ISO/boot artifact in a clean environment.
  - Boot it in QEMU with fixed CPU, memory, firmware, disk, and serial-log settings.
  - Capture serial output, screenshots, boot duration, kernel messages, and the first failure point.
  - Classify failures as bootloader, kernel, driver, init, shell, display, input, filesystem, or userspace.
  - Maintain a boot matrix beginning with QEMU x86_64, then expanding to other supported targets.
- **Minimum Daily Smoke Test**: Clean build → QEMU boot → Serial log captured → Shell prompt reached → Basic command executes → Graceful shutdown or controlled reset → Artifact hash and QEMU version recorded.
- **Target Focus**: QEMU x86_64 as the primary Tier 1 target, with ARM64 as a later milestone.

### 4. Capability Auditor
- **Purpose**: Prevent “aspirational completion.”
- **Repeated Tasks**:
  - Cross-check README claims, `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, source code, tests, docs, and runtime reports.
  - Flag any claim of “implemented,” “working,” or “production-ready” that lacks test, integration, or boot evidence.
  - Create narrowly scoped issues for unsupported claims.
  - Maintain a public evidence table.

#### Required Evidence Table Format

| Claim | Evidence type | Evidence link | Last verified | Status |
|---|---|---|---|---|
| Kernel scheduler works | Unit test + trace | `reports/test/...` | Date | `model-only` |
| Filesystem mount works | QEMU runtime log | `reports/boot/...` | Date | `qemu-verified` |
| Installer recovery works | Installer test + rollback log | `reports/installer/...` | Date | `integration-pending` |

### 5. Kernel and Subsystem Agents
Use the existing component-agent structure as ownership boundaries.

| Agent | Recurring responsibility | Minimum evidence |
|---|---|---|
| **Scheduler** | Scheduling correctness, starvation, deterministic behavior | Unit tests plus scheduler trace |
| **Memory** | Allocator invariants, leaks, fragmentation | Fuzz tests and allocation metrics |
| **VFS/filesystem** | Path resolution, mount semantics, file operations | Filesystem conformance suite |
| **Syscall** | ABI consistency, argument validation, error codes | Syscall table diff and tests |
| **IPC** | Pipes, signals, shared memory, message queues | Cross-process interoperability tests |
| **Drivers** | Probe, initialization, error handling | QEMU device logs |
| **Security** | Capability checks, sandbox boundaries, unsafe code | Threat model and denial tests |

- **Rule**: A subsystem agent may not modify another subsystem’s public interface without an integration review and updated tests.

### 6. Package and Distro Compatibility Agent
- **Purpose**: Maintain package metadata compatibility and sandboxed execution.
- **Repeated Tasks**:
  - Maintain schemas and translation rules for package metadata.
  - Test parsing against a fixed corpus of `.deb`, `.rpm`, Arch package, and BSD package manifests.
  - Track dependency-resolution regressions.
  - Maintain a compatibility matrix: format, metadata parse, dependency graph, dry-run install, scriptlet policy, and security review.
  - Begin with metadata parsing and dependency analysis before attempting real binary installation.
- **Rule**: Package scripts and downloaded binaries must never execute with broad host access. Use an isolated sandbox with explicitly declared filesystem, process, and network capabilities.

### 7. Documentation and Onboarding Agent
- **Purpose**: Make SigmaOS usable by contributors, not only by its author.
- **Repeated Tasks**:
  - Verify that README commands match Makefiles and scripts.
  - Generate a “What works today” page from verified capability data.
  - Convert repeated failures into troubleshooting entries.
  - Maintain a contributor quickstart: clone, build, test, QEMU smoke test, submit PR.
  - Flag documentation that promises unverified features.

## Standard Task Lifecycle

Every repeated task follows the 9-stage execution loop:

```text
1. SELECT
   Read agent ownership file, capability matrix, and task queue.

2. PLAN
   Identify affected paths, tests, risks, and rollback method.

3. ISOLATE
   Create a branch, workspace, or container.

4. EXECUTE
   Make the smallest possible change.

5. VERIFY
   Run scoped tests, repository checks, and runtime tests where applicable.

6. RECORD
   Write logs, metrics, hashes, and a structured report.

7. UPDATE STATUS
   Change capability status only if evidence supports it.

8. REVIEW
   Submit PR; human review is mandatory for kernel, security,
   installer, boot, and package-execution changes.

9. FOLLOW UP
   If verification fails, create a blocked task with the exact
   failing stage, log excerpt, and reproduction command.
```

## Repository Layout Contract

```text
.agents/
  registry.toml
  policies/
    safety.md
    code-change.md
    test-evidence.md
    release.md
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
  build/
  test/
  boot/
  capability/
  installer/
  package/
  security/

tools/
  agent_cli.py
  capability_audit.py
  boot_matrix.py
  evidence_signer.py
```

## Task File Format

Every recurring job is stored as a declarative TOML file:

```toml
id = "BOOT-X86-001"
title = "Validate QEMU x86_64 boot-to-shell"
agent = "boot-runtime-validator"
priority = "P0"
repeat = "daily"
scope = ["installer/", "src/kernel/", "scripts/", "tests/"]

steps = [
  "make clean",
  "make check",
  "make test",
  "build ISO artifact",
  "boot QEMU with serial logging",
  "wait for shell prompt",
  "run smoke commands",
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

on_failure = "create blocked task with failing stage and log excerpt"
```

## Mandatory Operating Rules

1. **Rule 1: Evidence before status**
   No agent may update `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, or documentation to say a feature works unless it attaches passing test output, QEMU boot/runtime log, installer/recovery test result, hardware test log, or security review record.
2. **Rule 2: One agent, one boundary**
   Each agent may modify only the paths listed in its ownership declaration. Cross-boundary changes require a separate integration task and review.
3. **Rule 3: Small changes only**
   A PR should ideally change one concern: one bug, one test, one document, one configuration, or one subsystem integration.
4. **Rule 4: Reproducibility is mandatory**
   Every report must include exact commands, commit hash, toolchain versions, OS/container image, timestamps, exit codes, artifact hashes, and log excerpts.
5. **Rule 5: Human approval for high-impact actions**
   Agents may autonomously run tests, generate reports, open issues, create documentation PRs, or fix mechanical test failures. Agents must obtain human approval before changing security boundaries, modifying bootloaders/installers/update mechanisms, executing package scripts, running 3rd-party code, changing release channels, deleting subsystems, or force-pushing.
6. **Rule 6: Sandbox untrusted execution**
   All external repositories, packages, scripts, models, and binaries are untrusted by default.
7. **Rule 7: No silent failure**
   If an agent cannot complete a task, it must create a blocked task containing Task ID, failing step, error message, log excerpt, suspected cause, and unblock info.
8. **Rule 8: Claims must be versioned**
   Every capability status must include a last-verified date and evidence link. Unverified claims older than the release cycle are downgraded to `needs-revalidation`.
9. **Rule 9: Security changes get separate review**
   Security-relevant changes must never be merged solely on an agent’s recommendation.
10. **Rule 10: Prefer runtime proof over model proof**
    When a subsystem has both a unit test and a runtime path, the runtime result controls the public status.

## Release Channels

| Channel | Meaning | Requirements |
|---|---|---|
| **dev** | Latest agent-generated branch artifacts | Build and unit tests pass |
| **nightly** | Automated daily artifact | Build, tests, and QEMU smoke test pass |
| **validated** | Recommended test release | Full boot matrix, installer test, known-issue list, rollback instructions |
| **experimental** | New subsystem or compatibility feature | Clearly labeled, not suitable for daily use |

## Ninety-Day Execution Plan

- **Days 1–14: Establish ground truth**
  Create agent registry, task schema, report directories, and evidence policy. Run a full audit of README, capability files, docs, source, and tests. Generate the first “Verified Today” report. Downgrade unevidenced claims to unverified or model-only.
- **Days 15–35: Stabilize build and tests**
  Automate clean builds and all existing test commands. Add flaky-test detection and failure classification. Require a verification report on every PR. Allow autonomous fixes only for mechanical failures.
- **Days 36–60: Prove one boot path**
  Focus on QEMU x86_64 boot-to-shell. Capture every boot attempt in a persistent boot matrix. Identify the first hard runtime blocker. Assign one subsystem agent per blocker.
- **Days 61–75: Connect subsystems to runtime**
  Select minimum subsystems required for boot, shell, filesystem access, and process execution. Replace “model passes test” claims with runtime-path evidence.
- **Days 76–90: Add release discipline**
  Define dev, nightly, validated, and experimental channels. Publish only QEMU-verified nightly artifacts. Add rollback instructions and known-issue lists. Start package metadata parsing.

## First Ten Backlog Tasks

1. `REG-001`: Create `.agents/registry.toml` defining every agent, scope, permissions, and report location.
2. `POL-001`: Create `.agents/policies/test-evidence.md` and make it mandatory for status changes.
3. `CLI-001`: Build `tools/agent_cli.py` to select a task, execute checks, and write structured reports.
4. `CI-001`: Add a CI workflow that runs the repository’s documented check and test commands.
5. `BOOT-X86-001`: Create `reports/boot/` and implement the QEMU x86_64 boot-to-shell validator.
6. `AUDIT-001`: Audit every “working” or “complete” claim in the capability and feature-status files.
7. `DASH-001`: Generate a “Verified Today” dashboard from test and boot evidence.
8. `TASK-001`: Convert the five largest roadmap documents into executable task files.
9. `PKG-001`: Add a package-metadata compatibility test corpus and dry-run policy.
10. `NIGHTLY-001`: Define the first nightly artifact requirements: clean build, tests, QEMU boot, logs, and rollback notes.

## Native Rust Engine Support

The Distro-Maintainer Operating Model PR Gateway Engine is implemented in `src/distro/distro_maintainer_operating_model_pr.rs`, re-exported in `src/distro/mod.rs`, and tested via `tests/test_distro_maintainer_operating_model.rs`.
