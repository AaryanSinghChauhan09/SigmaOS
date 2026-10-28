# SigmaOS Command Interpreter Development and Maintenance Plan

## Executive Summary & Background

SigmaOS defines a structured, evidence-based development and maintenance plan for the command interpreter (shell) subsystem. While `src/userspace/shell.rs` initially provided a prototype command runner with basic `echo`, `pwd`, and `ls` built-ins, full shell capabilities (pipelines, redirection, job control, expansion, and POSIX compliance) must earn their maturity status (`Prototype` -> `Integrated` -> `Supported`) through end-to-end evidence rather than code presence alone.

This document sets forth product and compatibility rules, a 17-PR implementation roadmap, periodic maintenance cadences, component-specific rules, PR review/merge gates, and rollout criteria.

---

## Product and Compatibility Rules

Before feature work, SigmaOS establishes a clear stance on shell strategy:
1. **Strategy**: Begin with a safe native command runner baseline providing limited syntax for interactive system management, then introduce a documented POSIX-oriented shell subset in explicit, tested phases.
2. **Compatibility Claims**: Do not advertise `sh` or POSIX compatibility until corresponding syntax, expansion, execution, and conformance tests pass in the target environment.

### Core Shell Rules

1. **Parse before executing**: Syntax errors must be identified before any part of the command executes where the defined grammar allows it. Never execute a valid prefix of a malformed pipeline or compound command.
2. **Preserve arguments exactly**: Represent arguments as structured values (`Vec<String>`); do not re-split expanded strings or invoke a host shell to execute commands. Quote-aware tokenization and expansion are required for correct argument boundaries.
3. **Use an AST, not ad hoc string splitting**: Enforce a strict architectural split between Lexer, Parser, Expander, Command Resolver, Executor, and Interactive Terminal Handler.
4. **Keep script and interactive modes distinct**: No prompts, history expansion, line editing, or interactive startup hooks in non-interactive script execution unless explicitly specified.
5. **Define behavior for unsupported syntax**: Emit a clear diagnostic error to stderr and return a non-zero exit status; never silently reinterpret unsupported syntax as a different command.
6. **Treat input and environment as untrusted**: Bound command length, token count, AST nesting depth, expansion output size, environment size, and history size. Never execute expanded strings as shell syntax unless command substitution is explicitly supported and safely parsed.
7. **No compatibility claims based on examples alone**: Every claimed feature must link to unit, parser, execution, and integration tests.
8. **Keep commands and privileged operations separate**: Built-ins that change system state must use typed internal interfaces and authorization checks; raw shell text must never be passed directly into a privileged service.

---

## Proposed PR Sequence (17-PR Gateway Roadmap)

| PR | Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **1** | `docs(shell): define command interpreter goals and compatibility profile` | Document target users, interactive vs script use, supported syntax, intended sh/POSIX status, built-in policy, limitations, and terminal assumptions. | A compatibility statement is published; "POSIX shell" is not claimed unless test criteria are defined; prototype status and missing features are listed. |
| **2** | `refactor(shell): split lexer parser expander and executor modules` | Replace monolithic parsing logic with distinct, testable components; introduce an AST for simple commands and operators. | Parser and executor can be tested independently; malformed syntax produces structured errors; existing built-ins continue to work without regression. |
| **3** | `test(shell): add golden lexer and parser regression suite` | Add fixtures for whitespace, quotes, escapes, operators, comments, multiline input, and malformed cases. Add property/fuzz targets for lexer/parser input. | Tests assert exact token/AST output; fuzz input cannot panic, hang, or exceed declared resource limits; every fixed parser bug becomes a regression case. |
| **4** | `feat(shell): implement command lookup and process execution contract` | Define environment, current directory, PATH lookup, executable invocation, standard streams, exit status, and error behavior. | `echo`, `pwd`, and `ls` work through the same documented command model; command-not-found and permission errors are distinct; process and file descriptor cleanup are tested. |
| **5** | `feat(shell): add variables assignments and safe quoting` | Add simple assignments, single/double quotes, escapes, and positional/environment variable handling according to the selected profile. | Tests cover empty values, spaces, quotes, variable boundaries, unset values, and expansion errors; argument arrays remain intact through execution. |
| **6** | `feat(shell): implement POSIX-style redirection primitives` | Add input (`<`), output (`>`), append (`>>`), error (`2>`), dup (`>&`), and combined (`&>`) redirection in a staged design; define ordering and descriptor semantics. | Tests cover missing paths, permissions, descriptor cleanup, redirection order, and failure before command execution; unsupported redirection forms fail explicitly. |
| **7** | `feat(shell): implement pipelines with bounded process and pipe lifecycle` | Implement `cmd1 \| cmd2` with correct input/output wiring, close-on-exec semantics, failure cleanup, and pipeline status policy. | Multi-command tests verify data flow, EOF, broken pipe, command failure, and resource cleanup; no deadlocks under documented limits. Output connects directly to next command input. |
| **8** | `feat(shell): define exit status and command-list operators` | Add command sequencing (`;`, `&&`, `\|\|`, `&`) and clearly documented status semantics (`$?`, short-circuiting). | Tests prove short-circuiting, last-command/pipeline status semantics, and signal-derived status behavior; no ambiguous fallback parsing. |
| **9** | `feat(shell): add environment and PATH built-ins` | Add required shell-state built-ins (`cd`, `export`, `unset`, `exit`, `alias`); define commands executing in current shell process. | State-changing built-ins affect current shell; invalid options and arguments return documented statuses; path/environment edge cases are covered. |
| **10** | `feat(shell): add script mode and startup policy` | Add script file execution and `-c` string mode, non-interactive behavior, script line error diagnostics, and startup-file policy. | Script mode emits no prompt or interactive history behavior; failures have deterministic status and line/column diagnostics; startup files are not sourced implicitly from unsafe locations. |
| **11** | `feat(shell): add control-flow subset for shell scripts` | Add `if/then/else`, `for/while/until` loops, functions, and `case` constructs in scoped PRs with grammar/execution fixed first. | Each construct has parser, execution, error, and integration tests; nested and malformed forms are handled predictably; supported subset is documented. |
| **12** | `feat(shell): add interactive line editor and bounded history` | Add terminal-aware input, cursor editing, history navigation/search, multiline prompts, and history persistence policy. | Works only when attached to a terminal; non-interactive mode is unaffected; history is size-bounded, permission-restricted, and avoids recording sensitive commands. |
| **13** | `feat(shell): add foreground job control and signal handling` | Implement foreground process groups, terminal ownership, interrupt/suspend behavior (`SIGINT`, `SIGTSTP`), and job table (`jobs`, `fg`, `bg`). | Tests cover Ctrl-C, suspend/resume, terminal restoration, shell exit with active jobs, and child cleanup in pseudo-terminal/QEMU; unsupported operations fail clearly. |
| **14** | `feat(shell): add command completion and help metadata` | Build completion from command/builtin metadata, filesystem entries, and documented argument providers; avoid executing candidate commands. | Completion is non-executing, bounded, permission-aware, and works with quoted/incomplete input; help text is generated from the same metadata. |
| **15** | `security(shell): enforce parser and expansion resource limits` | Add limits for source size, nesting, tokens, expansion output, environment, pipeline length, and open descriptors; audit command substitution and path resolution. | Exceeding a limit returns a diagnostic without kernel panic or partial privileged action; regression tests cover exhaustion and expansion attacks. |
| **16** | `test(shell): add POSIX subset and Linux BSD interoperability matrix` | Add differential tests for declared subset against reference shells (FreeBSD `sh`, GNU `bash`); classify differences as intentional or bugs. | Every test has a declared standard/profile; results are versioned; no broad compatibility claim is made from a small smoke-test set. |
| **17** | `docs(shell): publish command reference and maturity evidence` | Document grammar, built-ins, variables, status semantics, scripting, terminal behavior, security limits, and known gaps. Update capability inventory. | Every listed feature links to relevant tests; `Prototype`/`Integrated`/`Supported` status matches evidence; examples are executed in CI where practical. |

---

## Periodic Maintenance Cadences

| Cadence | Recurring Task | Required Output |
|---|---|---|
| **Every PR** | `ci(shell)`: run shell-specific checks for changed code | Formatting, compilation, parser regression tests, and relevant fuzz or integration checks; skipped checks explained. |
| **Weekly** | `test(shell)`: review fuzz crashes and parser regressions | Each crash is fixed, converted to a tracked issue with severity/owner, or documented as non-actionable; resolved crashes become regression tests. |
| **Weekly** | `docs(shell)`: reconcile feature status with implementation | Feature inventory and shell docs reflect tested behavior, not code presence; stale compatibility claims are removed. |
| **Monthly** | `security(shell)`: review command execution and expansion boundaries | Review token/AST handling, executable lookup, environment, path resolution, redirection, command substitution if implemented, and privileged built-ins. |
| **Monthly** | `test(shell)`: run script compatibility corpus | Deterministic tests for declared language subset; differences against Linux/BSD reference shells are classified and documented. |
| **Monthly** | `maint(shell)`: review terminal, signal, and child-process cleanup | PTY/integration tests for interrupted commands, terminal restoration, orphaned children, and descriptor leaks. |
| **Quarterly** | `docs(shell)`: review POSIX and reference-shell alignment | Recheck chosen POSIX edition and documented deviations; review FreeBSD `sh` and GNU `Bash` as behavior references without implying equivalent scope. |
| **Release Candidate** | `release(shell)`: certify shell support claims | Clean boot or supported test environment, interactive and script-mode smoke tests, regression suite, known limitations, exact target configuration. |
| **Post-Incident** | `fix(shell)`: prevent recurrence of incident | Root cause, minimized reproducer, regression test, impact assessment, and update to docs or resource limits if needed. |

---

## Rules for Shell-Related Components

1. **Lexer / Parser**: Preserve source line and column locations for diagnostics. Never use lossy token splitting. Enforce AST nesting depth and token input limits.
2. **Expansion**: Specify expansion order (brace -> tilde -> parameter -> command -> arithmetic -> word splitting -> pathname) and quoting behavior before implementing variables or globbing. Expansion must never accidentally turn data into executable syntax.
3. **Command Resolver**: Define PATH lookup, executable format handling, working directory, environment inheritance, and error precedence (`command not found` vs `permission denied`). Do not fall back to a host shell or implicitly execute scripts through an unspecified interpreter.
4. **Redirection / Pipelines**: Treat file creation and truncation as side effects. Validate AST structure prior to execution. Ensure every file descriptor is closed on both success and failure. Test error paths and resource exhaustion.
5. **Built-ins**: Explicitly mark each built-in as state-changing or read-only. Shell state built-ins (`cd`, `export`, `unset`, `exit`) must run in the current shell process. Administrative commands must invoke typed OS APIs with authorization checks.
6. **Interactive Terminal**: Separate terminal line editing from parsing and execution. Handle EOF, terminal resize (`SIGWINCH`), signals, Unicode/display width, and non-TTY input cleanly without corrupting input buffers.
7. **History / Completion**: Bound and permission-protect history; provide a no-history mode; avoid storing secrets by default. Completion must inspect metadata/filesystem state and must never execute candidate commands.
8. **Jobs and Signals**: Define process group, terminal foreground ownership, status collection, shell exit policy, and child cleanup. Test with pseudo-terminals (PTY) rather than unit-level stubs alone.
9. **Diagnostics**: Send error messages to stderr, include source line/column for script errors, enforce stable exit-status rules, and avoid leaking secrets from environment or history.
10. **User Documentation**: Provide a command reference, scripting guide, compatibility statement, and troubleshooting guide.

---

## PR Review and Merge Gates

### PR Review Template
Every shell pull request must adhere to the standard PR template:
- **Summary**: Problem solved.
- **Scope**: Parser, expansion, execution, terminal, built-in, tests, or documentation.
- **Compatibility Profile**: Native SigmaOS subset vs POSIX-oriented subset vs Interactive feature.
- **Design**: AST/lexer/parser behavior, argument representation, process/descriptor handling, state changes, error semantics.
- **Security & Failure Behavior**: Input/resource limits, permission boundaries, cleanup on failure.
- **Tests**: Unit, parser golden, fuzz/property, script, PTY/QEMU tests.
- **Status & Docs**: `Proposed` | `Prototype` | `Integrated` | `Supported`.

### Review Requirements
- **1 Maintainer Approval**: Ordinary shell changes (formatting, docs, minor built-in tweaks).
- **2 Maintainer Approvals**: Command execution, expansion, job control, redirection/pipeline lifecycle, privileged built-ins, or compatibility-sensitive behavior.

---

## Recommended Rollout and Maturity Gates

- **Foundation (PRs 1–4)**: Establish compatibility promise, parser architecture, tests, and runtime command contract.
- **Usable Command Runner (PRs 5–10)**: Add safe quoting/variables, redirection, pipelines, built-ins, and script mode in a controlled sequence.
- **Interactive Shell (PRs 12–14)**: Add terminal line editing, job control, and completion after process execution and signal semantics are stabilized.
- **Compatibility and Hardening (PRs 15–17)**: Enforce resource limits, collect conformance evidence, publish documentation, and certify capability status.

### Status Transition Gates
- **`Prototype`**: Parser and basic built-ins exist in codebase.
- **`Integrated`**: Command execution, scripts, redirection, and pipelines run end-to-end against the OS process and file descriptor mechanisms in integrated tests.
- **`Supported`**: Repeatable regression and PTY testing pass, resource limits are enforced, documented limitations exist, and the compatibility profile matches observed runtime behavior.
