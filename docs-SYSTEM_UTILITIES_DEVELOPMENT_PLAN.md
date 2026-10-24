# SigmaOS System Utilities Development Plan

## Executive Summary

SigmaOS's README describes an experimental OS codebase, not a supported general-purpose installation. Its feature inventory specifies that the shell is a prototype with built-ins (`echo`, `pwd`, `ls`) and requires process execution integration; the hosted system-report tool has no SigmaOS runtime integration; and init requires process spawning and service-dependency enforcement.

Therefore, this plan establishes a small set of real, useful utilities backed by working OS interfaces, rather than treating a large list of command names as an implemented userland. The development roadmap follows a sequence of proposed Pull Requests (PRs), supplemented by recurring periodic maintenance cadences inspired by POSIX utility conventions and BSD/Linux operational frameworks.

---

## Baseline Rules

### 1. Command Contract Definition
Every utility must define an explicit contract:
- **Synopsis:** Command usage syntax following POSIX option/operand conventions.
- **Options & Operands:** Parsing syntax adhering to POSIX utility standards (e.g., standard flag handling, `--` delimiter support).
- **Streams (stdin/stdout/stderr):** Standard input for stream tools; normal program output on `stdout`; error messages and diagnostics strictly on `stderr`.
- **Exit Status:** Stable, predictable exit codes (`0` success, `1` runtime failure, `2` syntax error, `126` permission denied, `127` command not found).
- **Environment & Compatibility Profile:** Declared POSIX profile along with GNU/Linux/BSD-inspired extensions clearly marked.

### 2. Tool Composability
- Never mix human-formatted text into machine-readable streams on `stdout`.
- Standardize stream-oriented tools to read from `stdin` when operand files are omitted or specified as `-`.
- Ensure output formats remain deterministic and pipeline-friendly across command combinations.

### 3. Structured Output & JSON Interfaces
- Structured output (such as `--json`) is provided as a separate, first-class interface for automation.
- Schema definitions for JSON payloads must be explicitly versioned.
- Machine-readable JSON output paths must be tested independently from human-readable text output.

### 4. Separation of Inspection from Mutation
- Status and report commands (`pwd`, `stat`, `df`, `ps`, `sigma-diag`) are strictly read-only and free of mutating side-effects.
- Destructive or mutating operations (`rm`, `userdel`, formatting) must explicitly identify target resources and incorporate safeguards such as previews, confirmation prompts, dry-run flags, or recovery options.

### 5. Truthful Backend Reporting
- Commands must never claim success when an underlying kernel or service backend is missing or unavailable.
- Implement explicit diagnostic distinctions for four separate failure conditions:
  1. `unsupported` (feature not implemented in current build)
  2. `not integrated` (backend exists but binding unavailable)
  3. `permission denied` (authorization failed)
  4. `operation failed` (runtime backend error)

### 6. Layered Testing & Runtime Evidence
Following Linux testing guidelines (distinguishing KUnit in-kernel white-box tests from kselftest userspace interface tests), test evidence must be classified into four distinct maturity tiers:
1. `Unit/Model`: In-memory Rust unit tests and simulated data models.
2. `Hosted`: Execution under standard host OS developer environments.
3. `QEMU Booted`: Verification against a booted SigmaOS kernel and VFS inside QEMU.
4. `Physical Target`: Validation on bare-metal target hardware.

### 7. Bounded Periodic Maintenance Framework
- Maintenance tasks must be safe, idempotent, and bounded by time and memory limits.
- Tasks must explicitly report outcome categories: `Pass`, `Finding`, `Warning`, `BadConfig`, or `ExecutionFailure`.
- Follow FreeBSD's periodic framework model for daily, weekly, monthly, and security maintenance tasks.

### 8. Focused PRs & Protected Branch Merge Gates
- PRs must be strictly scoped to single utility families or maintenance subsystems.
- All code additions require unit test coverage, updated manuals, passing CI checks, and resolved review conversations prior to merge.

---

## Proposed PR Roadmap

| PR # | Proposed Title | Scope | Acceptance Criteria |
|:---:|:---|:---|:---|
| **1** | `docs(utils): audit current tools and define compatibility policy` | Inventory utility code, shell built-ins, hosted tools, syscall/VFS dependencies, tests, and documentation. Define POSIX scope, naming, maturity vocabulary, and runtime evidence requirements. | Each existing command is labeled by implementation and test level; hosted tools are not described as SigmaOS runtime tools; compatibility claims have explicit criteria. |
| **2** | `docs(utils): add command registry and ownership map` | Add a registry of command name, purpose, owner, privilege, read/write behavior, backend, docs page, and tests. | No duplicate names or undocumented privileged commands; registry can drive generated help/docs where appropriate. |
| **3** | `feat(utils): add shared CLI contract helpers` | Add reusable Rust support for argument parsing, usage text, errors, exit status, stdout/stderr, TTY/color detection, and optional structured output. | Golden tests cover valid/invalid args, `--`, non-TTY use, stdout/stderr separation, and documented exit statuses. |
| **4** | `feat(utils): implement minimal file and directory utilities` | Add or integrate `cat`, `pwd`, `ls`, `stat`, `mkdir`, and `touch` against the real VFS and permission model. | QEMU/user-process tests exercise actual VFS calls; invalid paths, permissions, unusual filenames, and partial errors are covered. |
| **5** | `feat(utils): implement safe file mutation utilities` | Add `cp`, `mv`, `rm`, `rmdir`, and `ln` in small PRs; define overwrite, recursive, and symlink behavior. | Tests cover symlink/path traversal, same-file operations, overwrite behavior, permission failures, and interrupted operations; recursive deletion cannot escape the chosen target. |
| **6** | `feat(utils): add stream and text-processing utilities` | Add `head`, `tail`, `wc`, `cut`, `sort`, `uniq`, and `tee` incrementally. | Tools handle stdin, empty input, multiple operands, large streams, and defined text/binary behavior without unbounded buffering. |
| **7** | `feat(utils): add system identity and environment queries` | Add `env`, `printenv`, `id`, `whoami`, `uname`, and `hostname` only when backed by real process/account/kernel APIs. | Results come from SigmaOS providers; permissions and missing providers are handled explicitly; machine output schemas are stable if added. |
| **8** | `feat(utils): add process inspection and signal tools` | Add `ps`, `kill`, `sleep`, and `timeout` after process enumeration, signal delivery, and process-group semantics work. | Commands operate on real processes; target identity and authorization are checked; races and exit behavior are documented and tested. |
| **9** | `feat(utils): add storage and filesystem status tools` | Add `df`, `du`, `mount` inspection, `sync`, and device status once block/VFS/mount interfaces are integrated. Keep formatting and partition changes separate. | Read-only inspection never writes to devices; device identity is clear; errors and unavailable storage are distinguishable. |
| **10** | `feat(utils): add account and permission administration tools` | Add permission/account management only after identity storage, kernel authorization, and audit logging are implemented. | Permission checks occur in the authoritative kernel/service API; partial failures are reported; changes are auditable and tested. |
| **11** | `feat(utils): make system reports SigmaOS-runtime aware` | Extend the existing report tool with actual SigmaOS providers for kernel, hardware, filesystems, network, and services as they become available. | Each field identifies its source/freshness; host metrics are never represented as SigmaOS state; missing providers report unavailable. |
| **12** | `feat(periodic): add bounded maintenance-task runner` | Implement daily/weekly/monthly/security task groups, configuration, timeouts, output routing, and enable/disable policy. | Tasks are attributable and bounded; one task failure does not suppress others; logs are controlled; tasks do not perform destructive repairs by default. |
| **13** | `feat(periodic): add initial read-only health checks` | Add checks for failed services, disk capacity, update status, permission anomalies, and security events when those providers exist. | Every check reports pass/finding/unavailable/error; checks are read-only, rerunnable, and supported by fixtures or runtime tests. |
| **14** | `docs(utils): add manual pages and executable examples` | Document every command and periodic check using consistent manual sections; add examples exercised by tests where practical. | Manuals document syntax, options, exit status, examples, compatibility, and limitations; rendered output is validated. |
| **15** | `test(utils): add command contract and booted-system suite` | Add unit, golden-output, VFS, process, and QEMU integration tests; classify test evidence. | CI separates model tests from booted runtime tests; core utilities run against SigmaOS interfaces on a named QEMU target. |
| **16** | `security(utils): audit path, privilege, and data-handling risks` | Review path/symlink handling, numeric parsing, permissions, shell invocation, logs, and destructive operations. | No privileged command is built from shell strings; untrusted paths and inputs have regression tests; secrets/private data are redacted. |
| **17** | `ci(utils): validate registry, help, manuals, and status matrix` | Check installed command list, registry, help output, manual pages, feature status, and test coverage for drift. | CI fails on undocumented commands, stale claims, missing manuals, or parser/docs mismatches. |
| **18** | `release(utils): publish utility support matrix` | List commands, versions, compatibility profile, privilege, backend, architecture, and known gaps. | “Supported” tools have repeatable runtime evidence; unsupported behavior is explicitly listed. |

### Priority Sequence
The highest-value initial sequence is **PRs 1–4, 11, and 15**:
1. Establish truthful status claims.
2. Ensure consistent CLI parsing behavior.
3. Implement basic VFS utilities (`cat`, `pwd`, `ls`, `stat`, `mkdir`, `touch`).
4. Implement runtime-backed diagnostic reporting (`sigma-diag`).
5. Establish booted system test paths.

Process, storage, account administration, and periodic security tools are deferred until their respective kernel and service interfaces are fully integrated.

---

## Periodic Maintenance Cadence

| Cadence | Task Title | Acceptance Evidence |
|:---|:---|:---|
| **Every PR** | `ci(utils): run utility contract checks` | Build, unit/golden tests, docs checks, and affected QEMU tests; skipped checks explained. |
| **Weekly** | `maint(utils): triage failures and stale status claims` | Reproducer, affected command/backend, owner, priority, and fix or documented deferral; feature status updated only from evidence. |
| **Weekly** | `docs(utils): reconcile registry, help, and manuals` | Command names, options, examples, and status agree across code, registry, help, and docs. |
| **Monthly** | `test(utils): run core command behavior matrix` | Verify stdin, multiple operands, `--`, exit status, stderr, permission errors, long/unusual filenames, and non-TTY use. |
| **Monthly** | `security(utils): review privileged and destructive utilities` | Review `rm`, storage, account, mount, service, and update operations for authorization, target validation, confirmation, rollback, and data exposure. |
| **Monthly** | `maint(periodic): review maintenance-task results and retention` | Runtime, failure rate, duplicate scheduling, time/output bounds, and log retention assessed; unavailable checks are not counted as passed. |
| **Quarterly** | `docs(utils): review POSIX and Linux/BSD deviations` | Revalidate declared option syntax and semantics against POSIX; document platform-specific behavior. |
| **Quarterly** | `test(utils): run clean-boot and upgrade smoke tests` | Core commands and periodic tasks run after fresh QEMU boot and supported upgrade path; configuration and logs retained. |
| **Each Release Candidate** | `release(utils): certify shipped tools and maintenance checks` | Supported command matrix, manual pages, permissions, boot target, periodic results, and open limitations recorded. |
| **After Each Incident** | `fix(utils): prevent recurrence of <failure>` | Root cause, test reproducer, data-loss/security assessment, and backport decision included. |

---

## Component & Utility-Family Rules

- **File utilities:** Treat names as data, not shell syntax; define symlink-following, overwrite, recursion, and partial-I/O semantics; preserve file metadata only when explicitly specified.
- **Text utilities:** Stream large input; define byte-vs-text behavior, locale, and invalid encoding handling; avoid hidden buffering that can exhaust memory.
- **Process utilities:** Use kernel process APIs; authorize signals and inspection; make stale IDs/races predictable; do not report process state from a model or cache as live state.
- **Storage utilities:** Separate inspection from destructive operations; show stable device identity and exact target; require preview/confirmation/recovery for future writes.
- **System reporting:** Mark each value live, cached, host-derived, or unavailable; bound output and redact credentials and private user data.
- **Maintenance runner:** Make tasks idempotent and read-only by default; set time/resource limits; record task ID, start/end, result category, and concise output; allow administrator policy to disable tasks.
- **Documentation:** Use a consistent manual structure with synopsis, description, options, environment, exit status, examples, and standards/compatibility notes.
- **Testing:** Use unit tests for isolated logic and end-to-end tests for actual userspace interfaces. Linux distinguishes KUnit's internal white-box tests from kselftest's userspace/system-level tests; SigmaOS preserves this distinction in reports.

---

## PR Review Template & Merge Gates

```text
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

## Maturity & Completion Gates

1. **Integrated:** Mark a utility `Integrated` only when it uses the real SigmaOS interface on a documented boot target.
2. **Supported:** Mark it `Supported` only after repeatable tests, documented behavior, permissions, failure paths, and manual pages all agree.
3. **Periodic Checks:** Periodic checks must additionally show safe reruns, bounded runtime/output, and explicit handling when their provider is unavailable.
