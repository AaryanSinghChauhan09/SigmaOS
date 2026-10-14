# SigmaOS CLI Development Plan (Idea #32)

## Architectural Overview & Scope

A review of SigmaOS’s public `README.md`, `FEATURE_STATUS.toml`, and project wiki demonstrates that while the repository includes a hosted system-information report tool exporting text, JSON, and Markdown, it lacks deep SigmaOS runtime or GUI integration. Its inventory describes the current shell as a prototype with basic built-ins (`echo`, `pwd`, `ls`) awaiting real process execution. Furthermore, official documentation clarifies that SigmaOS is not yet documented as a supported general-purpose installation and that an ISO build or listed subsystem is not proof of a fully operational system.

This plan defines a coherent, scriptable, production-grade CLI interface operating from a terminal, maintaining maximum utility both before and after graphical desktop availability.

### Two-Layer Architecture Model

1. **System CLI Utilities Layer**: Standalone administrative and diagnostic executables and command trees (e.g., `sigma`, `sigma-pkg`, diagnostics) providing system control, system monitoring, configuration, and package management.
2. **Command Interpreter & Shell Layer**: Interactive TTY environment and non-interactive shell engine (`sigma-sh`) responsible for AST parsing, token expansion, job control, process spawning, pipelines, and signal dispatch.

The CLI roadmap explicitly decouples command utility contracts from shell capabilities, ensuring utilities function via typed system APIs regardless of whether pipelines or process execution are handled by a specific shell.

---

## CLI Rules & Compatibility Goals

1. **POSIX Baseline**: Adhere to POSIX.1-2024 utility conventions for syntax, option parsing (`-a`, `-b`), long options (`--option`), positional operands, and the `--` end-of-options delimiter. Standard input, standard output, standard error, and exit statuses (`0` for success, `>0` for error) are strictly defined. Any deviation must be explicitly documented.
2. **Consistent Discoverability**: Standalone utilities support `--help` and `--version`. Command trees support `sigma help <command>` and `sigma <command> --help`. GNU Coding Standards recommendations for standard long options and command discovery are strictly enforced.
3. **Machine-Readable Output**: Human-friendly formatting is printed to `stdout` by default; diagnostic warnings and error messages are written exclusively to `stderr`. For automated scripting, commands support versioned, stable JSON output schemas (`--json`).
4. **Typed APIs & Zero Shell-String Composition**: Administrative commands must invoke typed IPC interfaces, kernel system calls, or typed backend Rust services. User arguments must never be concatenated into shell command strings or executed via subshells.
5. **Explicit & Recoverable Mutations**: Destructive or state-altering operations require confirmation or an explicit `--yes`/`--force` flag. Where applicable, utilities provide `--dry-run` previews. Read-only diagnostic tools are strictly segregated from mutating commands.
6. **Honest Runtime State Reporting**: If an underlying OS runtime service or kernel backend is unavailable, the tool must return an actionable "not supported/not connected" error rather than simulating success or returning fake data.
7. **Terminal Usability & TTY Awareness**: Usability must not depend on color alone. Color output is automatically disabled when `stdout` is not a TTY or when `NO_COLOR` is set. Non-interactive scripted execution must never fail due to lack of a TTY.
8. **Comprehensive Documentation**: Every installed command must be accompanied by a manual reference page documenting Synopsis, Description, Options, Environment Variables, Exit Status, Examples, Compatibility, and Known Limitations.
9. **Untrusted Input Handling**: All CLI input (numeric parameters, paths, identifiers, and configuration directives) must be strictly validated and bounded. Secrets, API keys, and private credentials must never be leaked in command history, stdout logs, or diagnostic reports.
10. **Protected Branch & Code Review Standards**: All changes must be submitted via focused pull requests requiring passing CI, maintainer reviews, and automated contract tests before merging.

---

## Proposed PR Sequence (PR 1 - PR 18)

| PR | Title | Scope | Acceptance Criteria | Status |
|---|---|---|---|---|
| **PR 1** | `docs(cli): define CLI architecture and command compatibility profile` | Audit `src/tools/`, `src/userspace/`, shell, package manager, installer, diagnostics, and docs. Define command naming, interface layers, POSIX profile, status codes, output modes, and unsupported-feature behavior. | Inventory separates shell functionality from utilities; every existing command is listed with actual implementation/runtime status; no command is called supported without evidence. | Integrated |
| **PR 2** | `docs(cli): publish command registry and CLI contribution rules` | Add a command registry describing name, purpose, privilege, mutability, API backend, arguments, output schema, and owner. | Every proposed command has an owner and acceptance tests; duplicate names and inconsistent aliases are detected or documented. | Integrated |
| **PR 3** | `feat(cli): add shared argument, diagnostics, and output framework` | Add reusable Rust helpers for option parsing, usage text, error formatting, exit status, JSON serialization, color/TTY detection, and `--` handling. | Golden tests cover valid/invalid args, machine output, stderr, no-color/non-TTY behavior, and exit codes; framework does not force one parser dependency without license/MSRV review. | Integrated |
| **PR 4** | `feat(cli): add sigma command tree and help discovery` | Build a top-level `sigma` command with subcommands and nested help; keep it usable before all backends are implemented. | `sigma --help`, `sigma help`, and `sigma help <subcommand>` work; unknown commands suggest valid names; unavailable backends fail clearly. | Integrated |
| **PR 5** | `feat(cli): add version and build-information command` | Expose version, source revision, build target, and feature profile without leaking build secrets or unsupported release claims. | Output is deterministic in text and JSON; values are generated from build metadata; `--version` exits successfully. | Integrated |
| **PR 6** | `feat(cli): integrate system information report command` | Turn the existing hosted report tool into a `sigma system info` command; preserve text/JSON/Markdown exports and add a clear distinction between host data and SigmaOS runtime data. | Tests prove report source and target; running on Linux does not misrepresent host metrics as SigmaOS; unavailable metrics are explicitly marked. | Integrated |
| **PR 7** | `feat(cli): add read-only system status and diagnostics commands` | Add `sigma status`, `sigma hardware`, `sigma logs` (read-only), and health summaries using typed providers. | Commands show provider freshness/source; no unsupported subsystem is reported as healthy; output supports stable JSON and bounded logs. | Integrated |
| **PR 8** | `feat(cli): add package query commands` | Add `sigma pkg search/list/info` over the repository/package metadata; do not install/remove packages yet. | Search/list results are stable and scriptable; repository metadata is validated; package availability is distinct from installed status. | Proposed |
| **PR 9** | `feat(cli): add package install and removal with transaction preview` | Add package mutation only after package signing, dependency resolution, permissions, and rollback behavior are integrated. | Preview lists changes; signature/provenance verification is enforced; failure rolls back or leaves a documented recoverable state; destructive removal requires explicit confirmation. | Proposed |
| **PR 10** | `feat(cli): add network status and safe configuration commands` | Add read-only interface/address/route/DNS status first; add mutation only after network configuration and rollback are integrated. | Read-only mode never changes state; configuration validates before apply; rollback path tested; unsupported network stack features are reported honestly. | Proposed |
| **PR 11** | `feat(cli): add service status and lifecycle commands` | Expose service-manager status and controlled start/stop/restart once process spawning and service dependency enforcement are real. | Dependency failures and permissions are visible; commands do not claim services started unless the runtime confirms; stop/restart behavior is tested. | Proposed |
| **PR 12** | `feat(cli): add storage and installer inspection commands` | Add read-only block-device, partition, mount, and installation status; keep partitioning/formatting in separate high-risk PRs. | Device paths are resolved through trusted IDs; no destructive operation occurs in inspection commands; exact device/partition target is shown before any future mutation. | Proposed |
| **PR 13** | `feat(cli): add shell completions and command suggestions` | Generate completion for supported shells from the command registry; add typo suggestions without executing commands. | Completion uses the same source of truth as help; hidden/privileged commands are handled appropriately; completion does not query unstable network services. | Proposed |
| **PR 14** | `docs(cli): generate manuals and examples from command metadata` | Add man-page/reference generation and command examples; choose project-supported markup after checking installed tooling. | Generated docs match parser options; man pages lint and render; every public command documents exit status and examples. Dependency adoption reviewed. | Proposed |
| **PR 15** | `test(cli): add command behavior and compatibility harness` | Add golden tests for help/version, argument syntax, exit codes, stdout/stderr, JSON schemas, and non-TTY behavior; compare POSIX-like utilities only where compatibility is claimed. | Tests run deterministically in CI; test vectors document the target profile; intentional deviations are recorded. | Proposed |
| **PR 16** | `security(cli): audit privilege boundaries and sensitive output` | Audit privileged command handlers, auth/confirmation flows, filesystem path handling, secrets, logs, and generated reports. | No privilege comes from UI-only checks; secrets are redacted; read-only commands are demonstrably read-only; security checks are part of required CI. | Proposed |
| **PR 17** | `ci(cli): add command matrix and release verification` | Test commands against hosted development environment and, once available, the SigmaOS QEMU boot target. Add output-schema compatibility checks and release smoke tests. | CI identifies which backend was tested; host-only tests never count as SigmaOS runtime evidence; release claims match the tested target. | Proposed |
| **PR 18** | `docs(cli): publish CLI support matrix and troubleshooting guide` | Document supported command/backends, target environment, examples, exit codes, permissions, and known gaps. | Every "Integrated" or "Supported" command links to tests and a known runtime; errors include next actionable diagnostic without claiming unavailable features. | Proposed |

---

## Periodic Maintenance PR Cadence

- **Every PR**: `ci(cli)` - Run formatting, syntax, compilation, CLI contract golden tests, JSON schema checks, and security audit rules.
- **Weekly**: `maint(cli)` - Review command failure reports, stale backend assumptions, and ensure help texts reflect true capabilities.
- **Weekly**: `docs(cli)` - Synchronize command registry, generated `--help` screens, and documentation examples.
- **Monthly**: `test(cli)` - Run complete CLI compatibility matrix across TTY/non-TTY, color, `--json`, `--` parsing, and standard streams.
- **Monthly**: `security(cli)` - Audit privileged handlers, path handling, state mutation safety, dry-run previews, and secret redaction.
- **Monthly**: `audit(cli)` - Review option-parser dependencies, licenses, MSRV, advisories, and lockfile updates.
- **Quarterly**: `docs(cli)` - Reassess POSIX, Linux, and BSD utility compatibility claims, updating documented intentional deviations.
- **Quarterly**: `accessibility(cli)` - Evaluate non-TTY usability, screen reader output, color-independent state indicators, and line wrapping.
- **Each Release Candidate**: `release(cli)` - Formally certify command support claims against verified boot and QEMU test targets.
- **Post-Incident**: `fix(cli)` - Add minimal regression test vectors and update error messages following CLI issues.

---

## Rules for Command Families

1. **System Information**: Explicitly identify data sources (hosted Linux environment vs. native SigmaOS kernel runtime).
2. **Package Management**: Segregate query operations (`search`/`info`) from mutations (`install`/`remove`). Enforce package signature verification, preview transactions, and support atomic rollback.
3. **Networking**: Read-only status commands must never alter interface states. Mutations require explicit confirmation and automatic rollback timeout guards. Wi-Fi keys and tokens must be redacted.
4. **Services & Processes**: Query authoritative service manager state. Report lifecycle changes only after backend process manager confirmation.
5. **Storage & Installer**: Inspection tools default to non-mutating preview. Destructive partitioning or formatting commands require target UUID/path re-typing or explicit confirmation flags.
6. **Security & Audit**: Format findings with severity, source, and remediation instructions. Redact session tokens, private keys, and restricted file paths.
7. **Logs**: Provide time, unit, and severity filters. Output machine-readable JSON logs without mixing stdout human logs and structured data.
8. **JSON Output**: Version all JSON schemas. Emit warnings to `stderr` or structured error objects; never alter human output into JSON based solely on TTY auto-detection.
9. **Help & Error Handling**: Usage errors point to `--help` and display concise syntax summaries. Error exit status codes must not overflow 8-bit process status limits (`1..255`).
10. **Man Pages**: Include standard sections: `NAME`, `SYNOPSIS`, `DESCRIPTION`, `OPTIONS`, `ENVIRONMENT`, `EXIT STATUS`, `EXAMPLES`, `COMPATIBILITY`, `STANDARDS`, `SEE ALSO`, and `BUGS`.

---

## Rollout Criteria

- **Integrated**: Command calls real SigmaOS runtime backend and passes automated testing on a documented boot or QEMU target.
- **Supported**: Command contract, manual page, permission enforcement, error paths, JSON schema, and release test matrix are verified and repeatable across releases.
- **Prototype/Hosted**: Tools operating in hosted Linux environments or using simulated backends are explicitly labeled as `Hosted` or `Prototype` to ensure transparent capabilities.
