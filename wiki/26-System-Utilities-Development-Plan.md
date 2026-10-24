# SigmaOS system-utilities plan

SigmaOS describes itself as an experimental OS codebase, not a supported general-purpose installation. Its feature inventory shows the shell is still a prototype, system-information reporting is hosted-only, and core components such as process spawning, syscall dispatch, and storage integration still have runtime milestones ahead. So system utilities should be developed in dependency order: **first a small set of trustworthy tools backed by real OS services; then broader POSIX-like utilities and recurring maintenance automation.**

This plan treats a *system utility* as a command-line program for inspecting or managing files, processes, storage, accounts, networking, services, and system health. It uses POSIX utility conventions as the portability baseline, with Linux and BSD tools as design references—not as a claim of compatibility.

## Rules for utility development

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

Each row is a proposed, separately reviewable PR. Split further if a PR crosses independent interfaces or exceeds what maintainers can test.

| PR | Proposed title | Scope | Acceptance criteria |
|---|---|---|---|
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

### Priority order

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
and anything not tested.

## Status
Proposed | Prototype | Integrated | Supported
Explain the evidence behind the status.

## Checklist
- [ ] Focused PR with measurable acceptance criteria
- [ ] Tests cover normal, invalid, and permission/error paths
- [ ] Help, exit status, stderr, and documentation match behavior
- [ ] No false success when a backend is unavailable
- [ ] Destructive actions are explicit and recoverable
- [ ] Capability matrix updated with evidence
- [ ] Required reviews and CI checks pass
```

Require maintainer approval and all relevant CI checks for every PR; require an additional security or subsystem review for privileged, destructive, or data-handling utilities. Protect the default branch against direct updates, require reviews/status checks, and resolve review conversations before merge. GitHub branch protection supports these controls.

**Exit criterion:** call a utility Integrated only when it uses the real SigmaOS interface on a documented boot target. Call it Supported only when its command contract, permissions, errors, manual, and runtime tests are repeatable. For periodic tasks, require safe reruns, bounded execution/output, clear result categories, and proof that the task did not report success when its data source was unavailable. Linux’s distinction between white-box tests and booted userspace tests is a useful reminder: model tests and actual system behavior are different evidence tiers.
