# Pull Request Proposal: SigmaOS Distro-Maintainer Operating Model & Autonomous Execution Framework

## Pull Request Summary
- **Title**: `feat(distro): Adopt Distro-Maintainer Operating Model and Evidence-Gated Agent Framework`
- **Branch**: `feature/distro-maintainer-operating-model`
- **Status**: `Proposed / Specification Ready`
- **Scope**: Distro maintenance, agent roles, task lifecycles, report governance, release channels, and runtime evidence verification.

---

## 1. Executive Overview & Core Operating Model
SigmaOS adopts a **distro-maintainer operating model** where autonomous agents perform small, repeatable, evidence-producing tasks; humans approve high-impact changes; and no feature is considered complete until it is built, tested, booted, and documented.

SigmaOS explicitly draws on Linux, Arch, Mint, Redox, xv6, and BSD traditions. Source presence and unit test passes alone do not establish a bootable, installable, or hardware-supported operating system. The operating model optimizes for verified runtime reality over documentation volume.

### Operating Paradigm Mapping

| Open-Source OS Practice | SigmaOS Agent Practice |
| :--- | :--- |
| **Package maintainer owns a package** | One agent owns one subsystem or module boundary |
| **Debian/Arch release process** | Changes enter through PRs with attached test/boot evidence |
| **ArchWiki reproducibility** | Every recurring task has a machine-readable runbook |
| **FreeBSD/OpenBSD security discipline** | Least privilege, sandboxing, audit trails |
| **ISO build and smoke testing** | Every integration change must pass a boot/runtime gate |
| **Stable, testing, unstable channels** | `verified`, `experimental`, and `blocked` states |
| **Regression tracking** | Every failed test or boot becomes a tracked task |
| **Reproducible builds** | Fixed toolchain, lockfiles, recorded commands and hashes |

---

## 2. Seven Dedicated Agent Roles

### 1. Repository Steward
- **Purpose**: Maintain repository hygiene, coherence, and code review readiness.
- **Repeated Tasks**:
  - *Daily*: Inspect CI failures, stale branches, unmerged dependencies, and broken documentation links.
  - *Per PR*: Enforce conventional commit formats, issue linkage, scope limits, and required test evidence.
  - *Weekly*: Identify orphaned files, duplicate plans, contradictory status claims, and abandoned work.
- **Deliverables**:
  - `reports/steward/daily-status.md`
  - `reports/steward/stale-work.md`
  - Structured PR review checklist output.

### 2. Build and Test Sentinel
- **Purpose**: Make build and test workflows trustworthy and reproducible.
- **Repeated Tasks**:
  - Run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, and `make format` on a clean checkout.
  - Rerun failures up to three times to distinguish real regressions from flaky tests.
  - Produce a JSON/Markdown report containing command, exit code, duration, changed files, artifacts, and failure digest.
  - Open mechanical repair PRs for formatting, imports, stale snapshots, or broken script paths.
- **Rule**: The agent may mark a build "green" only after a clean checkout and recorded command output.

### 3. Boot and Runtime Validator
- **Purpose**: Close SigmaOS's most critical gap: proving the firmware-to-desktop path.
- **Repeated Tasks**:
  - Build the ISO/boot artifact in a clean environment.
  - Boot in QEMU with fixed CPU, memory, firmware, disk, and serial-log settings.
  - Capture serial output, screenshots, boot duration, kernel messages, and first failure point.
  - Classify failures (bootloader, kernel, driver, init, shell, display, input, filesystem, userspace).
  - Maintain a boot matrix beginning with Tier 1 QEMU x86_64 before expanding.
- **Minimum Daily Smoke Test**: Clean build -> QEMU boot -> Serial log captured -> Shell prompt reached -> Basic command executes -> Graceful shutdown -> Artifact hash & QEMU version recorded.

### 4. Capability Auditor
- **Purpose**: Prevent "aspirational completion" and enforce truth in advertising.
- **Repeated Tasks**:
  - Cross-check `README.md` claims, `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, source code, tests, docs, and runtime reports.
  - Flag any claim of "implemented", "working", or "production-ready" that lacks test, integration, or boot evidence.
  - Create narrowly scoped issues for unsupported claims and maintain the public evidence table.

#### Public Evidence Table Schema
| Claim | Evidence Type | Evidence Link | Last Verified | Status |
| :--- | :--- | :--- | :--- | :--- |
| Kernel scheduler works | Unit test + trace | `reports/test/...` | Date | `model-only` |
| Filesystem mount works | QEMU runtime log | `reports/boot/...` | Date | `qemu-verified` |
| Installer recovery works | Installer test + rollback log | `reports/installer/...` | Date | `integration-pending` |

### 5. Kernel and Subsystem Agents
Uses existing component-agent boundaries:
- **Scheduler Agent**: Scheduling correctness, starvation prevention, deterministic behavior. (*Evidence*: Unit tests + scheduler trace).
- **Memory Agent**: Allocator invariants, leak checks, fragmentation metrics. (*Evidence*: Fuzz tests + allocation metrics).
- **VFS/Filesystem Agent**: Path resolution, mount semantics, file operations. (*Evidence*: Filesystem conformance suite).
- **Syscall Agent**: ABI consistency, argument validation, error codes. (*Evidence*: Syscall table diff + tests).
- **IPC Agent**: Pipes, signals, shared memory, message queues. (*Evidence*: Cross-process interoperability tests).
- **Drivers Agent**: Probe, initialization, error handling. (*Evidence*: QEMU device logs).
- **Security Agent**: Capability checks, sandbox boundaries, unsafe code audit. (*Evidence*: Threat model + denial tests).
- **Rule**: A subsystem agent may not modify another subsystem's public interface without an integration review and updated tests.

### 6. Package and Distro Compatibility Agent
- **Purpose**: Maintain SigmaPkg and Linux/BSD package compatibility.
- **Repeated Tasks**:
  - Maintain schemas and translation rules for package metadata.
  - Test parsing against fixed corpora of `.deb`, `.rpm`, Arch, and BSD manifests.
  - Track dependency-resolution regressions.
  - Maintain compatibility matrix across format, metadata parse, dependency graph, dry-run install, scriptlet policy, and security review.
- **Rule**: Package scripts and downloaded binaries must never execute with broad host access. Use an isolated sandbox with explicitly declared capabilities.

### 7. Documentation and Onboarding Agent
- **Purpose**: Ensure usability and discoverability for human contributors and agents.
- **Repeated Tasks**:
  - Verify that `README.md` commands match Makefiles and scripts.
  - Generate a "What works today" page from verified capability data.
  - Convert repeated failures into troubleshooting entries.
  - Maintain contributor quickstart (clone, build, test, QEMU smoke test, submit PR).
  - Flag documentation that promises unverified features.

---

## 3. Standard Task Lifecycle (9 Steps)

Every repeated task follows a strict 9-stage loop:

```text
1. SELECT        Read agent ownership file, capability matrix, and task queue.
2. PLAN          Identify affected paths, tests, risks, and rollback method.
3. ISOLATE       Create a branch, workspace, or container.
4. EXECUTE       Make the smallest possible change.
5. VERIFY        Run scoped tests, repository checks, and runtime tests.
6. RECORD        Write logs, metrics, hashes, and a structured report.
7. UPDATE STATUS Change capability status only if evidence supports it.
8. REVIEW        Submit PR; human review is mandatory for kernel, security,
                 installer, boot, and package-execution changes.
9. FOLLOW UP     If verification fails, create a blocked task with exact
                 failing stage, log excerpt, and reproduction command.
```

---

## 4. Repository Layout Contract

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

---

## 5. Declarative Task Schema

Each recurring job uses a single machine-readable TOML task file:

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

---

## 6. Ten Mandatory Operating Rules

1. **Evidence Before Status**: No status updates without attached passing test output, QEMU boot log, installer/recovery log, hardware test log, or security review record.
2. **One Agent, One Boundary**: Agents modify only paths in their declared scope. Cross-boundary changes require integration tasks.
3. **Small Changes Only**: PRs address one concern (one bug, test, doc, config, or integration slice). Large PRs must be split.
4. **Reproducibility is Mandatory**: Reports require exact commands, commit hash, toolchain version, OS/container image, timestamps, exit codes, artifact hashes, and log excerpts.
5. **Human Approval for High-Impact Actions**: Human approval is mandatory before changing kernel security boundaries, modifying bootloaders/installers, executing package scripts, running downloaded third-party code, altering release channels, deleting subsystems, or force-pushing protected branches.
6. **Sandbox Untrusted Execution**: All external code, packages, scripts, models, and binaries are untrusted by default.
7. **No Silent Failure**: Uncompleted tasks spawn blocked tasks with full diagnostic context.
8. **Claims Must Be Versioned**: Unverified claims older than the release cycle are automatically downgraded to `needs-revalidation`.
9. **Security Changes Get Separate Review**: Security-relevant changes require dedicated human review.
10. **Prefer Runtime Proof Over Model Proof**: Runtime boot and integration results control public status over unit test logic alone.

---

## 7. Release Channels

- **`dev`**: Latest agent-generated branch artifacts. Requires clean build and passing unit tests.
- **`nightly`**: Automated daily artifact. Requires clean build, passing unit tests, and passing QEMU smoke test.
- **`validated`**: Recommended test release. Requires full boot matrix, installer test, known-issue list, and rollback instructions.
- **`experimental`**: New subsystem or compatibility features. Clearly labeled, not for daily use.

*Note: No stable release is published until boot, installation, update/recovery, and basic desktop operation are reproducibly verified.*

---

## 8. Ninety-Day Plan & First Ten Backlog Tasks

### Roadmap
- **Days 1–14 (Ground Truth)**: Agent registry, task schema, report directories, evidence policy, initial audit report, downgrade unevidenced claims.
- **Days 15–35 (Build & Test Stability)**: Automate clean builds and tests, flaky test detection, failure classification, verification reports on PRs.
- **Days 36–60 (Single Boot Path)**: Focus on QEMU x86_64 boot-to-shell, persistent boot matrix, single-blocker subsystem agent assignment.
- **Days 61–75 (Runtime Integration)**: Connect minimal required subsystems to actual boot path, replace model-only claims with runtime evidence.
- **Days 76–90 (Release Discipline)**: Release channels, nightly QEMU-verified artifacts, rollback guides, package metadata compatibility testing.

### First Ten Backlog Tasks
1. Create `.agents/registry.toml` defining agents, scopes, permissions, and report paths.
2. Create `.agents/policies/test-evidence.md` as mandatory policy for status changes.
3. Build `tools/agent_cli.py` to execute tasks and write structured reports.
4. Add CI workflow running documented check and test commands.
5. Create `reports/boot/` and implement QEMU x86_64 boot-to-shell validator.
6. Audit every "working" or "complete" claim in capability and feature status files.
7. Generate "Verified Today" dashboard from test and boot evidence.
8. Convert five largest roadmap documents into executable task files.
9. Add package metadata compatibility test corpus and dry-run policy.
10. Define initial nightly artifact requirements (clean build, tests, QEMU boot, logs, rollback notes).
