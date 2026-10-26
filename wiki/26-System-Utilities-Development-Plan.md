# 26. System Utilities Development Plan

## Overview
SigmaOS's README and feature inventory describe an experimental operating system codebase rather than a supported general-purpose distribution. Currently, the userspace shell is a prototype supporting basic `echo`, `pwd`, and `ls` built-ins while process execution is developed; the hosted system-report tool lacks direct SigmaOS runtime provider integration; and `init` requires process spawning and service-dependency enforcement.

To transition SigmaOS toward a reliable userspace, this development plan prioritizes a small set of real, composable, and useful system utilities backed by working kernel and subsystem interfaces. Rather than claiming a large catalog of unbacked command names as an implemented userland, each utility is brought through explicit maturity stages supported by repeatable runtime evidence.

## Baseline Architecture Rules

1. **Explicit Utility Contracts**: Every system command must define a strict contract covering synopsis, options, operands, `stdin`/`stdout`/`stderr` routing, exit status codes, environment dependencies, and compatibility profile (POSIX baseline with explicit GNU/Linux/BSD extension markers).
2. **Composability and Stream Design**: Normal results are written to `stdout`; error diagnostics and progress indicators are written exclusively to `stderr`. Stream-oriented utilities read from `stdin` when no file operands are supplied or when `-` is specified. Machine-readable output is never mixed with human-oriented formatted text.
3. **Structured JSON Interface**: Structured JSON output is provided as a distinct option (`--format json`) for automation. JSON output follows a versioned schema and is tested independently from human-readable text output.
4. **Separation of Inspection and Mutation**: Status and diagnostic commands are strictly read-only and free of side effects. Mutating or destructive operations must clearly identify their target and incorporate safety mechanisms such as dry-run preview (`--dry-run`), explicit interactive confirmation, or rollback capability.
5. **Truthful Backend Availability**: Utilities must never report false success when an underlying subsystem provider is missing. System utility returns must explicitly distinguish between `unsupported` (lack of kernel capability), `not integrated` (missing userspace provider), `permission denied` (authorization failure), and `operation failed` (runtime execution error).
6. **Tiered Runtime Evidence**: Testing guidance distinguishes between internal white-box unit/model tests (KUnit-style) and userspace interface integration tests (kselftest-style). Testing evidence is categorized across four tiers:
   - `Unit/Model`: In-memory data structure and parser verification.
   - `Hosted`: Command execution against host OS fallback providers.
   - `QEMU Booted`: Command execution against native SigmaOS kernel interfaces on QEMU.
   - `Physical Target`: Execution on actual bare-metal hardware.
7. **Bounded and Idempotent Periodic Runner**: System maintenance tasks (inspired by FreeBSD's periodic framework) are idempotent, strictly bounded by execution time and output size limits, and log structured results (`pass`, `finding`, `unavailable`, `error`). Tasks do not execute destructive repairs by default.
8. **Protected Branch and PR Review Gates**: All utility changes require focused PRs, passing status checks, and resolved discussions on protected branches. Privileged or security-critical tools require secondary subsystem review.

---

## Proposed 18-PR Sequence Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **1** | `docs(utils): audit current tools and define compatibility policy` | Inventory utility code, shell built-ins, hosted tools, syscall/VFS dependencies, tests, and documentation. Define POSIX scope, naming, maturity vocabulary, and runtime evidence requirements. | Each existing command is labeled by implementation and test level; hosted tools are not described as SigmaOS runtime tools; compatibility claims have explicit criteria. |
| **2** | `docs(utils): add command registry and ownership map` | Add a registry of command name, purpose, owner, privilege, read/write behavior, backend, docs page, and tests. | No duplicate names or undocumented privileged commands; registry can drive generated help/docs where appropriate. |
| **3** | `feat(utils): add shared CLI contract helpers` | Add reusable Rust support for argument parsing, usage text, errors, exit status, stdout/stderr, TTY/color detection, and optional structured output. | Golden tests cover valid/invalid args, `--`, non-TTY use, stdout/stderr separation, and documented exit statuses. |
| **4** | `feat(utils): implement minimal file and directory utilities` | Add or integrate `cat`, `pwd`, `ls`, `stat`, `mkdir`, and `touch` against the real VFS and permission model. | QEMU/user-process tests exercise actual VFS calls; invalid paths, permissions, unusual filenames, and partial errors are covered. |
| **5** | `feat(utils): implement safe file mutation utilities` | Add `cp`, `mv`, `rm`, `rmdir`, and `ln` in small PRs; define overwrite, recursive, and symlink behavior. | Tests cover symlink/path traversal, same-file operations, overwrite behavior, permission failures, and interrupted operations; recursive deletion cannot escape the chosen target. |
| **6** | `feat(utils): add stream and text-processing utilities` | Add `head`, `tail`, `wc`, `cut`, `sort`, `uniq`, and `tee` incrementally. | Tools handle stdin, empty input, multiple operands, large streams, and defined text/binary behavior without unbounded buffering. |
| **7** | `feat(utils): add system identity and environment queries` | Add `env`, `printenv`, `id`, `whoami`, `uname`, and `hostname` only when backed by real process/account/kernel APIs. | Results come from SigmaOS providers; permissions and missing providers are handled explicitly; machine output schemas are stable if added. |
| **8** | `feat(utils): add process inspection and signal tools` | Add `ps`, `kill`, `sleep`, and `timeout` after process enumeration, signal delivery, and process-group semantics work. | Commands operate on real processes; target identity and authorization are checked; races and exit behavior are documented and tested. |
| **9** | `feat(utils): add storage and filesystem status tools` | Add `df`, `du`, mount inspection, `sync`, and device status once block/VFS/mount interfaces are integrated. Keep formatting and partition changes separate. | Read-only inspection never writes to devices; device identity is clear; errors and unavailable storage are distinguishable. |
| **10** | `feat(utils): add account and permission administration tools` | Add permission/account management only after identity storage, kernel authorization, and audit logging are implemented. | Permission checks occur in the authoritative kernel/service API; partial failures are reported; changes are auditable and tested. |
| **11** | `feat(utils): make system reports SigmaOS-runtime aware` | Extend the existing report tool with actual SigmaOS providers for kernel, hardware, filesystems, network, and services as they become available. | Each field identifies its source/freshness; host metrics are never represented as SigmaOS state; missing providers report unavailable. |
| **12** | `feat(periodic): add bounded maintenance-task runner` | Implement daily/weekly/monthly/security task groups, configuration, timeouts, output routing, and enable/disable policy. | Tasks are attributable and bounded; one task failure does not suppress others; logs are controlled; tasks do not perform destructive repairs by default. |
| **13** | `feat(periodic): add initial read-only health checks` | Add checks for failed services, disk capacity, update status, permission anomalies, and security events when those providers exist. | Every check reports pass/finding/unavailable/error; checks are read-only, rerunnable, and supported by fixtures or runtime tests. |
| **14** | `docs(utils): add manual pages and executable examples` | Document every command and periodic check using consistent manual sections; add examples exercised by tests where practical. | Manuals document syntax, options, exit status, examples, compatibility, and limitations; rendered output is validated. |
| **15** | `test(utils): add command contract and booted-system suite` | Add unit, golden-output, VFS, process, and QEMU integration tests; classify test evidence. | CI separates model tests from booted runtime tests; core utilities run against SigmaOS interfaces on a named QEMU target. |
| **16** | `security(utils): audit path, privilege, and data-handling risks` | Review path/symlink handling, numeric parsing, permissions, shell invocation, logs, and destructive operations. | No privileged command is built from shell strings; untrusted paths and inputs have regression tests; secrets/private data are redacted. |
| **17** | `ci(utils): validate registry, help, manuals, and status matrix` | Check installed command list, registry, help output, manual pages, feature status, and test coverage for drift. | CI fails on undocumented commands, stale claims, missing manuals, or parser/docs mismatches. |
| **18** | `release(utils): publish utility support matrix` | List commands, versions, compatibility profile, privilege, backend, architecture, and known gaps. | “Supported” tools have repeatable runtime evidence; unsupported behavior is explicitly listed. |

### Initial High-Value Sequence
The highest-value initial sequence consists of **PRs 1–4, 11, and 15**: establishing truthful status, reusable CLI contracts, basic VFS utilities, runtime-backed diagnostics, and the QEMU integration test path. Process, storage, and account tools are deferred until their underlying kernel and service interfaces are fully integrated.

---

## Periodic Maintenance PR Schedule

| Cadence | Proposed PR/Task | Acceptance Evidence |
|---|---|---|
| **Every PR** | `ci(utils): run utility contract checks` | Build, unit/golden tests, docs checks, and affected QEMU tests pass; skipped checks explained. |
| **Weekly** | `maint(utils): triage failures and stale status claims` | Reproducer, affected command/backend, owner, priority, and fix or documented deferral recorded; status updated from evidence. |
| **Weekly** | `docs(utils): reconcile registry, help, and manuals` | Command names, options, examples, and status agree across code, registry, help text, and manual pages. |
| **Monthly** | `test(utils): run core command behavior matrix` | Verify stdin, multiple operands, `--`, exit status codes, stderr routing, permission handling, long filenames, and non-TTY execution. |
| **Monthly** | `security(utils): review privileged and destructive utilities` | Review `rm`, storage, account, mount, and service operations for authorization, target validation, confirmation, and data redaction. |
| **Monthly** | `maint(periodic): review maintenance-task results` | Runtime duration, failure rates, scheduling bounds, and log retention assessed; unavailable checks are not counted as passed. |
| **Quarterly** | `docs(utils): review POSIX and Linux/BSD deviations` | Revalidate option syntax against POSIX standards; document platform-specific extensions explicitly. |
| **Quarterly** | `test(utils): run clean-boot and upgrade smoke tests` | Core utilities and periodic tasks run cleanly after fresh QEMU boot and supported upgrade paths. |
| **Release Candidate** | `release(utils): certify shipped tools and maintenance checks` | Supported command matrix, manual pages, permissions, boot target, periodic results, and open limitations recorded. |
| **Post-Incident** | `fix(utils): prevent recurrence of <failure>` | Root cause analysis, automated regression reproducer, security impact assessment, and backport decisions documented. |

---

## Subsystem Component Rules

- **File Utilities**: Treat path strings as binary data rather than shell syntax. Enforce explicit symlink-following (`-L` vs `-P`), overwrite protection (`-i`), recursion limits, and path traversal guards. Preserve metadata only when explicitly requested.
- **Text Utilities**: Stream input through fixed-size buffers to handle large streams without unbounded memory growth. Define clear byte-versus-character handling, locale defaults, and invalid UTF-8 encoding behavior.
- **Process Utilities**: Rely strictly on authoritative kernel process table APIs. Enforce authorization checks for signals and process queries. Ensure state is fetched live rather than served from stale caches or mock models.
- **Storage Utilities**: Strictly separate read-only status reporting from storage mutation. Display clear device identifiers and partition bounds. Require explicit target validation and dry-run/preview flags for destructive operations.
- **System Reporting**: Annotate every metric field with its data source (`live`, `cached`, `host-derived`, or `unavailable`). Bound output buffers and automatically redact sensitive credentials or private tokens.
- **Maintenance Runner**: Ensure tasks are idempotent and read-only by default. Apply time and memory bounds per task. Record execution metrics (`task_id`, `start_time`, `end_time`, `result_category`, `summary`) and honor administrator policy overrides.
- **Documentation**: Use standard manual page sections (`NAME`, `SYNOPSIS`, `DESCRIPTION`, `OPTIONS`, `OPERANDS`, `EXIT STATUS`, `ENVIRONMENT`, `EXAMPLES`, `STANDARDS`). Keep manual pages, `--help` output, and registry metadata in sync.
- **Testing**: Maintain clear separation between internal KUnit-style white-box unit tests and kselftest-style userspace interface integration tests. Require QEMU runtime evidence before marking utilities as `Integrated` or `Supported`.

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

---

## Completion & Feature Status Gate

A system utility is marked according to strict milestone criteria:
- **Proposed**: Architecture, contract, and registry entry defined.
- **Prototype**: Implemented as in-memory model or hosted tool.
- **Integrated**: Operates against actual native SigmaOS kernel/VFS interfaces on a documented QEMU boot target.
- **Supported**: Verified with repeatable automated tests, manual pages, permission checks, failure handling, and zero drift between implementation and documentation.
