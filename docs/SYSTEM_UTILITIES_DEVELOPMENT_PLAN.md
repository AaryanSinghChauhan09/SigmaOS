# SigmaOS System Utilities Development Plan

## Executive Summary & Context

SigmaOS is an experimental operating system codebase. Its feature inventory clarifies that userland components, shell interpreters, and hosted utilities require real OS interface integration rather than treating a theoretical list of command names as an established userland.

This development plan establishes a structured, phased roadmap for building POSIX-aligned, production-grade, safe, and composable system utilities backed by authoritative kernel and service interfaces. Rather than claiming superficial command availability, SigmaOS utilities explicitly distinguish between model unit tests, hosted execution, QEMU booted runtime, and physical hardware validation.

---

## Baseline Rules & Operating Principles

1. **Command Contract Specification**:
   - Every utility must define an explicit contract: Synopsis, Options, Operands, Stdin/Stdout/Stderr streams, Exit Status Codes, Environment Dependencies, and POSIX / Linux / BSD Compatibility Profile.
   - Syntax and option parsing adhere strictly to POSIX utility conventions (standard option syntax, option-arguments, and `--` operand delimiters).

2. **Composability & Stream Protocol**:
   - Keep normal, expected query results on `stdout` and diagnostic/error streams strictly on `stderr`.
   - Support `stdin` input streaming for stream-oriented tools.
   - Maintain stable, deterministic exit status codes (`0` for success, `1` for general failure, `2` for syntax error, `126` for permission denied, `127` for command not found).
   - Never mix human-formatted decorative text into machine-parsable outputs.

3. **Structured JSON Interfaces**:
   - Provide explicit `--json` output options where structured output benefits automation and scripting.
   - Version all JSON schemas independently.
   - Test JSON output streams independently from human-readable terminal output.

4. **Inspection vs. Mutation Separation**:
   - Inspection, diagnostic, and reporting commands must be strictly read-only with zero side effects.
   - Destructive operations (such as `rm`, `userdel`, formatting) must explicitly target resources and provide safeguards such as preview dry-runs, interactive confirmation, or atomic recovery/rollback.

5. **Honest Backend Status Reporting**:
   - Utilities must never report false success when a backend kernel or service API is missing.
   - Output must clearly distinguish between `unsupported`, `not integrated`, `permission denied`, and `operation failed`.

6. **Evidence-Gated Testing**:
   - Tests and feature status entries must explicitly differentiate between:
     - **Unit / Model**: Standalone logic tests without kernel bindings.
     - **Hosted**: Executed on host operating system wrappers.
     - **QEMU Booted SigmaOS**: Verified on a booted QEMU target using real syscalls/VFS.
     - **Physical Hardware**: Validated on physical target hardware.
   - Follow Linux kernel test guidance by keeping in-kernel unit/white-box tests distinct from userspace interface selftests.

7. **Bounded Periodic Maintenance**:
   - Periodic maintenance tasks must be safe, idempotent, time-bounded, and resource-capped.
   - Every maintenance check must report one of four explicit outcomes: `pass`, `finding`, `unavailable`, or `error`.
   - Inspired by FreeBSD's periodic framework (`daily`, `weekly`, `monthly`, `security`).

8. **Protected Workflows & Governance**:
   - Require focused pull requests with explicit acceptance criteria.
   - Protected branch rules enforce mandatory maintainer/security reviews, automated CI status checks, and resolved conversations before merging.

---

## Proposed 18-PR Roadmap Sequence

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

---

## Initial High-Value Sequence Focus

The highest-value initial sequence comprises **PRs 1–4, 11, and 15**:
1. **PR 1**: Audit current tools, define POSIX compatibility and maturity vocabulary.
2. **PR 2**: Add command registry, ownership map, and privilege contracts.
3. **PR 3**: Implement shared argument parsing, POSIX flag handling, `--`, and JSON formatting framework.
4. **PR 4**: Implement minimal VFS-backed file utilities (`cat`, `pwd`, `ls`, `stat`, `mkdir`, `touch`).
5. **PR 11**: Make system diagnostics (`sigma-diag`) runtime-aware.
6. **PR 15**: Establish contract and booted QEMU integration test suites.

Process (PR 8), storage mutation (PR 9), and account administration (PR 10) tools are strictly deferred until their respective kernel and service subsystem interfaces are fully integrated into the runtime.

---

## Periodic Maintenance PR Cadences

| Cadence | Proposed Task | Acceptance Evidence |
|---|---|---|
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

## Subsystem & Component Rules

1. **File Utilities**:
   - Treat file paths as raw data, not shell syntax.
   - Define exact symlink-following, overwrite, recursion, and partial I/O behavior.
   - Preserve file metadata only when explicitly specified.

2. **Text & Stream Utilities**:
   - Stream large inputs using bounded memory buffers.
   - Explicitly define byte vs. text character handling and UTF-8 encoding error behavior.
   - Prevent unbounded buffering that could trigger kernel OOM conditions.

3. **Process Utilities**:
   - Use authoritative kernel process management APIs.
   - Authorize signal sending and process inspection against the requesting process credentials.
   - Handle PID reuse races predictably.

4. **Storage Utilities**:
   - Separate read-only inspection (`df`, `du`) from mutating volume operations.
   - Display stable block device identifiers and target mount points.
   - Require dry-run preview and explicit interactive confirmation for destructive modifications.

5. **System Reporting**:
   - Explicitly label every reported value as live, cached, host-derived, or unavailable.
   - Bound diagnostic output length and automatically redact secret credentials and user tokens.

6. **Periodic Maintenance Runner**:
   - Tasks must be idempotent and strictly read-only by default.
   - Enforce execution timeout limits and output size caps.
   - Record task ID, start/end timestamps, result status (`Pass`, `Finding`, `Unavailable`, `Error`), and concise log summaries.
   - Support administrator override policies to disable individual tasks.

7. **Documentation Standards**:
   - Every manual page must contain standard sections: Name, Synopsis, Description, Options, Environment, Exit Status, Examples, and Standards/Compatibility Notes.

8. **Testing Standards**:
   - Distinguish isolated unit tests from end-to-end userspace interface tests (mirroring KUnit vs. kselftest).

---

## PR Review Template & Merge Gates

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

---

## Completion & Feature Status Gates

- **Proposed**: Initial design or PR proposal written without full implementation.
- **Prototype**: Utility logic compiled with standalone unit tests; not yet connected to kernel VFS or service syscalls.
- **Integrated**: Utility is integrated with real SigmaOS kernel/VFS interfaces and verified on a documented QEMU boot target.
- **Supported**: Utility passes repeatable end-to-end tests, with documented permissions, error handling, manual pages, and zero stale claims.
- **Periodic Checks**: Periodic tasks must additionally demonstrate idempotent reruns, bounded execution, and explicit handling when backend services are unavailable.
