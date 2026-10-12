# SigmaOS CLI Development Plan

## Overview and Architecture Layering

SigmaOS requires a coherent, scriptable Command Line Interface (CLI) that functions reliably from a terminal environment before and alongside graphical desktop interfaces. To ensure clear architectural boundaries, the CLI environment is split into two distinct layers:

1. **System CLI Utilities**: Standalone administrative and diagnostic executables that query hardware, inspect packages, observe service states, and configure system parameters.
2. **Shell Engine**: The interactive command interpreter that parses command lines, handles pipelines, manages jobs, and spawns processes.

### Grounding Rules and Runtime Realism
- Do not assume that shell pipelines, process execution, or graphical services already work in all target environments.
- Distinguish between hosted development environments (running on Linux/macOS) and bare-metal/QEMU SigmaOS target runtimes.
- Avoid false representation: if an OS backend is not connected or active, tools must return explicit "not supported / backend unavailable" error statuses rather than mock success.

---

## CLI Rules and Compatibility Goals

1. **POSIX Utility Baseline**: Use POSIX utility conventions as the portability baseline. Define explicit syntax, options, operands, `--` end-of-options behavior, standard input/output handling, and exit statuses for every utility.
2. **Discoverability**: Provide `--help` and `--version` for standalone tools. For subcommand trees, support `sigma help <subcommand>` and `<subcommand> --help`. Follow GNU Coding Standards for standard help/version options and long-option naming.
3. **Machine-Readable Output**: Human-readable output goes to `stdout` by default; diagnostics and errors go to `stderr`. Provide stable JSON output with versioned schemas when requested via `--json` or `--format=json`.
4. **Typed Service APIs**: Administrative commands must call typed system services, Rust traits, or system calls. User input must never be passed into constructed shell strings or unvalidated system commands.
5. **Recoverable & Explicit Mutating Actions**: Require interactive confirmation or explicit `--yes` flags for state-mutating or destructive actions. Provide `--dry-run` or preview modes where applicable. Distinguish read-only inspection from state mutations.
6. **Truthful Feature Reporting**: Never pretend an unintegrated feature is active. Return actionable "backend unavailable" status codes when system services are not connected.
7. **Terminal Usability & TTY Handling**: Support `NO_COLOR` and disable ANSI formatting automatically when `stdout` is not a TTY. Never require a terminal for non-interactive scripts.
8. **Documentation Standards**: Document every public command with standard manual sections (NAME, SYNOPSIS, DESCRIPTION, OPTIONS, ENVIRONMENT, EXIT STATUS, EXAMPLES, COMPATIBILITY, STANDARDS, SEE ALSO).
9. **Security & Input Validation**: Treat all CLI input as untrusted. Validate numbers, paths, identifiers, and configuration inputs. Bound output and resource usage. Redact secrets, keys, and tokens from command history and diagnostics.
10. **Reviewable PRs and Merge Gates**: Require focused pull requests, test coverage, code owner review, and passing CI checks prior to merging.

---

## Proposed PR Sequence

| PR | Proposed Title | Scope | Acceptance Criteria |
| :--- | :--- | :--- | :--- |
| **1** | `docs(cli): define CLI architecture and command compatibility profile` | Audit `src/tools/`, `src/userspace/`, shell, package manager, installer, diagnostics, and docs. Define command naming, interface layers, POSIX profile, status codes, output modes, and unsupported-feature behavior. | Inventory separates shell functionality from utilities; every existing command is listed with actual implementation/runtime status; no command is called supported without evidence. |
| **2** | `docs(cli): publish command registry and CLI contribution rules` | Add a command registry describing name, purpose, privilege, mutability, API backend, arguments, output schema, and owner. | Every proposed command has an owner and acceptance tests; duplicate names and inconsistent aliases are detected or documented. |
| **3** | `feat(cli): add shared argument, diagnostics, and output framework` | Add reusable Rust helpers for option parsing, usage text, error formatting, exit status, JSON serialization, color/TTY detection, and `--` handling. | Golden tests cover valid/invalid args, machine output, stderr, no-color/non-TTY behavior, and exit codes; framework does not force one parser dependency without license/MSRV review. |
| **4** | `feat(cli): add sigma command tree and help discovery` | Build a top-level `sigma` command with subcommands and nested help; keep it usable before all backends are implemented. | `sigma --help`, `sigma help`, and `sigma help <subcommand>` work; unknown commands suggest valid names; unavailable backends fail clearly. |
| **5** | `feat(cli): add version and build-information command` | Expose version, source revision, build target, and feature profile without leaking build secrets or unsupported release claims. | Output is deterministic in text and JSON; values are generated from build metadata; `--version` exits successfully. |
| **6** | `feat(cli): integrate system information report command` | Turn the existing hosted report tool into a `sigma system info` command; preserve text/JSON/Markdown exports and add a clear distinction between host data and SigmaOS runtime data. | Tests prove report source and target; running on Linux does not misrepresent host metrics as SigmaOS; unavailable metrics are explicitly marked. |
| **7** | `feat(cli): add read-only system status and diagnostics commands` | Add `sigma status`, `sigma hardware`, `sigma logs` (read-only), and health summaries using typed providers. | Commands show provider freshness/source; no unsupported subsystem is reported as healthy; output supports stable JSON and bounded logs. |
| **8** | `feat(cli): add package query commands` | Add `sigma pkg search/list/info` over the repository/package metadata; do not install/remove packages yet. | Search/list results are stable and scriptable; repository metadata is validated; package availability is distinct from installed status. |
| **9** | `feat(cli): add package install and removal with transaction preview` | Add package mutation only after package signing, dependency resolution, permissions, and rollback behavior are integrated. | Preview lists changes; signature/provenance verification is enforced; failure rolls back or leaves a documented recoverable state; destructive removal requires explicit confirmation. |
| **10** | `feat(cli): add network status and safe configuration commands` | Add read-only interface/address/route/DNS status first; add mutation only after network configuration and rollback are integrated. | Read-only mode never changes state; configuration validates before apply; rollback path tested; unsupported network stack features are reported honestly. |
| **11** | `feat(cli): add service status and lifecycle commands` | Expose service-manager status and controlled start/stop/restart once process spawning and service dependency enforcement are real. | Dependency failures and permissions are visible; commands do not claim services started unless the runtime confirms; stop/restart behavior is tested. |
| **12** | `feat(cli): add storage and installer inspection commands` | Add read-only block-device, partition, mount, and installation status; keep partitioning/formatting in separate high-risk PRs. | Device paths are resolved through trusted IDs; no destructive operation occurs in inspection commands; exact device/partition target is shown before any future mutation. |
| **13** | `feat(cli): add shell completions and command suggestions` | Generate completion for supported shells from the command registry; add typo suggestions without executing commands. | Completion uses the same source of truth as help; hidden/privileged commands are handled appropriately; completion does not query unstable network services. |
| **14** | `docs(cli): generate manuals and examples from command metadata` | Add man-page/reference generation and command examples; choose project-supported markup after checking installed tooling. | Generated docs match parser options; man pages lint and render; every public command documents exit status and examples. |
| **15** | `test(cli): add command behavior and compatibility harness` | Add golden tests for help/version, argument syntax, exit codes, stdout/stderr, JSON schemas, and non-TTY behavior; compare POSIX-like utilities only where compatibility is claimed. | Tests run deterministically in CI; test vectors document the target profile; intentional deviations are recorded. |
| **16** | `security(cli): audit privilege boundaries and sensitive output` | Audit privileged command handlers, auth/confirmation flows, filesystem path handling, secrets, logs, and generated reports. | No privilege comes from UI-only checks; secrets are redacted; read-only commands are demonstrably read-only; security checks are part of required CI. |
| **17** | `ci(cli): add command matrix and release verification` | Test commands against hosted development environment and, once available, the SigmaOS QEMU boot target. Add output-schema compatibility checks and release smoke tests. | CI identifies which backend was tested; host-only tests never count as SigmaOS runtime evidence; release claims match the tested target. |
| **18** | `docs(cli): publish CLI support matrix and troubleshooting guide` | Document supported command/backends, target environment, examples, exit codes, permissions, and known gaps. | Every "Integrated" or "Supported" command links to tests and a known runtime; errors include next actionable diagnostic without claiming unavailable features. |

---

## Periodic Maintenance PR Cadence

| Cadence | Task | Required Output |
| :--- | :--- | :--- |
| **Every PR** | `ci(cli): run command contract checks for changed tools` | Formatting, compile/tests, CLI golden tests, JSON schema checks, security checks. |
| **Weekly** | `maint(cli): review command failures and stale backend assumptions` | Issue assignments, updated backend statuses, truthful help/status text. |
| **Weekly** | `docs(cli): sync command registry and help examples` | Registry, generated help, and docs aligned with command parser. |
| **Monthly** | `test(cli): run command compatibility and output matrix` | Verification of options, `--`, stdin/stdout/stderr, exit status, JSON, no-color, and non-TTY behavior. |
| **Monthly** | `security(cli): review privileged and mutating commands` | Authorization, confirmation, dry-run, path handling, rollback, secret redaction audit. |
| **Monthly** | `audit(cli): review dependency and supply-chain changes` | Parser/output dependencies, licenses, security advisories, lockfile review. |
| **Quarterly** | `docs(cli): review POSIX/Linux/BSD compatibility claims` | Reassess utility syntax, status conventions, options, and documented deviations. |
| **Quarterly** | `accessibility(cli): review terminal usability` | Keyboard discoverability, screen-reader friendly output, color-independent status, wrapping. |
| **Release Candidate** | `release(cli): certify command support claims` | Tested build/revision/target matrix, manual generation, release output samples. |
| **Post-Incident** | `fix(cli): prevent recurrence of <command issue>` | Minimal reproducer, regression test, version impact, documentation fix. |

---

## Rules for Command Families

1. **System Information**: Label every data source explicitly (host vs kernel model vs SigmaOS runtime). Never infer SigmaOS hardware or service state from the build host.
2. **Package Management**: Separate query operations from mutations. Verify signed metadata and package signatures. Show transaction plans prior to execution and support recovery.
3. **Networking**: Keep status commands read-only. Make apply and rollback actions explicit. Redact credentials and Wi-Fi secrets.
4. **Services and Processes**: Display authoritative runtime state from the service supervisor. Do not report success until the service manager confirms state change.
5. **Storage and Installer**: Require explicit target identification (e.g. UUID, disk ID). Default to dry-run/preview. Keep inspection strictly separate from formatting or partitioning.
6. **Security & Audit**: Report findings with severity, source, and remediation guidance. Redact sensitive tokens, keys, and private paths.
7. **Logs**: Bound output volume and support time/source filters. Preserve raw logs in machine-readable JSON format without stdout/stderr contamination.
8. **JSON Output**: Version schemas explicitly (`schema_version: "1.0"`). Maintain key stability. Output warnings or diagnostics to `stderr` or structured log fields.
9. **Help and Errors**: Errors show concise syntax and point to detailed help pages. Exit statuses use standard ranges without relying on exit codes as error counters.
10. **Man Pages**: Every utility maintains standard manual pages linted and rendered in CI pipelines.

---

## PR Template and Merge Gates

```markdown
## Summary
What command behavior or operational task does this PR add or fix?

## Scope
Command(s), backend(s), privileges, and what is explicitly not implemented.

## CLI Contract
Synopsis, options, operands, stdin/stdout/stderr, exit statuses, human/JSON output, and compatibility profile.

## Backend and Runtime Evidence
Hosted | unit/model | QEMU SigmaOS | physical target
State the data source and exact test environment.

## Safety and Security
Read-only or mutating; authorization, confirmation, dry-run, rollback, input validation, path handling, and secret redaction.

## Tests and Documentation
Commands run, golden outputs, schema tests, manual pages, examples.

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

### Review & Merge Requirements
- **1 Maintainer Approval**: Required for standard read-only commands and documentation updates.
- **2 Maintainer Approvals**: Required for privileged operations, package installation/removal, network mutations, storage mutations, or security-sensitive output.

---

## Rollout Criteria

- **Integrated**: A CLI command is marked *Integrated* only when it invokes the real SigmaOS runtime backend and passes automated tests on a documented boot target (QEMU or hardware).
- **Supported**: A command is marked *Supported* only after its command contract, manual page, permissions, error paths, JSON schema, and release test matrix are repeatable in CI.
- **Prototype / Hosted**: Commands without active runtime backends must be clearly labeled as *Prototype* or *Hosted*, explicitly marking host-derived data as non-native.
