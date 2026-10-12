# SigmaOS System Utilities Master Architecture, Support Matrix, and PR Plan

<<<<<<< HEAD
SigmaOS describes itself as an experimental OS codebase, not a supported general-purpose installation. Its feature inventory shows the shell is still a prototype, system-information reporting is hosted-only, and core components such as process spawning, syscall dispatch, and storage integration still have runtime milestones ahead. So system utilities should be developed in dependency order: **first a small set of trustworthy tools backed by real OS services; then broader POSIX-like utilities and recurring maintenance automation.**
=======
SigmaOS describes itself as an experimental OS codebase, not a supported general-purpose installation. Its feature inventory says the shell is a prototype with only `echo`, `pwd`, and `ls` built-ins and still needs process execution; the hosted system-report tool has no SigmaOS runtime integration; and init still needs process spawning and service-dependency enforcement. So the plan starts with a small set of real, useful utilities backed by working OS interfaces, rather than treating a large list of command names as an implemented userland.
>>>>>>> origin/main

This plan uses POSIX utility conventions for common command behavior as a baseline, while clearly marking GNU/Linux/BSD-inspired extensions.

## Rules for utility development

<<<<<<< HEAD
- **Keep the contract consistent.** Document synopsis, options, operands, standard input/output/error, exit statuses, environment, examples, and compatibility for every command. POSIX conventions cover utility syntax and the `--` end-of-options delimiter.
- **Separate portable behavior from extensions.** Label POSIX behavior, SigmaOS-specific options, and any GNU/Linux/BSD-inspired behavior. GNU Coreutils provides a broad reference for standard file and system utilities; FreeBSD’s base utilities offer a useful BSD comparison.
- **Make tools compose safely.** Use stdout for primary output and stderr for diagnostics; support stdin or `-` for file-oriented commands where appropriate; define predictable exit statuses; do not mix human output into machine-readable output.
- **Prefer stable machine-readable output for administration.** Add JSON only where scripting or automation benefits; version the output schema and test it separately from human-readable text.
- **Avoid false capability claims.** A utility must report when a service, syscall, filesystem operation, or device is unavailable; it must not substitute mock success for runtime behavior.
- **Design mutations for safety.** Make inspection read-only. For destructive operations, identify the target clearly, validate inputs, avoid dangerous path/symlink behavior, and provide preview, confirmation, or recovery where appropriate.
- **Use bounded and robust implementations.** Test long paths, unusual filenames, invalid encodings, large files, permission failures, partial I/O, and resource exhaustion.
- **Document and test the real execution path.** Distinguish unit/model tests from a utility running against SigmaOS syscalls and VFS. Linux distinguishes internal unit tests from userspace tests that exercise interfaces on a booted system; SigmaOS should make those test levels clear too.
- **Treat periodic maintenance as a first-class system facility.** FreeBSD’s `periodic` framework runs daily, weekly, monthly, and security tasks with configurable output and scheduling; SigmaOS can adopt the task categories without copying its implementation details.

## Proposed PR sequence
=======
## Baseline Rules

1. **Command Contracts**: Define each command’s contract: synopsis, options, operands, stdin/stdout/stderr, exit status, environment, and compatibility profile. POSIX’s utility conventions provide a baseline for option and operand syntax.
2. **Composability & Streams**: Keep normal results on stdout and diagnostics on stderr; support stdin for stream-oriented tools; use stable exit statuses; never mix human text into machine-readable output.
3. **Structured Machine Interfaces**: Treat JSON as a separate interface: provide it where useful for automation, version its schema, and test it independently from the human-readable output.
4. **Inspection vs Mutation**: Separate inspection from mutation: make status/report commands read-only; destructive operations must identify their target and provide safeguards such as preview, confirmation, or recovery.
5. **Truthful Error Reporting**: Never claim success for an unavailable backend: distinguish “unsupported,” “not integrated,” “permission denied,” and “operation failed.”
6. **Runtime Evidence Classification**: Test runtime behavior: label unit/model, hosted, QEMU booted SigmaOS, and physical-device evidence separately. Linux’s testing guidance distinguishes in-kernel white-box tests from userspace selftests that exercise actual interfaces.
7. **Safe and Bounded Maintenance**: Keep periodic checks safe and bounded: maintenance tasks should be idempotent where possible, have time/output limits, and report whether they passed, found an issue, or could not run. FreeBSD’s periodic framework is a useful reference for daily, weekly, monthly, and security-oriented tasks.
8. **Branch Protection & Review**: Use focused PRs and protected branches: require relevant review and passing checks; GitHub supports required pull-request reviews, status checks, and conversation resolution for protected branches.
>>>>>>> origin/main

Each row is a proposed, separately reviewable PR. Split further if a PR crosses independent interfaces or exceeds what maintainers can test.

| PR | Proposed title | Scope | Acceptance criteria |
|---|---|---|---|
<<<<<<< HEAD
| 1 | `docs(utils): define system-utility scope and compatibility policy` | Audit current utility code, shell built-ins, system-information tooling, VFS/syscall dependencies, and feature inventory. Define naming, POSIX subset, extension policy, status levels, and supported boot target. | Inventory shows each utility’s implementation, runtime dependencies, tests, and limitations; host-only tools are distinguished from SigmaOS runtime utilities. |
| 2 | `docs(utils): add command contract and utility registry` | Add a registry for command name, function, privilege, read/write behavior, dependencies, owner, manual page, and test evidence. | Every shipped/proposed command has one entry; duplicate names and unowned privileged commands are detected or resolved. |
| 3 | `feat(utils): add shared argument and diagnostic conventions` | Create common Rust helpers for argument parsing, usage output, error formatting, stdout/stderr, exit status, and optional JSON output. | Golden tests cover valid and invalid options, `--`, output streams, and status codes; shared helpers do not conceal command-specific semantics. |
| 4 | `feat(utils): add foundational file-inspection commands` | Implement or integrate `cat`, `pwd`, `ls`, `stat`, `mkdir`, and `touch` against SigmaOS VFS and permissions. | Commands use the real VFS path; tests cover missing paths, permission denial, unusual filenames, directories, and partial errors. The repository currently identifies `echo`, `pwd`, and `ls` as shell built-ins in a prototype shell; do not count those alone as a complete utility suite. |
| 5 | `feat(utils): add safe file mutation commands` | Add `cp`, `mv`, `rm`, `rmdir`, and `ln` in staged PRs, with explicit symlink, overwrite, and recursive behavior. | Tests cover same-file operations, overwrite refusal, symlinks, root/path edge cases, permission failures, and interruption; recursive deletion cannot escape its intended target. |
| 6 | `feat(utils): add text and stream tools` | Add `head`, `tail`, `wc`, `cut`, `sort`, `uniq`, and `tee` in dependency-sized PRs. | Tools correctly handle stdin, multiple files, empty input, large streams, UTF-8/binary policy, and pipeline errors; compatibility subset is documented. |
| 7 | `feat(utils): add environment and identity inspection tools` | Add `env`, `printenv`, `id`, `whoami`, `groups`, `uname`, and `hostname` where corresponding OS services exist. | Output comes from authoritative SigmaOS runtime providers; permission and unavailable-provider behavior is explicit; JSON schemas are stable if provided. |
| 8 | `feat(utils): add process and job-control utilities` | Add `ps`, `kill`, `sleep`, and `timeout` after process enumeration, signals, and process-group semantics are integrated. | Commands operate on real processes, enforce permissions, handle races and stale identifiers, and document signal/status behavior. |
| 9 | `feat(utils): add storage and filesystem status tools` | Add `df`, `du`, `mount`, `umount`, `sync`, and disk/device inspection after block, mount, and filesystem interfaces work. Keep formatting/partitioning separate. | Read-only commands cannot mutate devices; device identities are unambiguous; any future destructive operation has target confirmation and recovery guidance. |
| 10 | `feat(utils): add account and permission administration tools` | Add `chmod`, `chown`, `useradd`, `userdel`, and related tools only after identity database and access-control enforcement are real. | Authorization occurs in kernel/service APIs; privilege failures are tested; account changes are transactional and auditable. |
| 11 | `feat(utils): add system status and diagnostics commands` | Extend the existing hosted report tool into a provider-backed SigmaOS diagnostic command; distinguish host, cached, and live OS data. | No host metrics are presented as SigmaOS data; missing services report unavailable; report output is bounded and secrets are redacted. The current inventory calls the report hosted-only with no SigmaOS runtime integration. |
| 12 | `feat(periodic): add scheduled system-maintenance runner` | Implement a small scheduler/runner for daily, weekly, monthly, and security task groups; define task registration, timeout, environment, output, and configuration. | Tasks are opt-in or configured predictably; one failing task does not silently suppress the rest; logs are bounded, rotated or expired, and attributable. FreeBSD’s periodic model separates these task groups and allows configurable output. |
| 13 | `feat(periodic): add initial read-only health and security checks` | Add checks for failed services, filesystem capacity, stale temporary files, failed updates, insecure permissions, and authentication/security events—only where data sources exist. | Checks are read-only, report evidence and severity, are safe to rerun, and do not label unavailable checks as passing. |
| 14 | `docs(utils): add manual pages and runnable examples` | Document every shipped utility and periodic task; use consistent manual-page sections and validate rendered output. | Each command has a manual/reference page with synopsis, options, exit status, examples, and compatibility; documentation matches parser tests. FreeBSD’s documentation guide recommends standard sections and lint/render checks for manual pages. |
| 15 | `test(utils): add command contract and booted-system suite` | Add unit tests, golden CLI tests, filesystem fixtures, process tests, and QEMU integration tests for commands that depend on the OS runtime. | CI clearly labels test level; core utilities run against SigmaOS in a booted target; failures preserve logs and exact target configuration. |
| 16 | `security(utils): add unsafe-input and destructive-operation checks` | Audit path handling, symlink traversal, permissions, numeric parsing, environment variables, command execution, and log/report redaction. | High-risk paths have adversarial tests; utilities never build privileged shell commands from strings; unresolved security findings have owners and disposition. |
| 17 | `ci(utils): enforce command docs, status, and test consistency` | Add checks that compare the command registry, installed binaries, help output, manual pages, capability matrix, and integration tests. | CI catches undocumented commands, stale status, missing tests, or manual/help option drift; required checks are enforced before merging. |
| 18 | `release(utils): publish supported utility and maintenance-task matrix` | Document included commands, standards profile, architecture/boot target, dependencies, privileges, and support limitations. | “Supported” means tested against the stated runtime and target; no utility is called POSIX-compatible without appropriate conformance evidence. |
=======
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
>>>>>>> origin/main

### Priority order

<<<<<<< HEAD
Start with PRs **1–4, 11, and 15**: audit what exists, define shared behavior, implement a small real-VFS utility set, and add accurate runtime diagnostics. Next add safe file operations and stream tools. Defer process, storage, account administration, and scheduled security checks until the underlying process, filesystem, identity, and logging APIs are genuinely integrated. That keeps the tool surface aligned with SigmaOS’s current boot/runtime status rather than its feature names.

## Periodic maintenance PR cadence

| Cadence | Proposed recurring PR/task | Required output |
|---|---|---|
| Every PR | `ci(utils): run contract checks for changed commands` | Formatting/build, utility tests, help/manual checks, and relevant booted-system tests; skipped checks explained. |
| Weekly | `maint(utils): triage utility failures and stale capability claims` | Reproducer, runtime target, responsible owner, priority, and fix or documented deferral; inventory updated only with evidence. |
| Weekly | `docs(utils): reconcile command registry, help, and manuals` | Names/options/examples/status agree across code, registry, help output, and docs. |
| Monthly | `test(utils): run core command behavior matrix` | Check stdin, multiple operands, `--`, exit codes, stderr, permissions, long paths, unusual filenames, and non-TTY output. |
| Monthly | `security(utils): review privileged and destructive tools` | Revisit `rm`, mount/storage, permission, account, update, and service commands for path traversal, authorization, confirmation, rollback, and logging. |
| Monthly | `maint(periodic): review maintenance task results and output retention` | Task duration/failure, duplicate scheduling, log volume, output routing, and disabled/unavailable checks reviewed. |
| Quarterly | `docs(utils): review POSIX and Linux/BSD behavior differences` | Confirm supported option syntax and semantics against POSIX; document Linux/BSD extensions and deviations. POSIX utility conventions provide the common argument-syntax baseline. |
| Quarterly | `test(utils): run clean-boot and upgrade smoke matrix` | Validate core utilities and maintenance tasks on clean QEMU boot and supported update path; record exact revision/configuration. |
| Each release candidate | `release(utils): certify command set and periodic checks` | Utility matrix, manual pages, installed paths, privileges, maintenance-task results, known gaps, and release notes. |
| After each incident | `fix(utils): prevent recurrence of <failure>` | Root cause, minimized reproducer, regression test, data-loss/security impact, and backport/release decision. |

The periodic runner should never take destructive action merely because a timer fired. Default tasks should inspect, report, and request explicit policy for cleanup or repair. FreeBSD’s periodic system is a useful model for separating daily, weekly, monthly, and security checks and routing task output deliberately.

## Utility-family rules

- **File and text tools:** Treat filenames as data, not shell syntax; define symlink traversal and overwrite behavior; support streaming without unbounded buffering. GNU Coreutils’ manual is a broad reference for file, directory, text, process, permission, and system-context utility families.
- **Process tools:** Require real process/service APIs; distinguish process IDs from stable handles where possible; define races, signals, and exit status. Never signal a process without the kernel enforcing the caller’s authority.
- **Storage tools:** Keep status/inspection separate from formatting or partition changes; identify devices by stable properties as well as transient names; preview writes and support recovery.
- **Network and service tools:** Expose actual state from the network/service manager; mutation must validate first and have rollback or a documented recovery path.
- **Identity and permission tools:** Enforce ownership and capability checks in the system service or kernel; report partial failure clearly; audit changes.
- **System reports:** Identify data source and freshness; redact credentials and private user data; keep machine output schema-versioned.
- **Periodic checks:** Make checks idempotent and bounded; configure schedule and output; distinguish pass, finding, warning, bad configuration, and execution failure. FreeBSD’s `periodic` assigns different output/exit handling to successful, informative, warning, and failure results—a useful design reference.

## PR template and merge gates

```text
## Summary
What system task or utility gap does this PR address?

## Command/task
Name, synopsis or schedule, owner, privilege level, and backend dependencies.

## Compatibility contract
POSIX subset, SigmaOS behavior, Linux/BSD extensions, and deviations.

## Side effects and safety
Read-only or mutating; path/permission checks, confirmation, dry-run,
rollback/recovery, and output redaction.

## Runtime evidence
Unit/model | hosted | QEMU booted SigmaOS | physical target
State exactly where it was tested and what data source it used.

## Tests and docs
Commands run, expected output/exit statuses, manual page, examples,
=======
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
>>>>>>> origin/main
and anything not tested.

## Status
Proposed | Prototype | Integrated | Supported
Explain the evidence behind the status.

## Checklist
- [ ] Focused PR with measurable acceptance criteria
<<<<<<< HEAD
- [ ] Tests cover normal, invalid, and permission/error paths
- [ ] Help, exit status, stderr, and documentation match behavior
- [ ] No false success when a backend is unavailable
- [ ] Destructive actions are explicit and recoverable
- [ ] Capability matrix updated with evidence
- [ ] Required reviews and CI checks pass
```

Require maintainer approval and all relevant CI checks for every PR; require an additional security or subsystem review for privileged, destructive, or data-handling utilities. Protect the default branch against direct updates, require reviews/status checks, and resolve review conversations before merge. GitHub branch protection supports these controls.

**Exit criterion:** call a utility Integrated only when it uses the real SigmaOS interface on a documented boot target. Call it Supported only when its command contract, permissions, errors, manual, and runtime tests are repeatable. For periodic tasks, require safe reruns, bounded execution/output, clear result categories, and proof that the task did not report success when its data source was unavailable. Linux’s distinction between white-box tests and booted userspace tests is a useful reminder: model tests and actual system behavior are different evidence tiers.
=======
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
>>>>>>> origin/main
