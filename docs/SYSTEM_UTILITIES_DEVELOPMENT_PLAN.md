# SigmaOS System Utilities Master Architecture & Development Plan

## Executive Summary & Vision

SigmaOS's current codebase represents an experimental OS framework under active development rather than a general-purpose, fully integrated distribution. An audit of the existing feature inventory reveals that:
- The system shell is a prototype featuring built-in commands (`echo`, `pwd`, `ls`) but still requiring full userland process execution integration.
- The hosted system-report tool currently operates without live SigmaOS runtime integration (relying on host metrics or synthetic models).
- The initialization subsystem (`sigma_init`) requires process spawning and service-dependency enforcement.

To transition SigmaOS from an experimental prototype to a robust, production-grade operating system, this plan establishes a grounded development sequence. Rather than treating a long list of command names as an implemented userland, the plan prioritizes a core set of real, useful utilities backed by working OS kernel and VFS interfaces.

POSIX utility conventions serve as our baseline for command syntax and option handling, while explicit extensions draw inspiration from GNU Coreutils, Linux, and FreeBSD.

---

## Baseline Rules & Architecture Guidelines

### 1. Command Contract Definition
Every utility introduced to SigmaOS must formally document its contract:
- **Synopsis & Options**: Follow POSIX option and operand syntax standards (e.g., short options `-a`, long options `--all`, option-arguments, and operand parameters).
- **Streams**: Composable design—normal results emitted strictly to `stdout`; error messages and diagnostics directed to `stderr`.
- **Exit Status**: Deterministic exit codes (`0` for success, `1` for general failure, `2` for syntax/argument error, `126` for command non-executable, `127` for command not found).
- **Environment**: Explicit list of required or consulted environment variables (e.g., `PATH`, `HOME`, `TERM`, `LANG`).
- **Compatibility Profile**: Explicitly tag POSIX, GNU, BSD, or SigmaOS-native behaviors.

### 2. Utility Composability
- Never mix human-formatted decorative text into machine-readable streams.
- Support standard input (`stdin`) streaming for all text processing and transformation utilities.
- Maintain consistent output behavior regardless of stream redirection or pipe chaining.

### 3. Separate JSON Automation Interface
- Provide structured JSON output via explicit `--json` flags where automation or API consumption is required.
- Version all JSON schemas independently from human-readable terminal text.
- Maintain independent test suites for human output vs. JSON schemas.

### 4. Separation of Inspection from Mutation
- Status, query, and reporting commands must be strictly read-only and free of side-effects.
- Destructive mutation utilities (e.g., `rm`, `mv`, `fdisk`, account modifications) must explicitly identify target paths/resources and provide safeguards (dry-run `--preview`, interactive confirmation `-i`, or rollback/recovery mechanisms).

### 5. Backend Status Transparency
- Never report false success when a underlying kernel feature or service backend is unavailable.
- Clearly distinguish between:
  1. `UNSUPPORTED`: Functionality not defined or planned for current architecture.
  2. `NOT_INTEGRATED`: Backend driver/kernel subsystem exists but userland bridge is missing.
  3. `PERMISSION_DENIED`: Action blocked by kernel authorization or privilege policies.
  4. `OPERATION_FAILED`: Execution failed in the backend during runtime.

### 6. Testing Runtime Behavior Classification
Test evidence must explicitly classify execution context into four distinct tiers:
- **Unit/Model**: Pure Rust unit tests exercising logic in isolation without kernel state.
- **Hosted**: Executed within host OS environment (e.g., cargo test on Linux developer workstation).
- **QEMU Booted SigmaOS**: Verified inside a booted QEMU virtual machine running the actual SigmaOS kernel and VFS.
- **Physical Target**: Executed on real bare-metal hardware.

Following Linux kernel testing guidelines, internal white-box unit tests (akin to KUnit) are tracked separately from end-to-end userspace selftests (akin to `kselftest`).

### 7. Bounded Periodic Maintenance Framework
- Periodic maintenance tasks (daily, weekly, monthly) must be idempotent and read-only by default.
- Every check must enforce strict execution timeout and memory bounds.
- Status reports must classify outcomes into four categories: `PASS`, `FINDING`, `UNAVAILABLE`, or `ERROR`.
- Modeled after FreeBSD's periodic framework (`/etc/periodic`).

### 8. Focused PRs & Protected Branches
- Every pull request must be tightly scoped to a single logical utility or maintenance capability.
- GitHub protected branch rules enforce mandatory maintainer reviews, automated CI status checks, and resolved conversation threads before merging.

---

## Proposed PR Roadmap

The development sequence is divided into 18 focused Pull Requests.

| PR | Proposed Title | Scope | Acceptance Criteria |
| :--- | :--- | :--- | :--- |
| **PR 1** | `docs(utils): audit current tools and define compatibility policy` | Inventory utility code, shell built-ins, hosted tools, syscall/VFS dependencies, tests, and documentation. Define POSIX scope, naming, maturity vocabulary, and runtime evidence requirements. | Each existing command is labeled by implementation and test level; hosted tools are not described as SigmaOS runtime tools; compatibility claims have explicit criteria. |
| **PR 2** | `docs(utils): add command registry and ownership map` | Add a registry of command name, purpose, owner, privilege, read/write behavior, backend, docs page, and tests. | No duplicate names or undocumented privileged commands; registry can drive generated help/docs where appropriate. |
| **PR 3** | `feat(utils): add shared CLI contract helpers` | Add reusable Rust support for argument parsing, usage text, errors, exit status, stdout/stderr, TTY/color detection, and optional structured output. | Golden tests cover valid/invalid args, `--`, non-TTY use, stdout/stderr separation, and documented exit statuses. |
| **PR 4** | `feat(utils): implement minimal file and directory utilities` | Add or integrate `cat`, `pwd`, `ls`, `stat`, `mkdir`, and `touch` against the real VFS and permission model. | QEMU/user-process tests exercise actual VFS calls; invalid paths, permissions, unusual filenames, and partial errors are covered. |
| **PR 5** | `feat(utils): implement safe file mutation utilities` | Add `cp`, `mv`, `rm`, `rmdir`, and `ln` in small PRs; define overwrite, recursive, and symlink behavior. | Tests cover symlink/path traversal, same-file operations, overwrite behavior, permission failures, and interrupted operations; recursive deletion cannot escape target. |
| **PR 6** | `feat(utils): add stream and text-processing utilities` | Add `head`, `tail`, `wc`, `cut`, `sort`, `uniq`, and `tee` incrementally. | Tools handle stdin, empty input, multiple operands, large streams, and defined text/binary behavior without unbounded buffering. |
| **PR 7** | `feat(utils): add system identity and environment queries` | Add `env`, `printenv`, `id`, `whoami`, `uname`, and `hostname` only when backed by real process/account/kernel APIs. | Results come from SigmaOS providers; permissions and missing providers are handled explicitly; machine output schemas are stable if added. |
| **PR 8** | `feat(utils): add process inspection and signal tools` | Add `ps`, `kill`, `sleep`, and `timeout` after process enumeration, signal delivery, and process-group semantics work. | Commands operate on real processes; target identity and authorization are checked; races and exit behavior are documented and tested. |
| **PR 9** | `feat(utils): add storage and filesystem status tools` | Add `df`, `du`, mount inspection, `sync`, and device status once block/VFS/mount interfaces are integrated. Keep formatting/partitioning separate. | Read-only inspection never writes to devices; device identity is clear; errors and unavailable storage are distinguishable. |
| **PR 10** | `feat(utils): add account and permission administration tools` | Add permission/account management only after identity storage, kernel authorization, and audit logging are implemented. | Permission checks occur in authoritative kernel/service API; partial failures are reported; changes are auditable and tested. |
| **PR 11** | `feat(utils): make system reports SigmaOS-runtime aware` | Extend the existing report tool with actual SigmaOS providers for kernel, hardware, filesystems, network, and services as available. | Each field identifies its source/freshness; host metrics are never represented as SigmaOS state; missing providers report unavailable. |
| **PR 12** | `feat(periodic): add bounded maintenance-task runner` | Implement daily/weekly/monthly/security task groups, configuration, timeouts, output routing, and enable/disable policy. | Tasks are attributable and bounded; one task failure does not suppress others; logs are controlled; tasks do not perform destructive repairs by default. |
| **PR 13** | `feat(periodic): add initial read-only health checks` | Add checks for failed services, disk capacity, update status, permission anomalies, and security events when providers exist. | Every check reports pass/finding/unavailable/error; checks are read-only, rerunnable, and supported by fixtures or runtime tests. |
| **PR 14** | `docs(utils): add manual pages and executable examples` | Document every command and periodic check using consistent manual sections; add examples exercised by tests where practical. | Manuals document syntax, options, exit status, examples, compatibility, and limitations; rendered output is validated. |
| **PR 15** | `test(utils): add command contract and booted-system suite` | Add unit, golden-output, VFS, process, and QEMU integration tests; classify test evidence. | CI separates model tests from booted runtime tests; core utilities run against SigmaOS interfaces on a named QEMU target. |
| **PR 16** | `security(utils): audit path, privilege, and data-handling risks` | Review path/symlink handling, numeric parsing, permissions, shell invocation, logs, and destructive operations. | No privileged command is built from shell strings; untrusted paths/inputs have regression tests; secrets/private data are redacted. |
| **PR 17** | `ci(utils): validate registry, help, manuals, and status matrix` | Check installed command list, registry, help output, manual pages, feature status, and test coverage for drift. | CI fails on undocumented commands, stale claims, missing manuals, or parser/docs mismatches. |
| **PR 18** | `release(utils): publish utility support matrix` | List commands, versions, compatibility profile, privilege, backend, architecture, and known gaps. | "Supported" tools have repeatable runtime evidence; unsupported behavior is explicitly listed. |

### Highest-Value Initial Sequence
The immediate implementation priority consists of **PRs 1–4, 11, and 15**:
1. Establish truthful status claims and baseline compatibility policies (PR 1).
2. Establish the command registry and ownership mapping (PR 2).
3. Build robust shared CLI argument and streaming contract helpers (PR 3).
4. Deliver minimal file/directory utilities (`cat`, `pwd`, `ls`, `stat`, `mkdir`, `touch`) tied directly to the SigmaOS VFS (PR 4).
5. Convert system reporting tools to consume true SigmaOS kernel providers (PR 11).
6. Build the booted QEMU integration test framework to verify contract compliance (PR 15).

Process, storage, and account administration utilities are deferred until their respective kernel syscalls and daemon interfaces reach operational maturity.

---

## Periodic Maintenance Framework

| Cadence | Task / Title | Acceptance Evidence |
| :--- | :--- | :--- |
| **Every PR** | `ci(utils): run utility contract checks` | Build, unit/golden tests, docs checks, and affected QEMU tests; skipped checks explained. |
| **Weekly** | `maint(utils): triage failures and stale status claims` | Reproducer, affected command/backend, owner, priority, and fix or documented deferral; feature status updated only from evidence. |
| **Weekly** | `docs(utils): reconcile registry, help, and manuals` | Command names, options, examples, and status agree across code, registry, help, and docs. |
| **Monthly** | `test(utils): run core command behavior matrix` | Verify stdin, multiple operands, `--`, exit status, stderr, permission errors, long/unusual filenames, and non-TTY use. |
| **Monthly** | `security(utils): review privileged and destructive utilities` | Review `rm`, storage, account, mount, service, and update operations for authorization, target validation, confirmation, rollback, and data exposure. |
| **Monthly** | `maint(periodic): review maintenance-task results and retention` | Runtime, failure rate, duplicate scheduling, time/output bounds, and log retention assessed; unavailable checks are not counted as passed. |
| **Quarterly** | `docs(utils): review POSIX and Linux/BSD deviations` | Revalidate declared option syntax and semantics against POSIX; document platform-specific behavior. |
| **Quarterly** | `test(utils): run clean-boot and upgrade smoke tests` | Core commands and periodic tasks run after fresh QEMU boot and supported upgrade path; configuration and logs retained. |
| **Release Candidate** | `release(utils): certify shipped tools and maintenance checks` | Supported command matrix, manual pages, permissions, boot target, periodic results, and open limitations recorded. |
| **Post-Incident** | `fix(utils): prevent recurrence of <failure>` | Root cause, test reproducer, data-loss/security assessment, and backport decision included. |

---

## Component Rules

1. **File Utilities**: Treat filenames strictly as binary/UTF-8 data, not shell syntax. Define explicit symlink-following, overwrite, recursion, and partial-I/O semantics. Preserve file metadata only when explicitly requested (`-p`/`--preserve`).
2. **Text Utilities**: Stream large inputs efficiently without unbounded memory allocation. Define exact byte-vs-text processing, encoding handling, and EOF behaviors.
3. **Process Utilities**: Interact directly with kernel process tables via standard syscalls or `/proc` equivalents. Authorize signal sending and process inspection. Make process state races predictable and never report mock state as live system state.
4. **Storage Utilities**: Strictly isolate read-only inspection from write operations. Display stable device identifiers (UUID, PARTUUID, dev path). Require interactive confirmation, dry-run previews, or forced flags for destructive mutations.
5. **System Reporting**: Mark every reported metric as `LIVE`, `CACHED`, `HOST_DERIVED`, or `UNAVAILABLE`. Enforce strict buffer limits and automatically redact user credentials, keys, or sensitive memory contents.
6. **Maintenance Runner**: Ensure all task executions are idempotent and non-destructive by default. Enforce configurable CPU/memory timeouts. Log task execution metadata (task ID, duration, status code, concise log output).
7. **Documentation**: Maintain standard manual page structure: `NAME`, `SYNOPSIS`, `DESCRIPTION`, `OPTIONS`, `OPERANDS`, `EXIT STATUS`, `ENVIRONMENT`, `EXAMPLES`, `COMPATIBILITY`, `SEE ALSO`.
8. **Testing**: Maintain clean separation between internal white-box unit tests (KUnit model) and end-to-end userspace contract tests (kselftest model).

---

## PR Review Template and Merge Gates

```markdown
## Summary
What utility or maintenance task does this PR add or fix?

## Scope
Command/task, backend, privilege, and excluded work.

## Command Contract
Synopsis, options, operands, stdin/stdout/stderr, exit status,
environment, output format, and compatibility profile.

## Runtime Evidence
Unit/model | hosted | QEMU booted SigmaOS | physical target
State the exact target and data source.

## Safety and Side Effects
Read-only or mutating; input/path validation, authorization, preview,
confirmation, rollback/recovery, and data redaction.

## Tests and Docs
Commands run, expected output/status, manual page, periodic-task behavior,
and anything not tested.

## Status
Proposed | Prototype | Integrated | Supported
Explain the evidence behind the status.

## Checklist
- [ ] Focused PR with measurable acceptance criteria
- [ ] Normal, invalid-input, permission, and failure cases tested
- [ ] Help, exit status, stderr, and manual match implementation
- [ ] Unavailable backend never reports false success
- [ ] Destructive behavior is explicit and recoverable
- [ ] Feature inventory updated with test evidence
- [ ] Required CI passes; review conversations resolved
```

### Merge Gates
- Mandatory maintainer review for all PRs.
- Additional security or subsystem reviewer required for privileged, destructive, or credential-handling utilities.
- Branch protection enforcing required status checks, conversation resolution, and linear git history (no force pushes).

---

## Completion Gates & Subsystem Status Definition

- **Prototype**: Syntax parser or in-memory model exists; unit tests pass in hosted environment.
- **Integrated**: Utility executes against the real SigmaOS kernel, VFS, or daemon interfaces on a documented boot target (e.g. QEMU x86_64).
- **Supported**: Utility has repeatable runtime test evidence across normal/failure paths, complete manual pages, verified permission checks, and clean error handling for unavailable backends.

Code presence or hosted unit tests alone do NOT qualify a utility as `Supported`.
