# SigmaOS CLI Development and Maintenance Plan

## Overview & Architecture
This plan focuses on establishing a coherent, scriptable, robust Command-Line Interface (CLI) for SigmaOS that operates effectively from terminal environments and remains productive before and alongside the graphical desktop.

To prevent architectural ambiguity and avoid premature assumptions about active runtime services, SigmaOS strictly distinguishes between two distinct CLI layers:
1. **System CLI Utilities:** Standalone administration, inspection, configuration, package management, and diagnostic utilities (e.g., `sigma`, `sigma-info`, `sigpkg`, `sigmanet`).
2. **Command Interpreter / Shell:** The shell environment (`sigma-sh` / `universal_cli_shell_system`) that parses command lines, handles quoting, variables, redirections, pipelines, built-ins, and manages process execution and job control.

The CLI roadmap explicitly avoids assuming that process execution, pipelines, or graphical desktop services are fully operational. Each utility must degrade gracefully, report honest status errors, and operate independently of shell capabilities.

---

## CLI Rules and Compatibility Profile

1. **POSIX Utility Conventions Baseline:**
   Use POSIX.1-2024 utility guidelines as the baseline for options (`-a`), flags, positional operands, and the `--` delimiter (end-of-options). State deviations explicitly in documentation.

2. **Consistent Discoverability:**
   Every standalone utility supports `--help` and `--version`. For sub-command trees (such as `sigma`), both `sigma help <command>` and `sigma <command> --help` must present consistent usage guidance. Follow GNU Coding Standards for option naming and standard help output.

3. **Machine-Readable Standard Output:**
   Human-readable summaries are output to `stdout` by default. Errors, warnings, and diagnostic telemetry are emitted to `stderr`. For scriptable tools, supply deterministic, schema-versioned JSON output when `--json` or `--format=json` is requested.

4. **Typed APIs Over Shell-String Composition:**
   Administrative tools must interact with system services and kernel interfaces via typed IPC, syscalls, or structured Rust APIs. Never format user arguments into raw shell command strings or execute privileged shell sub-shells.

5. **Explicit and Recoverable Mutating Operations:**
   Destructive actions (e.g., package removal, disk formatting, service termination, network changes) require explicit interactive confirmation or a `--yes` / `--force` flag. Where applicable, provide `--dry-run` / `--preview` modes and clear status separation between read-only diagnostics and stateful mutations.

6. **Honest Status Reporting (No False Active States):**
   If a backend service, kernel driver, or subsystem is unavailable or un-implemented, tools must fail closed and return an explicit, actionable "not supported / service unreachable" error with an exit status of 3 (`ENOSYS` / `UNAVAILABLE`). Simulated success is strictly prohibited.

7. **Terminal and Non-Terminal Usability:**
   Do not rely on ANSI color for critical meaning. Disable color output automatically when `stdout` is not a TTY or when `NO_COLOR` is present in the environment. Never require an interactive TTY for script execution.

8. **Comprehensive Documentation:**
   Every public command installed in userspace must have manual pages (`man`) structured according to standard UNIX sections: `NAME`, `SYNOPSIS`, `DESCRIPTION`, `OPTIONS`, `ENVIRONMENT`, `EXIT STATUS`, `EXAMPLES`, `COMPATIBILITY`, and `KNOWN LIMITATIONS`.

9. **Untrusted Input Handling:**
   Validate all numbers, filesystem paths, identifiers, and configuration inputs against standard limits. Avoid leaking credentials, auth tokens, or private paths in shell history, error streams, or diagnostic reports.

10. **Reviewable PRs and Protected Gates:**
    All CLI modifications require atomic, focused PRs, unit/golden tests, maintainer code review, and passing CI gates. Privileged operations require double maintainer review.

---

## 18-PR Implementation Sequence Roadmap

| PR | Title | Scope & Objectives | Acceptance Criteria | Stage |
|---|---|---|---|---|
| **1** | `docs(cli): define CLI architecture and command compatibility profile` | Audit `src/tools/`, `src/userspace/`, shell, package manager, installer, and diagnostics. Define command naming, POSIX profile, status codes, and unsupported-feature policy. | Shell functionality separated from utilities; all existing commands cataloged with actual runtime status; no unverified claims. | Stage 1: Foundation |
| **2** | `docs(cli): publish command registry and CLI contribution rules` | Add command registry specifying name, purpose, privilege requirements, mutability, API backend, arguments, output schema, and code owners. | Every command mapped to an owner and acceptance tests; duplicate names and conflicting aliases eliminated. | Stage 1: Foundation |
| **3** | `feat(cli): add shared argument, diagnostics, and output framework` | Implement reusable Rust helpers for POSIX argument parsing, standard usage text, error formatting, standard exit codes, JSON serialization, and NO_COLOR/TTY detection. | Golden tests cover valid/invalid args, machine output, stderr routing, NO_COLOR/non-TTY handling, and exit codes. | Stage 1: Foundation |
| **4** | `feat(cli): add sigma command tree and help discovery` | Build the top-level `sigma` command router with subcommands and nested help discovery; maintain usability regardless of backend availability. | `sigma --help`, `sigma help`, and `sigma help <subcommand>` work cleanly; unknown subcommands suggest fuzzy matches; unavailable backends fail with clear error. | Stage 1: Foundation |
| **5** | `feat(cli): add version and build-information command` | Expose build version, source git revision, target architecture, and feature status profile without leaking build secrets. | Output is deterministic in text and versioned JSON; generated from build metadata; `--version` returns exit code 0. | Stage 1: Foundation |
| **6** | `feat(cli): integrate system information report command` | Convert host/system report tools into `sigma info` / `sigma system report`; distinguish host data from SigmaOS native runtime metrics. | Golden tests verify report source tagging; host host metrics never falsely reported as SigmaOS kernel metrics. | Stage 1: Foundation |
| **7** | `feat(cli): add read-only system status and diagnostics commands` | Add `sigma status`, `sigma hardware`, `sigma logs` (read-only), and health summaries using typed data providers. | Providers show source and freshness; unsupported subsystems explicitly marked unavailable; bounded logs support stable JSON. | Stage 2: Diagnostics & Operations |
| **8** | `feat(cli): add package query commands` | Add `sigma pkg search`, `sigma pkg list`, and `sigma pkg info` over repository metadata; query-only (no install/remove). | Search/list results scriptable and stable; metadata validated; package availability strictly distinguished from installation state. | Stage 2: Diagnostics & Operations |
| **9** | `feat(cli): add package install and removal with transaction preview` | Implement package mutation after signature verification (Dilithium5/Signify), dependency resolution, permission checks, and rollback integration. | Previews changes; enforces signature provenance; failure triggers atomic rollback; destructive removal requires explicit confirmation (`--yes`). | Stage 2: Diagnostics & Operations |
| **10** | `feat(cli): add network status and safe configuration commands` | Add read-only interface/address/route/DNS status first; enable mutation only after transactional network config and rollback exist. | Read-only mode cannot mutate state; configuration validated prior to apply; rollback path tested; unsupported protocols reported cleanly. | Stage 2: Diagnostics & Operations |
| **11** | `feat(cli): add service status and lifecycle commands` | Expose service supervisor status, `start`, `stop`, and `restart` once process spawning and dependency tracking are operational. | Dependency failures and permissions visible; commands do not claim service started unless runtime confirms; restart/stop timeouts tested. | Stage 2: Diagnostics & Operations |
| **12** | `feat(cli): add storage and installer inspection commands` | Add read-only block device, partition, filesystem mount, and installation target inspection. High-risk formatting/partitioning handled separately. | Device paths resolved via immutable UUIDs; zero destructive ops in inspection mode; target drive clearly displayed prior to any mutation. | Stage 2: Diagnostics & Operations |
| **13** | `feat(cli): add shell completions and command suggestions` | Generate completion scripts for `bash`, `zsh`, `fish`, and `sigma-sh` directly from command registry; provide typo suggestions. | Completions use command registry as single source of truth; hidden/privileged commands handled safely; offline execution guaranteed. | Stage 3: Polishing & Hardening |
| **14** | `docs(cli): generate manuals and examples from command metadata` | Add automated man-page reference generation and practical command examples. | Manuals match parser options; pages pass `mandoc` linter; every public utility documents exit statuses and examples. | Stage 3: Polishing & Hardening |
| **15** | `test(cli): add command behavior and compatibility harness` | Add golden tests for help/version, argument syntax, exit codes, stdout/stderr isolation, JSON schemas, and non-TTY behavior. | Tests run deterministically in CI; test vectors document target profile; intentional POSIX deviations explicitly recorded. | Stage 3: Polishing & Hardening |
| **16** | `security(cli): audit privilege boundaries and sensitive output` | Audit privileged command handlers, auth/confirmation flows, path traversal guards, token redaction, and error outputs. | No authorization granted via UI checks; sensitive data redacted in logs/reports; security checks integrated into CI release gates. | Stage 3: Polishing & Hardening |
| **17** | `ci(cli): add command matrix and release verification` | Test commands across hosted development environments and QEMU boot targets; run output schema regression tests. | CI explicitly labels test environment backend; host-only passes do not count as native kernel evidence; release claims match target test. | Stage 3: Polishing & Hardening |
| **18** | `docs(cli): publish CLI support matrix and troubleshooting guide` | Document supported commands/backends, target requirements, permission matrices, exit codes, and known gaps. | Every "Integrated" or "Supported" command links to test vectors; error messages include actionable diagnostic steps. | Stage 3: Polishing & Hardening |

---

## Periodic Maintenance Cadence

- **Every PR:** Run formatting, compilation, CLI golden tests, JSON schema validation, and security path audits.
- **Weekly:** Review command error rates, stale backend assumptions, and sync command registry help examples.
- **Monthly:** Execute full CLI compatibility & output matrix tests (TTY, non-TTY, NO_COLOR, JSON schemas); review privileged/mutating command security.
- **Quarterly:** Reassess POSIX/Linux/BSD utility compatibility claims and conduct terminal usability/accessibility reviews.
- **Each Release Candidate:** Certify command support claims against physical or QEMU boot targets with generated release test matrices.

---

## Rules for Command Families

1. **System Information:** Explicitly tag all data sources (`Host Environment`, `Model / Prototype`, `SigmaOS Native Kernel`).
2. **Package Management:** Read-only queries separated from stateful mutations. All updates require cryptographic signature verification and transactional rollback.
3. **Networking:** Read-only inspection default. Applying configuration requires explicit validation and auto-rollback timers. Secrets (Wi-Fi passwords, private keys) are redacted.
4. **Services and Processes:** Query real runtime supervisor state (`systemd`, `openrc`, `runit`, `s6`). Do not claim success until supervisor state changes.
5. **Storage and Installer:** Enforce UUID/ID device targeting. Confirm target drive before write operations. Default to dry-run.
6. **Security and Audit:** Redact tokens, keys, private paths, and passwords in telemetry and output reports.
7. **Logs:** Bounded output buffer limits. Support time range and source filtering. Keep JSON schema separate from stderr text stream.
8. **JSON Output:** Version schemas (`schema_version: "1.0"`). Standardize status fields (`status: "ok" | "error"`).
9. **Help and Error Messages:** Show concise syntax on usage errors; direct user to detailed help (`sigma help <subcommand>`). Standardize exit statuses (0=success, 1=general error, 2=invalid args, 3=unavailable backend, 126=permission denied, 127=command not found).

---

## Standard PR Template & Merge Gates

```markdown
## Summary
What command behavior or operational task does this PR add or fix?

## Scope
Command(s), backend(s), privileges required, and explicit non-goals.

## CLI Contract
Synopsis, options, operands, stdin/stdout/stderr, exit statuses, human/JSON output format, and POSIX profile.

## Backend and Runtime Evidence
Hosted | Unit / Model | QEMU SigmaOS Target | Physical Bare-Metal Target

## Safety and Security
Read-only vs Mutating; authorization, confirmation flags, dry-run, rollback, path validation, secret redaction.

## Tests and Documentation
Golden tests, schema tests, manual pages, usage examples.

## Support Status
Proposed | Prototype | Integrated | Supported
```

---

## Rollout Criteria & Support Matrix Vocabulary

- **Proposed:** Documented plan and interface contract; no code implementation.
- **Prototype:** In-memory model or standalone test logic; incomplete backend connection.
- **Integrated:** Connected to actual SigmaOS runtime services; passes automated test suite in QEMU or hosted target.
- **Supported:** Fully integrated, documented with manual pages, schema-validated, audited for privilege boundaries, and verified on release boot target.
