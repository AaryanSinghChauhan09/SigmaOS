# SigmaOS CLI Development Plan

This document defines the architecture, portability standards, command rules, PR roadmap, maintenance cadence, and rollout criteria for the SigmaOS Command-Line Interface (CLI).

---

## 1. Executive Summary & Architecture

SigmaOS provides a scriptable, reliable CLI experience that operates seamlessly from a terminal or automated environment, regardless of whether a graphical user interface (GUI) or display server is active.

To prevent architectural ambiguity, SigmaOS explicitly separates the CLI ecosystem into two distinct layers:

1. **System CLI Utilities (`sigma` & System Tools)**: Executable programs and administrative diagnostic commands that inspect runtime state, manage packages, configure networking, inspect storage, and control services. Administrative commands invoke typed system services or kernel syscalls via Rust APIs, never via shell-string concatenation or unvalidated subprocess execution.
2. **Command Interpreter (`shell`)**: The interactive shell binary responsible for parsing user input, processing redirections and pipelines, managing variables, handling signals/job control, and executing system binaries.

**Key Rule**: The CLI roadmap must not assume that shell pipelines, process execution, or graphical desktop services are fully available or functional before their respective backends pass verification.

---

## 2. CLI Rules and Compatibility Goals

### POSIX Profile Baseline
- Use POSIX utility conventions as the baseline for options, operands, flags, standard streams, and exit statuses.
- Support `--` as the explicit end-of-options delimiter.
- Document any deliberate deviation from standard POSIX or GNU conventions explicitly in command documentation and man pages.

### Discoverability & Standards
- Standalone utilities must provide `--help` and `--version`.
- Subcommand trees (`sigma`) must support both `sigma help <command>` and `sigma <command> --help`.
- Follow GNU Coding Standards for long option naming and standard help/version exit codes (0 for `--help`/`--version`).

### Machine-Readable & Human-Readable Output
- Human-readable output defaults to standard output (`stdout`).
- Errors, warnings, and diagnostic messages are routed exclusively to standard error (`stderr`).
- Machine-readable output is supported via `--format=json` or `--json`, utilizing versioned schemas (`schema_version: "1.0"`).
- Terminal detection (TTY vs non-TTY) or output redirection must not automatically change stdout from text to JSON.

### Safety, Confirmation & Recovery
- All state-mutating operations require explicit user confirmation or an explicit `--yes` / `-y` flag in non-interactive/scripted contexts.
- Provide dry-run / preview modes (`--dry-run`) for destructive or complex administrative commands (e.g., package removal, partitioning, service stops).
- Maintain a strict boundary between read-only diagnostic queries and state-mutating operations.

### Honest Subsystem Reporting
- System commands must never report simulated success or fake health for unavailable or uninitialized subsystems.
- If a runtime service is offline or unsupported on the current target, commands must return an actionable error status indicating "not connected" or "unsupported feature".
- Always explicitly label data sources as host-level, kernel-model, or live SigmaOS runtime.

### Terminal Usability & Accessibility
- Never rely on color alone to convey status or meaning (always pair colors with text/symbols like `[OK]`, `[FAIL]`, `[WARN]`).
- Respect `NO_COLOR` environment variable and non-TTY file descriptors by automatically disabling ANSI color codes.
- Ensure CLI output is non-blocking, script-friendly, and usable with screen readers or text-to-speech tooling.

### Untrusted Input & Security
- Treat all command-line parameters, positional arguments, and environment variables as untrusted input.
- Bound output size, string lengths, and log outputs to avoid memory exhaustion or denial of service.
- Automatically redact sensitive credentials, tokens, private keys, Wi-Fi passwords, and command history parameters in diagnostic output.

---

## 3. Proposed 18-PR Sequence Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **PR 1** | `docs(cli): define CLI architecture and command compatibility profile` | Audit `src/tools/`, `src/userspace/`, shell, package manager, installer, diagnostics, and docs. Define command naming, interface layers, POSIX profile, status codes, output modes, and unsupported-feature behavior. | Inventory separates shell functionality from utilities; every existing command is listed with actual implementation/runtime status; no command is called supported without evidence. |
| **PR 2** | `docs(cli): publish command registry and CLI contribution rules` | Add a command registry describing name, purpose, privilege, mutability, API backend, arguments, output schema, and owner. | Every proposed command has an owner and acceptance tests; duplicate names and inconsistent aliases are detected or documented. |
| **PR 3** | `feat(cli): add shared argument, diagnostics, and output framework` | Add reusable Rust helpers for option parsing, usage text, error formatting, exit status, JSON serialization, color/TTY detection, and `--` handling. | Golden tests cover valid/invalid args, machine output, stderr, no-color/non-TTY behavior, and exit codes; framework does not force one parser dependency without license/MSRV review. |
| **PR 4** | `feat(cli): add sigma command tree and help discovery` | Build a top-level `sigma` command with subcommands and nested help; keep it usable before all backends are implemented. | `sigma --help`, `sigma help`, and `sigma help <subcommand>` work; unknown commands suggest valid names; unavailable backends fail clearly. |
| **PR 5** | `feat(cli): add version and build-information command` | Expose version, source revision, build target, and feature profile without leaking build secrets or unsupported release claims. | Output is deterministic in text and JSON; values are generated from build metadata; `--version` exits successfully. |
| **PR 6** | `feat(cli): integrate system information report command` | Turn the existing hosted report tool into a `sigma system info` command; preserve text/JSON/Markdown exports and add a clear distinction between host data and SigmaOS runtime data. | Tests prove report source and target; running on Linux does not misrepresent host metrics as SigmaOS; unavailable metrics are explicitly marked. |
| **PR 7** | `feat(cli): add read-only system status and diagnostics commands` | Add `sigma status`, `sigma hardware`, `sigma logs` (read-only), and health summaries using typed providers. | Commands show provider freshness/source; no unsupported subsystem is reported as healthy; output supports stable JSON and bounded logs. |
| **PR 8** | `feat(cli): add package query commands` | Add `sigma pkg search/list/info` over the repository/package metadata; do not install/remove packages yet. | Search/list results are stable and scriptable; repository metadata is validated; package availability is distinct from installed status. |
| **PR 9** | `feat(cli): add package install and removal with transaction preview` | Add package mutation only after package signing, dependency resolution, permissions, and rollback behavior are integrated. | Preview lists changes; signature/provenance verification is enforced; failure rolls back or leaves a documented recoverable state; destructive removal requires explicit confirmation. |
| **PR 10** | `feat(cli): add network status and safe configuration commands` | Add read-only interface/address/route/DNS status first; add mutation only after network configuration and rollback are integrated. | Read-only mode never changes state; configuration validates before apply; rollback path tested; unsupported network stack features are reported honestly. |
| **PR 11** | `feat(cli): add service status and lifecycle commands` | Expose service-manager status and controlled start/stop/restart once process spawning and service dependency enforcement are real. | Dependency failures and permissions are visible; commands do not claim services started unless the runtime confirms; stop/restart behavior is tested. |
| **PR 12** | `feat(cli): add storage and installer inspection commands` | Add read-only block-device, partition, mount, and installation status; keep partitioning/formatting in separate high-risk PRs. | Device paths are resolved through trusted IDs; no destructive operation occurs in inspection commands; exact device/partition target is shown before any future mutation. |
| **PR 13** | `feat(cli): add shell completions and command suggestions` | Generate completion for supported shells from the command registry; add typo suggestions without executing commands. | Completion uses the same source of truth as help; hidden/privileged commands are handled appropriately; completion does not query unstable network services. |
| **PR 14** | `docs(cli): generate manuals and examples from command metadata` | Add man-page/reference generation and command examples; choose project-supported markup after checking installed tooling. | Generated docs match parser options; man pages lint and render; every public command documents exit status and examples. Rust CLI tools such as clap can generate help/completions, and companion tooling can generate manuals, but dependency adoption must be reviewed. |
| **PR 15** | `test(cli): add command behavior and compatibility harness` | Add golden tests for help/version, argument syntax, exit codes, stdout/stderr, JSON schemas, and non-TTY behavior; compare POSIX-like utilities only where compatibility is claimed. | Tests run deterministically in CI; test vectors document the target profile; intentional deviations are recorded. |
| **PR 16** | `security(cli): audit privilege boundaries and sensitive output` | Audit privileged command handlers, auth/confirmation flows, filesystem path handling, secrets, logs, and generated reports. | No privilege comes from UI-only checks; secrets are redacted; read-only commands are demonstrably read-only; security checks are part of required CI. |
| **PR 17** | `ci(cli): add command matrix and release verification` | Test commands against hosted development environment and, once available, the SigmaOS QEMU boot target. Add output-schema compatibility checks and release smoke tests. | CI identifies which backend was tested; host-only tests never count as SigmaOS runtime evidence; release claims match the tested target. |
| **PR 18** | `docs(cli): publish CLI support matrix and troubleshooting guide` | Document supported command/backends, target environment, examples, exit codes, permissions, and known gaps. | Every “Integrated” or “Supported” command links to tests and a known runtime; errors include next actionable diagnostic without claiming unavailable features. |

---

## 4. Periodic Maintenance PR Cadence

| Cadence | Required Recurring PR / Task | Output & Verification Criteria |
|---|---|---|
| **Every PR** | `ci(cli): run command contract checks for changed tools` | Formatting, compile/tests, CLI golden tests, JSON schema checks, and security checks relevant to the command; skipped checks explained. |
| **Weekly** | `maint(cli): review command failures and stale backend assumptions` | New issues assigned to an owner; unsupported services are not represented as functional; help/status text remains truthful. |
| **Weekly** | `docs(cli): sync command registry and help examples` | Registry, generated help, examples, and docs stay aligned with the command parser. |
| **Monthly** | `test(cli): run command compatibility and output matrix` | Verify options, `--`, stdin/stdout/stderr, exit status, JSON, no-color, and non-TTY behavior for each supported command. |
| **Monthly** | `security(cli): review privileged and mutating commands` | Review authorization, confirmation, dry-run, path handling, transaction/rollback, and secret redaction. |
| **Monthly** | `audit(cli): review dependency and supply-chain changes` | Review parser/output dependencies, licenses, advisories, and lockfile changes; exceptions documented and time-bounded. |
| **Quarterly** | `docs(cli): review POSIX/Linux/BSD compatibility claims` | Reassess utility syntax, status conventions, options, and documented deviations; avoid implying binary or command compatibility. |
| **Quarterly** | `accessibility(cli): review terminal usability` | Keyboard-only discoverability, screen-reader-friendly output, color-independent statuses, wrapping, and localization issues. |
| **Each Release Candidate** | `release(cli): certify command support claims` | Tested build/revision/target, command matrix, help/manual generation, known limitations, and release output samples. |
| **Post Incident** | `fix(cli): prevent recurrence of <command issue>` | Minimal reproducer, regression test, affected versions, severity, and docs/error-message update. |

---

## 5. Rules for Command Families

1. **System Information**: Label every data source (host, kernel model, or SigmaOS runtime). Do not infer SigmaOS hardware or service state from the build host.
2. **Package Management**: Separate query operations from mutation; verify signed metadata and packages; show transaction plans; support atomic recovery/rollback.
3. **Networking**: Keep status commands read-only; make apply/rollback explicit; avoid printing credentials or Wi-Fi secrets; expose unsupported protocols as unavailable.
4. **Services and Processes**: Show authoritative runtime state; define start/stop permissions and timeout behavior; do not report success until the service manager confirms state change.
5. **Storage and Installer**: Require exact target identification and confirmation; default to dry-run; keep inspection separate from formatting and partitioning.
6. **Security & Audit**: Report findings with severity, source, and remediation status; redact tokens, keys, command history, and private paths unless explicitly requested.
7. **Logs**: Bound output and provide time/source filters; preserve raw logs in machine-readable mode; use stable ISO-8601 timestamps and keep stderr separate from structured output.
8. **JSON Output**: Version schemas (`schema_version`); keep keys stable; send warnings/diagnostics to stderr or structured fields; do not switch output format automatically based on TTY presence alone.
9. **Help & Errors**: Usage errors should display concise syntax and point to detailed help; failures should indicate which operation failed and what was attempted; avoid returning raw error counts as exit status codes.
10. **Man Pages**: Include standard sections: `NAME`, `SYNOPSIS`, `DESCRIPTION`, `OPTIONS`, `ENVIRONMENT`, `EXIT STATUS`, `EXAMPLES`, `COMPATIBILITY`, `STANDARDS`, `SEE ALSO`, and limitations. Lint and render pages in CI.

---

## 6. PR Template and Merge Gates

All pull requests modifying or adding CLI commands must utilize the standard PR template structure:

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

### Review Approvals Requirements
- **1 Maintainer Approval**: Required for non-mutating utilities, documentation, or help updates.
- **2 Maintainer Approvals**: Required for privileged operations, package mutations, network changes, storage/installer mutations, or security-sensitive output changes.

---

## 7. Rollout and Exit Criteria

- **Proposed**: Command specified in the registry roadmap with defined arguments, output schema, and owner.
- **Prototype**: Initial command implementation exists in code with unit or model tests, but runtime backend integration is pending.
- **Integrated**: Command invokes the real SigmaOS runtime backend and passes test suites on a documented boot target (e.g., QEMU or native).
- **Supported**: Command contract, manual page, permission checks, error handling paths, JSON schema, and release test matrix are all repeatable and validated in CI.
