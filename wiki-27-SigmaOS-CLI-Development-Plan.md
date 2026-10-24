# SigmaOS CLI Development Plan

## Overview & Architecture

SigmaOS provides a scriptable, reliable command-line environment designed to operate effectively from a terminal, maintaining complete utility prior to and independently of graphical desktop availability.

To maintain architectural clarity and avoid circular dependencies, SigmaOS strictly distinguishes between two distinct CLI layers:

1. **System CLI Utilities Layer**: Standalone administrative, diagnostic, package, network, and system tools (e.g., `sigma`, system utilities, package queries, health monitors). These utilities communicate directly with kernel syscalls, IPC mechanisms, or typed backend service APIs.
2. **Shell Engine Layer**: The command interpreter that parses interactive or scripted command pipelines, performs expansions, handles redirections, manages process execution, and coordinates job control.

The CLI roadmap does not assume that shell pipelines, process execution, or graphical services already work; instead, administrative utilities function via direct typed IPC and syscall bindings.

---

## CLI Rules and Compatibility Goals

1. **POSIX Baseline & Syntax**:
   - Standard POSIX utility conventions serve as the baseline for syntax, option flags, positional operands, `--` end-of-options delimiters, standard I/O streams, and exit statuses.
   - Any intentional deviation from POSIX standards must be explicitly documented in the command reference manual.

2. **Discoverability**:
   - Standalone CLI utilities support `--help` and `--version`.
   - Command trees support both `sigma help <subcommand>` and `sigma <subcommand> --help`.
   - Options adhere to GNU Coding Standards for stable naming of long options.

3. **Machine-Readable Output**:
   - Standard output (`stdout`) contains human-readable output by default.
   - Diagnostics, warnings, and errors are directed exclusively to standard error (`stderr`).
   - Scriptable commands provide a stable JSON output option (`--json`) backed by a versioned schema.

4. **Typed APIs, Never Shell-String Composition**:
   - Administrative commands interact with kernel and service APIs through strongly-typed Rust structures and IPC.
   - User inputs are never passed into constructed shell strings or unvalidated system calls.

5. **Explicit and Recoverable Destructive Actions**:
   - Mutating or destructive operations default to dry-run/preview mode or require explicit interactive confirmation (`--yes` flag for non-interactive execution).
   - Read-only diagnostic commands are strictly separated from mutating state commands.

6. **Honest Capability Reporting**:
   - Utilities never simulate active service states. If a backend service or kernel subsystem is unavailable, the CLI returns an actionable "not supported / not connected" error with exit status rather than synthetic success.

7. **Terminal & Non-Terminal Respect**:
   - Output must not rely solely on color to convey meaning.
   - Color formatting is disabled automatically when output is not attached to a TTY or when the `NO_COLOR` environment variable is set.

8. **Comprehensive Documentation**:
   - Every public utility is documented with a manual page detailing `SYNOPSIS`, `DESCRIPTION`, `OPTIONS`, `ENVIRONMENT`, `EXIT STATUS`, `EXAMPLES`, `COMPATIBILITY`, and `KNOWN LIMITATIONS`.

9. **Untrusted Input Handling**:
   - All CLI parameters, paths, option arguments, and configuration keys are validated and bounded.
   - Sensitive data (tokens, keys, passwords) are redacted from logs, errors, and output reports.

10. **Reviewable PRs and Protected Gates**:
    - All command implementations require PR review, passing CI checks, and maintainer approval.

---

## Proposed PR Sequence

| PR | Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **1** | `docs(cli): define CLI architecture and command compatibility profile` | Audit `src/tools/`, `src/userspace/`, `shell`, `sigpkg`, installer, and docs. Define naming, POSIX profile, exit status codes, and unsupported feature behavior. | Inventory separates shell functionality from utilities; every existing command listed with actual runtime status; no command claimed as supported without evidence. |
| **2** | `docs(cli): publish command registry and CLI contribution rules` | Add command registry describing name, purpose, required privilege, mutability, backend API, arguments, output schema, and owner. | Every command has owner and test suite; duplicate names and alias collisions detected and documented. |
| **3** | `feat(cli): add shared argument, diagnostics, and output framework` | Shared Rust helpers for option parsing, usage text, error formatting, exit status, JSON serialization, color/TTY detection, and `--` parsing. | Golden tests cover valid/invalid flags, machine JSON output, stderr routing, `NO_COLOR`/non-TTY behavior, and exit status. |
| **4** | `feat(cli): add sigma command tree and help discovery` | Top-level `sigma` command with subcommand dispatch and nested help discovery. | `sigma --help`, `sigma help`, and `sigma help <subcommand>` work; unknown subcommands suggest valid names; unavailable backends fail cleanly. |
| **5** | `feat(cli): add version and build-information command` | Expose version, git commit revision, target architecture, and feature status without leaking build secrets. | Output is deterministic in text and JSON modes; values generated from build metadata; `--version` exits with code 0. |
| **6** | `feat(cli): integrate system information report command` | Convert host report generator into `sigma system info` command with text, JSON, and Markdown export modes distinguishing host vs SigmaOS runtime data. | Tests verify data sources; host execution explicitly labels host metrics vs SigmaOS metrics; unavailable metrics clearly marked. |
| **7** | `feat(cli): add read-only system status and diagnostics commands` | Add `sigma status`, `sigma hardware`, `sigma logs` (read-only), and health summaries via typed providers. | Commands display provider source and freshness; unsupported subsystems marked unavailable; stable JSON output supported. |
| **8** | `feat(cli): add package query commands` | Add `sigma pkg search`, `list`, `info` over repository metadata; no package mutation. | Search and list results are deterministic and scriptable; repository signatures validated; availability distinct from installation status. |
| **9** | `feat(cli): add package install and removal with transaction preview` | Add package mutation commands after signing, dependency resolution, and transaction rollback are integrated. | Preview lists proposed state changes; signature verification enforced; failure triggers clean rollback; removal requires `--yes` confirmation. |
| **10** | `feat(cli): add network status and safe configuration commands` | Read-only network status (`sigma net status`); mutation commands with auto-rollback added after network manager integration. | Read-only mode causes zero state changes; config validated prior to apply; safe auto-rollback tested; unsupported protocols reported. |
| **11** | `feat(cli): add service status and lifecycle commands` | Expose service status, start, stop, restart once service supervisor IPC and process spawning are operational. | Dependency graph failures visible; state confirmed with service supervisor before reporting success; lifecycle timeouts tested. |
| **12** | `feat(cli): add storage and installer inspection commands` | Read-only block device, partition table, mount, and installation target inspection (`sigma storage list`). | Device paths resolved through stable disk IDs; destructive partitioning isolated in distinct high-risk PRs with explicit confirmation. |
| **13** | `feat(cli): add shell completions and command suggestions` | Generate bash/zsh/fish completions from command registry; add typo distance suggestions without auto-execution. | Completions derive from same source-of-truth registry as help; hidden/privileged commands handled safely; zero external network queries. |
| **14** | `docs(cli): generate manuals and examples from command metadata` | Generate manual pages (`man`) and reference examples from command metadata. | Generated manuals match parser flags; man pages lint cleanly; every public utility documents exit statuses and examples. |
| **15** | `test(cli): add command behavior and compatibility harness` | Golden test harness for help, argument syntax, exit statuses, stdout/stderr isolation, JSON schemas, and non-TTY execution. | Harness runs deterministically in CI; test vectors match POSIX/GNU compatibility claims; deviations recorded. |
| **16** | `security(cli): audit privilege boundaries and sensitive output` | Audit privileged command handlers, auth/confirmation flows, file path traversal, secret handling, and diagnostic reports. | Authorization enforced by kernel/daemon, not UI; secrets redacted; read-only commands verified non-mutating; audit checks in CI. |
| **17** | `ci(cli): add command matrix and release verification` | Test commands against hosted dev environment and QEMU SigmaOS boot target with output schema validation. | CI explicitly records test target; hosted evidence never misrepresented as native runtime evidence; release smoke tests pass. |
| **18** | `docs(cli): publish CLI support matrix and troubleshooting guide` | Document supported commands, backends, environment variables, exit codes, permissions, and known limitations. | Integrated/Supported commands link to verified tests; errors include actionable resolution steps without false claims. |

*Initial Priority focus: PRs 1–7 (clarifying command baseline, shared CLI framework, and truthful read-only system diagnostics).*

---

## Periodic Maintenance PR Cadence

- **Every PR**: `ci(cli): run command contract checks` — Formatter, clippy/compile tests, CLI golden tests, JSON schema verification, security linting.
- **Weekly**: `maint(cli): review command failures and stale backend assumptions` — Assign new failures; update help text to ensure truthfulness regarding backend readiness.
- **Weekly**: `docs(cli): sync command registry and help examples` — Align command registry, generated help, man pages, and examples.
- **Monthly**: `test(cli): run command compatibility and output matrix` — Re-verify flag parsing, `--` option ending, stdin/stdout/stderr isolation, exit status codes, JSON schema compliance, and `NO_COLOR`/non-TTY behavior.
- **Monthly**: `security(cli): review privileged and mutating commands` — Verify authorization checks, confirmation prompts, dry-run previews, path sanitization, and secret redaction.
- **Monthly**: `audit(cli): review dependency and supply-chain changes` — Audit option parsers, serialization dependencies, MSRV, and licenses.
- **Quarterly**: `docs(cli): review POSIX/Linux/BSD compatibility claims` — Reassess utility flags and standards compliance; update documented deviations.
- **Quarterly**: `accessibility(cli): review terminal usability` — Keyboard navigation, screen-reader compatibility, color-independent status indicators, line wrapping, and localization.
- **Release Candidate**: `release(cli): certify command support claims` — Matrix certification across build revision, target architecture, manual generation, and verified test reports.
- **Post-Incident**: `fix(cli): prevent recurrence of <command issue>` — Add minimal reproducer test case, regression fix, docs update, and actionable error messages.

---

## Rules for Command Families

- **System Information**: Explicitly label data source (host OS, kernel model, or native SigmaOS runtime). Never report build host metrics as native OS state.
- **Package Management**: Enforce separation between query (`search`/`info`) and mutation (`install`/`remove`). Enforce signature verification and pre-transaction previews.
- **Networking**: Keep status commands read-only. Configuration changes require explicit confirmation and safe rollback mechanisms. Redact Wi-Fi passphrases and auth tokens.
- **Services and Processes**: Display authoritative state directly from service supervisor IPC. Never report state changes before supervisor confirmation.
- **Storage and Installer**: Require exact disk identification by stable UID/serial. Default to preview/dry-run. Partitioning and formatting require explicit `--yes` confirmation.
- **Security & Audit**: Output findings with clear severity, source module, and actionable remediation steps. Redact private keys, tokens, and credentials.
- **Logs**: Bounded log queries with time and source filtering. Keep raw log streams unpolluted by human formatting when JSON is requested.
- **JSON Output**: Schema versioning (`schema_version`). Stable key naming. Warnings and diagnostics routed to `stderr` or structured log fields.
- **Help and Errors**: Concise syntax guidance on usage error, pointing to detailed help (`sigma help <subcommand>`). Exit status ranges adhere to standard POSIX conventions (0 for success, 1 for runtime failure, 2 for CLI usage/argument error).
- **Manual Pages**: Standardized sections: `NAME`, `SYNOPSIS`, `DESCRIPTION`, `OPTIONS`, `ENVIRONMENT`, `EXIT STATUS`, `EXAMPLES`, `COMPATIBILITY`, `STANDARDS`, `SEE ALSO`, and `BUGS/LIMITATIONS`.

---

## PR Template and Merge Gates

All CLI pull requests must include the standard PR checklist and structure:

```markdown
## Summary
What command behavior or operational task does this PR add or fix?

## Scope
Command(s), backend(s), privileges, and what is explicitly not implemented.

## CLI contract
Synopsis, options, operands, stdin/stdout/stderr, exit statuses,
human/JSON output, and compatibility profile.

## Backend and runtime evidence
Hosted | unit/model | QEMU SigmaOS | physical target
State the data source and exact test environment.

## Safety and security
Read-only or mutating; authorization, confirmation, dry-run, rollback,
input validation, path handling, and secret redaction.

## Tests and documentation
Commands run, golden outputs, schema tests, manual pages, examples,
and anything not tested.

## Status
Proposed | Prototype | Integrated | Supported
Explain the evidence for the status.

## Checklist
- [ ] Focused and reviewable change
- [ ] Help/version and error behavior are tested
- [ ] POSIX deviations and output schema are documented
- [ ] No shell-string execution or false success on unavailable backends
- [ ] Destructive actions have explicit preview/confirmation/recovery
- [ ] Required CI passes; review conversations resolved
```

**Merge Gates**: Requires 1 maintainer approval for read-only utility/docs changes; requires 2 maintainer approvals for privileged execution, package mutation, network changes, storage modification, or security-sensitive output handling.

---

## Rollout Criteria

A CLI command is marked according to four explicit maturity tiers:
1. **Proposed**: Specification and interface defined in command registry.
2. **Prototype**: Implementation exists in hosted or mock harness; backends are simulated.
3. **Integrated**: Invokes native SigmaOS kernel/service backend and passes automated tests on a documented QEMU or hardware target.
4. **Supported**: Contract, manual, permission enforcement, error paths, JSON schema, and release test matrix pass repeatably in production release testing.

Until **Integrated** or **Supported** status is proven, tools must explicitly state their hosted/prototype status and clearly identify host vs native runtime data sources.
