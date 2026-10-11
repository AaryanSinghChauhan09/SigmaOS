# SigmaOS System Utilities Development Plan & Command Registry Architecture

SigmaOS describes itself as an experimental OS codebase, not a supported general-purpose installation. Its feature inventory shows the shell is still a prototype, system-information reporting is hosted-only, and core components such as process spawning, syscall dispatch, and storage integration still have runtime milestones ahead. System utilities should be developed in dependency order: **first a small set of trustworthy tools backed by real OS services; then broader POSIX-like utilities and recurring maintenance automation.**

This plan treats a *system utility* as a command-line program for inspecting or managing files, processes, storage, accounts, networking, services, and system health. It uses POSIX utility conventions as the portability baseline, with Linux and BSD tools as design references—not as a claim of compatibility.

---

## Rules for Utility Development

- **Keep the contract consistent:** Document synopsis, options, operands, standard input/output/error, exit statuses, environment, examples, and compatibility for every command. POSIX conventions cover utility syntax and the `--` end-of-options delimiter.
- **Separate portable behavior from extensions:** Label POSIX behavior, SigmaOS-specific options, and any GNU/Linux/BSD-inspired behavior. GNU Coreutils provides a broad reference for standard file and system utilities; FreeBSD’s base utilities offer a useful BSD comparison.
- **Make tools compose safely:** Use stdout for primary output and stderr for diagnostics; support stdin or `-` for file-oriented commands where appropriate; define predictable exit statuses; do not mix human output into machine-readable output.
- **Prefer stable machine-readable output for administration:** Add JSON only where scripting or automation benefits; version the output schema and test it separately from human-readable text.
- **Avoid false capability claims:** A utility must report when a service, syscall, filesystem operation, or device is unavailable; it must not substitute mock success for runtime behavior.
- **Design mutations for safety:** Make inspection read-only. For destructive operations, identify the target clearly, validate inputs, avoid dangerous path/symlink behavior, and provide preview, confirmation, or recovery where appropriate.
- **Use bounded and robust implementations:** Test long paths, unusual filenames, invalid encodings, large files, permission failures, partial I/O, and resource exhaustion.
- **Document and test the real execution path:** Distinguish unit/model tests from a utility running against SigmaOS syscalls and VFS.
- **Treat periodic maintenance as a first-class system facility:** FreeBSD’s `periodic` framework runs daily, weekly, monthly, and security tasks with configurable output and scheduling; SigmaOS adopts the task categories without copying implementation details.

---

## Proposed PR Sequence (18-PR Roadmap)

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(utils): define system-utility scope and compatibility policy` | Audit current utility code, shell built-ins, system-information tooling, VFS/syscall dependencies, and feature inventory. | Inventory shows each utility’s implementation, runtime dependencies, tests, and limitations; host-only tools are distinguished from SigmaOS runtime utilities. |
| 2 | `docs(utils): add command contract and utility registry` | Add a registry for command name, function, privilege, read/write behavior, dependencies, owner, manual page, and test evidence. | Every shipped/proposed command has one entry; duplicate names and unowned privileged commands are detected or resolved. |
| 3 | `feat(utils): add shared argument and diagnostic conventions` | Create common Rust helpers for argument parsing, usage output, error formatting, stdout/stderr, exit status, and optional JSON output. | Golden tests cover valid and invalid options, `--`, output streams, and status codes; shared helpers do not conceal command-specific semantics. |
| 4 | `feat(utils): add foundational file-inspection commands` | Implement or integrate `cat`, `pwd`, `ls`, `stat`, `mkdir`, and `touch` against SigmaOS VFS and permissions. | Commands use the real VFS path; tests cover missing paths, permission denial, unusual filenames, directories, and partial errors. |
| 5 | `feat(utils): add safe file mutation commands` | Add `cp`, `mv`, `rm`, `rmdir`, and `ln` in staged PRs, with explicit symlink, overwrite, and recursive behavior. | Tests cover same-file operations, overwrite refusal, symlinks, root/path edge cases, permission failures, and interruption. |
| 6 | `feat(utils): add text and stream tools` | Add `head`, `tail`, `wc`, `cut`, `sort`, `uniq`, and `tee` in dependency-sized PRs. | Tools correctly handle stdin, multiple files, empty input, large streams, UTF-8/binary policy, and pipeline errors. |
| 7 | `feat(utils): add environment and identity inspection tools` | Add `env`, `printenv`, `id`, `whoami`, `groups`, `uname`, and `hostname` where corresponding OS services exist. | Output comes from authoritative SigmaOS runtime providers; permission and unavailable-provider behavior is explicit. |
| 8 | `feat(utils): add process and job-control utilities` | Add `ps`, `kill`, `sleep`, and `timeout` after process enumeration, signals, and process-group semantics are integrated. | Commands operate on real processes, enforce permissions, handle races and stale identifiers, and document signal/status behavior. |
| 9 | `feat(utils): add storage and filesystem status tools` | Add `df`, `du`, `mount`, `umount`, `sync`, and disk/device inspection after block, mount, and filesystem interfaces work. | Read-only commands cannot mutate devices; device identities are unambiguous; target confirmation for destructive operations. |
| 10 | `feat(utils): add account and permission administration tools` | Add `chmod`, `chown`, `useradd`, `userdel`, and related tools only after identity database and access-control enforcement are real. | Authorization occurs in kernel/service APIs; privilege failures are tested; account changes are transactional and auditable. |
| 11 | `feat(utils): add system status and diagnostics commands` | Extend system-info tools into provider-backed SigmaOS diagnostic commands (`sigma-diag`); distinguish host, cached, and live OS data. | No host metrics are presented as SigmaOS data; missing services report unavailable; report output is bounded and secrets are redacted. |
| 12 | `feat(periodic): add scheduled system-maintenance runner` | Implement a small scheduler/runner (`sigma-periodic`) for daily, weekly, monthly, and security task groups. | Tasks are opt-in or configured predictably; one failing task does not suppress the rest; logs are bounded, rotated or expired. |
| 13 | `feat(periodic): add initial read-only health and security checks` | Add checks for failed services, filesystem capacity, stale temporary files, failed updates, insecure permissions, and security events. | Checks are read-only, report evidence and severity, are safe to rerun, and do not label unavailable checks as passing. |
| 14 | `docs(utils): add manual pages and runnable examples` | Document every shipped utility and periodic task; use consistent manual-page sections and validate rendered output. | Each command has a manual page with synopsis, options, exit status, examples, and compatibility. |
| 15 | `test(utils): add command contract and booted-system suite` | Add unit tests, golden CLI tests, filesystem fixtures, process tests, and QEMU integration tests. | CI clearly labels test level; core utilities run against SigmaOS in a booted target. |
| 16 | `security(utils): add unsafe-input and destructive-operation checks` | Audit path handling, symlink traversal, permissions, numeric parsing, environment variables, command execution, and log redaction. | High-risk paths have adversarial tests; utilities never build privileged shell commands from strings. |
| 17 | `ci(utils): enforce command docs, status, and test consistency` | Add checks that compare the command registry, installed binaries, help output, manual pages, capability matrix, and integration tests. | CI catches undocumented commands, stale status, missing tests, or manual/help option drift. |
| 18 | `release(utils): publish supported utility and maintenance-task matrix` | Document included commands, standards profile, architecture/boot target, dependencies, privileges, and support limitations. | "Supported" means tested against stated runtime and target; no utility called POSIX-compatible without conformance evidence. |

---

## Periodic Maintenance Cadence

- **Every PR:** `ci(utils): run contract checks for changed commands`
- **Weekly:** `maint(utils): triage utility failures and stale capability claims`
- **Weekly:** `docs(utils): reconcile command registry, help, and manuals`
- **Monthly:** `test(utils): run core command behavior matrix`
- **Monthly:** `security(utils): review privileged and destructive tools`
- **Monthly:** `maint(periodic): review maintenance task results and output retention`
- **Quarterly:** `docs(utils): review POSIX and Linux/BSD behavior differences`
- **Quarterly:** `test(utils): run clean-boot and upgrade smoke matrix`
- **Release Candidate:** `release(utils): certify command set and periodic checks`
- **Post Incident:** `fix(utils): prevent recurrence of failure`
