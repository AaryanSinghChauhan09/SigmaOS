# Pull Request: Adopt Distro-Maintainer Operating Model and Evidence-Gated Agent Framework

## Title
`feat(distro): Adopt Distro-Maintainer Operating Model and Evidence-Gated Agent Framework`

## Summary
This proposal establishes a distro-maintainer operating model for SigmaOS: autonomous agents perform small, repeatable, evidence-producing tasks; humans approve high-impact changes; and no feature is called complete until it is built, tested, booted, and documented.

SigmaOS explicitly draws on Linux, Arch, Mint, Redox, xv6, and BSD traditions, while optimizing for verified reality rather than documentation volume.

---

## Core Operating Model
Treat SigmaOS like a small Linux/BSD distribution project, not a single monolithic AI project.

| Open-source OS practice | SigmaOS agent practice |
| :--- | :--- |
| Package maintainer owns a package | One agent owns one subsystem or module boundary |
| Debian/Arch release process | Changes enter through PRs with test evidence |
| ArchWiki reproducibility | Every recurring task has a machine-readable runbook |
| FreeBSD/OpenBSD security discipline | Least privilege, sandboxing, audit trails |
| ISO build and smoke testing | Every integration change must pass a boot/runtime gate |
| Stable, testing, unstable channels | `verified`, `experimental`, and `blocked` states |
| Regression tracking | Every failed test or boot becomes a tracked task |
| Reproducible builds | Fixed toolchain, lockfiles, recorded commands and hashes |

---

## 7 Dedicated Agent Roles

1. **Repository Steward**
   - *Purpose*: Keep the repository clean, coherent, and reviewable.
   - *Daily*: Inspect CI failures, stale branches, unmerged dependency updates, and broken documentation links.
   - *Per PR*: Enforce conventional commits, issue linkage, scope limits, and required test evidence.
   - *Weekly*: Identify orphaned files, duplicate plans, contradictory status claims, and abandoned work.
   - *Deliverables*: `reports/steward/daily-status.md`, `reports/steward/stale-work.md`, PR review checklist output.

2. **Build and Test Sentinel**
   - *Purpose*: Make the existing build and test workflow trustworthy.
   - *Repeated Tasks*: Run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, and `make format` on a clean checkout. Rerun failures up to three times to distinguish real regressions from flaky tests. Produce a JSON/Markdown report containing command, exit code, duration, changed files, artifacts, and failure digest. Open mechanical repair PRs for formatting, imports, stale snapshots, or broken script paths.
   - *Rule*: The agent may mark a build “green” only after a clean checkout and recorded command output.

3. **Boot and Runtime Validator**
   - *Purpose*: Close SigmaOS’s most important gap: proving the firmware-to-desktop path.
   - *Repeated Tasks*: Build the ISO/boot artifact in a clean environment. Boot it in QEMU with fixed CPU, memory, firmware, disk, and serial-log settings. Capture serial output, screenshots, boot duration, kernel messages, and the first failure point. Classify failures as bootloader, kernel, driver, init, shell, display, input, filesystem, or userspace. Maintain a boot matrix beginning with QEMU x86_64, then expanding to other supported targets.
   - *Minimum daily smoke test*: Clean build -> QEMU boot -> Serial log captured -> Shell prompt reached -> Basic command executes -> Graceful shutdown or controlled reset -> Artifact hash and QEMU version recorded.

4. **Capability Auditor**
   - *Purpose*: Prevent “aspirational completion.”
   - *Repeated Tasks*: Cross-check README claims, `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, source code, tests, docs, and runtime reports. Flag any claim of “implemented,” “working,” or “production-ready” that lacks test, integration, or boot evidence. Create narrowly scoped issues for unsupported claims. Maintain a public evidence table.
   - *Required evidence table format*:
     | Claim | Evidence type | Evidence link | Last verified | Status |
     | :--- | :--- | :--- | :--- | :--- |
     | Kernel scheduler works | Unit test + trace | `reports/test/...` | Date | `model-only` |
     | Filesystem mount works | QEMU runtime log | `reports/boot/...` | Date | `qemu-verified` |
     | Installer recovery works | Installer test + rollback log | `reports/installer/...` | Date | `integration-pending` |

5. **Kernel and Subsystem Agents**
   - *Scheduler*: Scheduling correctness, starvation, deterministic behavior (Unit tests plus scheduler trace).
   - *Memory*: Allocator invariants, leaks, fragmentation (Fuzz tests and allocation metrics).
   - *VFS/filesystem*: Path resolution, mount semantics, file operations (Filesystem conformance suite).
   - *Syscall*: ABI consistency, argument validation, error codes (Syscall table diff and tests).
   - *IPC*: Pipes, signals, shared memory, message queues (Cross-process interoperability tests).
   - *Drivers*: Probe, initialization, error handling (QEMU device logs).
   - *Security*: Capability checks, sandbox boundaries, unsafe code (Threat model and denial tests).
   - *Rule*: A subsystem agent may not modify another subsystem’s public interface without an integration review and updated tests.

6. **Package and Distro Compatibility Agent**
   - *Purpose*: Maintain schemas and translation rules for package metadata.
   - *Repeated Tasks*: Test parsing against a fixed corpus of `.deb`, `.rpm`, Arch package, and BSD package manifests. Track dependency-resolution regressions. Maintain a compatibility matrix: format, metadata parse, dependency graph, dry-run install, scriptlet policy, and security review.
   - *Rule*: Package scripts and downloaded binaries must never execute with broad host access. Use an isolated sandbox with explicitly declared filesystem, process, and network capabilities.

7. **Documentation and Onboarding Agent**
   - *Purpose*: Make SigmaOS usable by contributors.
   - *Repeated Tasks*: Verify that README commands match Makefiles and scripts. Generate a “What works today” page from verified capability data. Convert repeated failures into troubleshooting entries. Maintain a contributor quickstart. Flag documentation that promises unverified features.

---

## 9-Step Standard Task Lifecycle
```text
1. SELECT        Read agent ownership file, capability matrix, and task queue.
2. PLAN          Identify affected paths, tests, risks, and rollback method.
3. ISOLATE       Create a branch, workspace, or container.
4. EXECUTE       Make the smallest possible change.
5. VERIFY        Run scoped tests, repository checks, and runtime tests where applicable.
6. RECORD        Write logs, metrics, hashes, and a structured report.
7. UPDATE STATUS Change capability status only if evidence supports it.
8. REVIEW        Submit PR; human review is mandatory for kernel, security, installer, boot, and package-execution changes.
9. FOLLOW UP     If verification fails, create a blocked task with exact failing stage, log excerpt, and reproduction command.
```

---

## Contract Repository Layout
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

## Task File Format
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

## 10 Mandatory Operating Rules
1. **Evidence before status**: No claim of "implemented" or "working" without attached passing test/boot output.
2. **One agent, one boundary**: Modify only declared paths in ownership file.
3. **Small changes only**: PR should change one concern.
4. **Reproducibility is mandatory**: Every report must include exact commands, commit hash, toolchain versions, OS, timestamps, exit codes, hashes, log excerpts.
5. **Human approval for high-impact actions**: Mandatory for security boundaries, bootloaders/installers, package script execution, third-party code download/execution, channel changes, subsystem replacement/deletion, force pushes.
6. **Sandbox untrusted execution**: External repos, packages, scripts, models, and binaries are untrusted by default.
7. **No silent failure**: Blocked task created on failure with task ID, failing step, error message, log excerpt, suspected cause, unblock requirement.
8. **Claims must be versioned**: Include last-verified date and evidence link. Older claims downgrade automatically.
9. **Security changes get separate review**: Capability enforcement, sandboxing, memory safety, privilege boundaries, crypto, package execution, network access.
10. **Prefer runtime proof over model proof**: Runtime boot/integration result controls public status over unit test logic.

---

## Distro Release Channels
| Channel | Meaning | Requirements |
| :--- | :--- | :--- |
| `dev` | Latest agent-generated branch artifacts | Build and unit tests pass |
| `nightly` | Automated daily artifact | Build, unit tests, and QEMU smoke test pass |
| `validated` | Recommended test release | Full boot matrix, installer test, known-issue list, rollback instructions |
| `experimental` | New subsystem or compatibility feature | Clearly labeled, not suitable for daily use |

---

## 90-Day Plan
- **Days 1–14**: Establish ground truth (Agent registry, task schema, report dirs, evidence policy, full claim audit, first "Verified Today" report).
- **Days 15–35**: Stabilize build and tests (Automate clean builds, flaky-test detection, verification reports on PRs).
- **Days 36–60**: Prove one boot path (QEMU x86_64 boot-to-shell, persistent boot matrix, one subsystem agent per blocker).
- **Days 61–75**: Connect subsystems to runtime (Minimum subsystems for boot/shell/FS/process, replacement of model claims with runtime proof).
- **Days 76–90**: Add release discipline (Channels, nightly artifacts, rollback notes, metadata package compatibility).

---

## First 10 Initial Backlog Tasks
1. `REG-001`: Create `.agents/registry.toml` defining agents and scopes (`repository-steward`, P0, once).
2. `POL-001`: Create `.agents/policies/test-evidence.md` policy (`capability-auditor`, P0, once).
3. `CLI-001`: Build `tools/agent_cli.py` task runner (`build-test-sentinel`, P0, once).
4. `CI-001`: Add CI workflow for documented check/test commands (`build-test-sentinel`, P0, once).
5. `BOOT-X86-001`: Implement QEMU x86_64 boot-to-shell validator (`boot-runtime-validator`, P0, daily).
6. `AUDIT-001`: Audit every working claim in capability files (`capability-auditor`, P0, weekly).
7. `DASH-001`: Generate Verified Today dashboard from test/boot evidence (`documentation-onboarding-agent`, P1, daily).
8. `TASK-001`: Convert five largest roadmap docs into task files (`repository-steward`, P1, once).
9. `PKG-001`: Add package metadata compatibility test corpus and dry-run policy (`package-distro-compatibility-agent`, P1, weekly).
10. `NIGHTLY-001`: Define first nightly artifact requirements (`boot-runtime-validator`, P0, daily).
