# SigmaOS System Utilities Master Architecture, Support Matrix, and PR Plan

SigmaOS describes itself as an experimental OS codebase, not a supported general-purpose installation. Its feature inventory says the shell is a prototype with only `echo`, `pwd`, and `ls` built-ins and still needs process execution; the hosted system-report tool has no SigmaOS runtime integration; and init still needs process spawning and service-dependency enforcement. So the plan starts with a small set of real, useful utilities backed by working OS interfaces, rather than treating a large list of command names as an implemented userland.

This plan uses POSIX utility conventions for common command behavior as a baseline, while clearly marking GNU/Linux/BSD-inspired extensions.

---

## Baseline Rules

1. **Command Contracts**: Define each command’s contract: synopsis, options, operands, stdin/stdout/stderr, exit status, environment, and compatibility profile. POSIX’s utility conventions provide a baseline for option and operand syntax.
2. **Composability & Streams**: Keep normal results on stdout and diagnostics on stderr; support stdin for stream-oriented tools; use stable exit statuses; never mix human text into machine-readable output.
3. **Structured Machine Interfaces**: Treat JSON as a separate interface: provide it where useful for automation, version its schema, and test it independently from the human-readable output.
4. **Inspection vs Mutation**: Separate inspection from mutation: make status/report commands read-only; destructive operations must identify their target and provide safeguards such as preview, confirmation, or recovery.
5. **Truthful Error Reporting**: Never claim success for an unavailable backend: distinguish “unsupported,” “not integrated,” “permission denied,” and “operation failed.”
6. **Runtime Evidence Classification**: Test runtime behavior: label unit/model, hosted, QEMU booted SigmaOS, and physical-device evidence separately. Linux’s testing guidance distinguishes in-kernel white-box tests from userspace selftests that exercise actual interfaces.
7. **Safe and Bounded Maintenance**: Keep periodic checks safe and bounded: maintenance tasks should be idempotent where possible, have time/output limits, and report whether they passed, found an issue, or could not run. FreeBSD’s periodic framework is a useful reference for daily, weekly, monthly, and security-oriented tasks.
8. **Branch Protection & Review**: Use focused PRs and protected branches: require relevant review and passing checks; GitHub supports required pull-request reviews, status checks, and conversation resolution for protected branches.

---

## Proposed PR Sequence (18-PR Roadmap)

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(utils): audit current tools and define compatibility policy` | Inventory utility code, shell built-ins, hosted tools, syscall/VFS dependencies, tests, and documentation. Define POSIX scope, naming, maturity vocabulary, and runtime evidence requirements. | Each existing command is labeled by implementation and test level; hosted tools are not described as SigmaOS runtime tools; compatibility claims have explicit criteria. |
| 2 | `docs(utils): add command registry and ownership map` | Add a registry of command name, purpose, owner, privilege, read/write behavior, backend, docs page, and tests. | No duplicate names or undocumented privileged commands; registry can drive generated help/docs where appropriate. |
| 3 | `feat(utils): add shared CLI contract helpers` | Add reusable Rust support for argument parsing, usage text, errors, exit status, stdout/stderr, TTY/color detection, and optional structured output. | Golden tests cover valid/invalid args, `--`, non-TTY use, stdout/stderr separation, and documented exit statuses. |
| 4 | `feat(utils): implement minimal file and directory utilities` | Add or integrate `cat`, `pwd`, `ls`, `stat`, `mkdir`, and `touch` against the real VFS and permission model. | QEMU/user-process tests exercise actual VFS calls; invalid paths, permissions, unusual filenames, and partial errors are covered. |
| 5 | `feat(utils): implement safe file mutation utilities` | Add `cp`, `mv`, `rm`, `rmdir`, and `ln` in small PRs; define overwrite, recursive, and symlink behavior. | Tests cover symlink/path traversal, same-file operations, overwrite behavior, permission failures, and interrupted operations; recursive deletion cannot escape the chosen target. |
| 6 | `feat(utils): add stream and text-processing utilities` | Add `head`, `tail`, `wc`, `cut`, `sort`, `uniq`, and `tee` incrementally. | Tools handle stdin, empty input, multiple operands, large streams, and defined text/binary behavior without unbounded buffering. |
| 7 | `feat(utils): add system identity and environment queries` | Add `env`, `printenv`, `id`, `whoami`, `uname`, and `hostname` only when backed by real process/account/kernel APIs. | Results come from SigmaOS providers; permissions and missing providers are handled explicitly; machine output schemas are stable if added. |
| 8 | `feat(utils): add process inspection and signal tools` | Add `ps`, `kill`, `sleep`, and `timeout` after process enumeration, signal delivery, and process-group semantics work. | Commands operate on real processes; target identity and authorization are checked; races and exit behavior are documented and tested. |
| 9 | `feat(utils): add storage and filesystem status tools` | Add `df`, `du`, `mount` inspection, `sync`, and device status once block/VFS/mount interfaces are integrated. Keep formatting and partition changes separate. | Read-only inspection never writes to devices; device identity is clear; errors and unavailable storage are distinguishable. |
| 10 | `feat(utils): add account and permission administration tools` | Add permission/account management only after identity storage, kernel authorization, and audit logging are implemented. | Permission checks occur in the authoritative kernel/service API; partial failures are reported; changes are auditable and tested. |
| 11 | `feat(utils): make system reports SigmaOS-runtime aware` | Extend the existing report tool with actual SigmaOS providers for kernel, hardware, filesystems, network, and services as they become available. | Each field identifies its source/freshness; host metrics are never represented as SigmaOS state; missing providers report unavailable. The inventory currently labels this report hosted-only. |
| 12 | `feat(periodic): add bounded maintenance-task runner` | Implement daily/weekly/monthly/security task groups, configuration, timeouts, output routing, and enable/disable policy. | Tasks are attributable and bounded; one task failure does not suppress others; logs are controlled; tasks do not perform destructive repairs by default. |
| 13 | `feat(periodic): add initial read-only health checks` | Add checks for failed services, disk capacity, update status, permission anomalies, and security events when those providers exist. | Every check reports pass/finding/unavailable/error; checks are read-only, rerunnable, and supported by fixtures or runtime tests. |
| 14 | `docs(utils): add manual pages and executable examples` | Document every command and periodic check using consistent manual sections; add examples exercised by tests where practical. | Manuals document syntax, options, exit status, examples, compatibility, and limitations; rendered output is validated. FreeBSD’s documentation guide describes man pages as the standard reference for commands and configuration. |
| 15 | `test(utils): add command contract and booted-system suite` | Add unit, golden-output, VFS, process, and QEMU integration tests; classify test evidence. | CI separates model tests from booted runtime tests; core utilities run against SigmaOS interfaces on a named QEMU target. |
| 16 | `security(utils): audit path, privilege, and data-handling risks` | Review path/symlink handling, numeric parsing, permissions, shell invocation, logs, and destructive operations. | No privileged command is built from shell strings; untrusted paths and inputs have regression tests; secrets/private data are redacted. |
| 17 | `ci(utils): validate registry, help, manuals, and status matrix` | Check installed command list, registry, help output, manual pages, feature status, and test coverage for drift. | CI fails on undocumented commands, stale claims, missing manuals, or parser/docs mismatches. |
| 18 | `release(utils): publish utility support matrix` | List commands, versions, compatibility profile, privilege, backend, architecture, and known gaps. | “Supported” tools have repeatable runtime evidence; unsupported behavior is explicitly listed. |

### Initial High-Value Sequence

The highest-value initial sequence is **PRs 1–4, 11, and 15**: establish truthful status, consistent command behavior, basic VFS utilities, runtime-backed reports, and the test path. Defer process, storage, and account tools until their kernel and service interfaces are integrated. The repository’s own status file says process spawning, syscall dispatch, and several runtime paths remain milestones—not established general-purpose OS capabilities.

---

## Periodic Maintenance PRs

These are recurring review tasks, not automatic code changes. FreeBSD’s periodic model demonstrates daily, weekly, monthly, and security checks as distinct operational categories; SigmaOS should only enable checks whose data sources and safe behavior are implemented.

| Cadence | Proposed PR/Task | Acceptance Evidence |
|---|---|---|
| Every PR | `ci(utils): run utility contract checks` | Build, unit/golden tests, docs checks, and affected QEMU tests; skipped checks explained. |
| Weekly | `maint(utils): triage failures and stale status claims` | Reproducer, affected command/backend, owner, priority, and fix or documented deferral; feature status updated only from evidence. |
| Weekly | `docs(utils): reconcile registry, help, and manuals` | Command names, options, examples, and status agree across code, registry, help, and docs. |
| Monthly | `test(utils): run core command behavior matrix` | Verify stdin, multiple operands, `--`, exit status, stderr, permission errors, long/unusual filenames, and non-TTY use. |
| Monthly | `security(utils): review privileged and destructive utilities` | Review `rm`, storage, account, mount, service, and update operations for authorization, target validation, confirmation, rollback, and data exposure. |
| Monthly | `maint(periodic): review maintenance-task results and retention` | Runtime, failure rate, duplicate scheduling, time/output bounds, and log retention assessed; unavailable checks are not counted as passed. |
| Quarterly | `docs(utils): review POSIX and Linux/BSD deviations` | Revalidate declared option syntax and semantics against POSIX; document platform-specific behavior. |
| Quarterly | `test(utils): run clean-boot and upgrade smoke tests` | Core commands and periodic tasks run after fresh QEMU boot and supported upgrade path; configuration and logs retained. |
| Each release candidate | `release(utils): certify shipped tools and maintenance checks` | Supported command matrix, manual pages, permissions, boot target, periodic results, and open limitations recorded. |
| After each incident | `fix(utils): prevent recurrence of <failure>` | Root cause, test reproducer, data-loss/security assessment, and backport decision included. |

---

## Component Rules

- **File Utilities**: Treat names as data, not shell syntax; define symlink-following, overwrite, recursion, and partial-I/O semantics; preserve file metadata only when explicitly specified.
- **Text Utilities**: Stream large input; define byte-vs-text behavior, locale, and invalid encoding handling; avoid hidden buffering that can exhaust memory.
- **Process Utilities**: Use kernel process APIs; authorize signals and inspection; make stale IDs/races predictable; do not report process state from a model or cache as live state.
- **Storage Utilities**: Separate inspection from destructive operations; show stable device identity and exact target; require preview/confirmation/recovery for future writes.
- **System Reporting**: Mark each value live, cached, host-derived, or unavailable; bound output and redact credentials and private user data.
- **Maintenance Runner**: Make tasks idempotent and read-only by default; set time/resource limits; record task ID, start/end, result category, and concise output; allow administrator policy to disable tasks.
- **Documentation**: Use a consistent manual structure with synopsis, description, options, environment, exit status, examples, and standards/compatibility notes.
- **Testing**: Use unit tests for isolated logic and end-to-end tests for actual userspace interfaces. Linux distinguishes KUnit’s internal white-box tests from kselftest’s userspace/system-level tests; SigmaOS should preserve this distinction in reports.

---

## PR Review Template and Merge Gates

```markdown
## Summary
What utility or maintenance task does this PR add or fix?

## Scope
Command/task, backend, privilege, and excluded work.

## Command contract
Synopsis, options, operands, stdin/stdout/stderr, exit status,
environment, output format, and compatibility profile.

## Runtime evidence
Unit/model | hosted | QEMU booted SigmaOS | physical target
State the exact target and data source.

## Safety and side effects
Read-only or mutating; input/path validation, authorization, preview,
confirmation, rollback/recovery, and data redaction.

## Tests and docs
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

Require maintainer review and required checks for all changes; require an additional security or subsystem reviewer for privileged, destructive, or data-handling utilities. GitHub protected branches can enforce PR approvals, status checks, conversation resolution, and restrictions on force pushes.

---

## Completion Gate

Mark a utility **Integrated** only when it uses the real SigmaOS interface on a documented boot target.

Mark it **Supported** only after repeatable tests, documented behavior, permissions, failure paths, and manual pages all agree.

Periodic checks must additionally show safe reruns, bounded runtime/output, and explicit handling when their provider is unavailable. This matches the repo’s warning that code presence and unit tests alone do not prove a usable OS.
