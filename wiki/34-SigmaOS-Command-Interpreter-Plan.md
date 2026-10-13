# Idea 34: SigmaOS Command Interpreter Plan & Architecture Strategy

## Overview & Status
SigmaOS lists `src/userspace/shell.rs` and `src/shell/sovereign_command_interpreter.rs` as a command interpreter. Initial milestones focus on pipelines, redirection, and job control. Code presence or unit tests do not prove full OS readiness or runtime integration; shell capabilities must earn maturity status through end-to-end evidence.

The plan is organized as proposed PRs for a small, usable command interpreter first, then an explicitly scoped POSIX-like shell. POSIX defines shell processing as tokenization, parsing, expansion, redirection, execution, and status collection; quoting and expansion order are central to correct behavior. Bash and BSD shells add interactive conveniences such as line editing, history, and job control, but those should not be confused with baseline script-language compatibility.

## Product & Compatibility Rules

Before feature work, decide and document whether SigmaOS's shell is:
1. **A small native command runner**: a limited syntax for interactive system management, with no promise to run POSIX shell scripts; or
2. **A POSIX-oriented shell**: a staged implementation of the standardized shell language and utilities, with compatibility measured against a declared standard and test suite.

**Recommended approach**: begin with a safe native command runner, then add a documented POSIX subset in phases. Do not advertise `sh` or POSIX compatibility until the corresponding syntax, expansion, execution, and conformance tests pass.

### Mandatory Rules for All Shell PRs:
* **Parse before executing**: Syntax errors must be identified before any part of the command executes where the defined grammar allows it. Never execute a valid prefix of a malformed pipeline or compound command.
* **Preserve arguments exactly**: Represent arguments as structured values; do not re-split expanded strings or invoke another shell to execute commands. Quote-aware tokenization and expansion are required for correct argument boundaries.
* **Use an AST, not ad hoc string splitting**: Separate lexer, parser, expansion, command resolution, execution, and interactive terminal handling.
* **Keep script and interactive modes distinct**: No prompts, history expansion, line editing, or interactive startup hooks in noninteractive scripts unless explicitly specified.
* **Define behavior for unsupported syntax**: Emit a clear error and nonzero status; never silently reinterpret unsupported syntax as a different command.
* **Treat input and environment as untrusted**: Bound command length, token count, nesting depth, expansion size, environment size, and history size. Never execute a string produced by an expansion as shell syntax unless command substitution is an explicitly supported language construct.
* **No compatibility claims based on examples alone**: Every claimed feature must link to parser, execution, and integration tests.
* **Keep commands and privileged operations separate**: Built-ins that change system state must use typed interfaces and permissions; never pass raw shell text into a privileged service.

## Proposed PR Sequence

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(shell): define command interpreter goals and compatibility profile` | Document target users, interactive vs script use, supported syntax, intended sh/POSIX status, built-in policy, limitations, and terminal assumptions. | Compatibility statement published; "POSIX shell" not claimed unless test criteria defined; prototype status listed. |
| 2 | `refactor(shell): split lexer parser expander and executor modules` | Replace monolithic parsing logic with distinct, testable components; introduce an AST for simple commands and operators. | Parser and executor tested independently; malformed syntax produces structured errors; existing built-ins continue working. |
| 3 | `test(shell): add golden lexer and parser regression suite` | Add fixtures for whitespace, quotes, escapes, operators, comments, multiline input, and malformed cases. Add property/fuzz targets for lexer/parser input. | Tests assert exact token/AST output; fuzz input cannot panic, hang, or exceed resource limits; fixed parser bugs become regression cases. |
| 4 | `feat(shell): implement command lookup and process execution contract` | Define environment, current directory, PATH lookup, executable invocation, standard streams, exit status, and error behavior. | `echo`, `pwd`, and `ls` work through documented model; command-not-found and permission errors distinct; cleanup tested. |
| 5 | `feat(shell): add variables assignments and safe quoting` | Add simple assignments, single/double quotes, escapes, and positional/environment variable handling. | Tests cover empty values, spaces, quotes, variable boundaries, unset values, expansion errors; argument arrays remain intact. |
| 6 | `feat(shell): implement POSIX-style redirection primitives` | Add input, output, append, and error redirection in a staged design; define ordering and descriptor semantics. | Tests cover missing paths, permissions, descriptor cleanup, redirection order, failure before execution; unsupported forms fail explicitly. |
| 7 | `feat(shell): implement pipelines with bounded process and pipe lifecycle` | Implement `cmd1 \| cmd2` with input/output wiring, close-on-exec semantics, failure cleanup, and pipeline status policy. | Multi-command tests verify data flow, EOF, broken pipe, command failure, resource cleanup; no deadlocks under documented limits. |
| 8 | `feat(shell): define exit status and command-list operators` | Add command sequencing and documented status model; add `&&`/`\|\|` short-circuit operators. | Short-circuiting, last-command/pipeline status semantics, signal-derived status proven; no ambiguous fallback parsing. |
| 9 | `feat(shell): add environment and PATH built-ins` | Add required shell-state built-ins (`cd`, `export`, `unset`, `exit`, `alias`, `unalias`). | State-changing built-ins affect current shell; invalid options return documented statuses; path/environment edge cases covered. |
| 10 | `feat(shell): add script mode and startup policy` | Add file and `-c` execution modes, noninteractive behavior, script errors, cautious startup-file policy. | Script mode emits no prompt or interactive history; failures have deterministic status & diagnostics; startup files not sourced implicitly from unsafe paths. |
| 11 | `feat(shell): add control-flow subset for shell scripts` | Add `if`, loops, functions, and `case` with grammar and execution semantics fixed first. | Each construct has parser, execution, error, and integration tests; nested/malformed forms handled predictably; supported subset documented. |
| 12 | `feat(shell): add interactive line editor and bounded history` | Add terminal-aware input, cursor editing, history navigation/search, multiline prompts, history persistence policy. | Works only when attached to terminal; noninteractive mode unaffected; history size-bounded and permission-restricted. |
| 13 | `feat(shell): add foreground job control and signal handling` | Implement foreground process groups, terminal ownership, interrupt/suspend behavior, job table; background jobs added after foreground control is stable. | Tests cover Ctrl-C, suspend/resume, terminal restoration, shell exit with active jobs, child cleanup in PTY/QEMU; unsupported operations fail clearly. |
| 14 | `feat(shell): add command completion and help metadata` | Build completion from command/builtin metadata, filesystem entries, documented argument providers; avoid executing candidate commands. | Completion non-executing, bounded, permission-aware, works with quoted input; help text generated from command metadata. |
| 15 | `security(shell): enforce parser and expansion resource limits` | Add limits for source size, nesting, tokens, expansion output, environment, pipeline length, open descriptors; audit command substitution and path resolution. | Exceeding limit returns diagnostic without panic or partial privileged action; regression tests cover exhaustion/expansion attacks. |
| 16 | `test(shell): add POSIX subset and Linux BSD interoperability matrix` | Add differential tests for declared subset against reference shells; classify differences as intentional or bugs. | Declared standard/profile per test; results versioned; no broad compatibility claim from small smoke-test set. |
| 17 | `docs(shell): publish command reference and maturity evidence` | Document grammar, built-ins, variables, status semantics, scripting, terminal behavior, security limits, known gaps. Update capability inventory and wiki. | Every listed feature links to relevant tests; Prototype/Integrated/Supported status matches evidence. |

## Periodic Maintenance Framework

* **Every PR**: `ci(shell)` — Run shell checks for changed code (formatting, compile/tests, parser regression, fuzz/integration checks).
* **Weekly**: `test(shell)` — Review fuzz crashes and parser regressions; resolved crashes become regression tests.
* **Weekly**: `docs(shell)` — Reconcile feature status with implementation; stale claims removed.
* **Monthly**: `security(shell)` — Review command execution, expansion boundaries, environment, path resolution, redirection, and privileged built-ins.
* **Monthly**: `test(shell)` — Run script compatibility corpus against declared language subset.
* **Monthly**: `maint(shell)` — Review terminal, signal, and child-process cleanup under pseudo-terminal tests.
* **Quarterly**: `docs(shell)` — Review POSIX and reference-shell alignment (FreeBSD `sh`, GNU `bash`).
* **Each Release Candidate**: `release(shell)` — Certify shell support claims via clean boot/test environment smoke tests.
* **After Each Shell Incident**: `fix(shell)` — Prevent recurrence with root cause, minimized reproducer, regression test, and impact assessment.

## Component-Specific Rules

* **Lexer/Parser**: Preserve source locations for diagnostics; never use lossy token splitting; enforce nesting and input limits.
* **Expansion**: Specify expansion order and quote behavior before implementing variables, globbing, or arithmetic.
* **Command Resolver**: Define PATH lookup, executable format handling, working directory, environment inheritance, error precedence.
* **Redirection/Pipelines**: Treat file creation/truncation as side effects. Validate parse structure first; close every descriptor on success and failure.
* **Built-ins**: Mark each as state-changing or read-only. `cd`, `export`, `unset`, and `exit` modify current shell state.
* **Interactive Terminal**: Separate terminal editing from parsing and execution. Handle EOF, terminal resize, signals, Unicode/display width without corrupting command bytes.
* **History/Completion**: Bound and protect history; no-history mode; avoid storing secrets. Completion must inspect metadata/filesystem, never execute candidate commands.
* **Jobs & Signals**: Define process group, terminal foreground ownership, status collection, shell-exit policy, cleanup.
* **Diagnostics**: Send errors to stderr, include source line/column for scripts, use stable exit-status rules, avoid exposing secrets.
* **User Documentation**: Provide command reference, scripting guide, compatibility statement, and troubleshooting guide.

## PR Review & Merge Gates Template

Each shell PR must include:
* **Summary**: What user or developer problem does this PR solve?
* **Scope**: Parser, expansion, execution, terminal, built-in, tests, or documentation.
* **Compatibility Profile**: Native SigmaOS subset | POSIX-oriented subset | Interactive-only feature.
* **Design**: AST behavior, argument representation, process/descriptor handling, state changes, error/status semantics.
* **Security & Failure Behavior**: Input/resource limits, permission boundaries, cleanup on failure, command-execution risks.
* **Tests**: Unit, parser golden, fuzz/property, script, PTY/QEMU, reference-shell tests.
* **Status & Docs**: Proposed | Prototype | Integrated | Supported.
* **Checklist**:
  - [ ] Focused change; no unrelated refactor
  - [ ] Tests cover success, syntax errors, and failure cleanup
  - [ ] No shell-string re-splitting or implicit host-shell execution
  - [ ] Resource limits and side effects considered
  - [ ] CI passes; review conversations resolved
  - [ ] Compatibility claims match tested behavior

Require one maintainer approval for ordinary shell changes and two reviewers for command execution, expansion, job control, privileged built-ins, or compatibility-sensitive behavior.

## Rollout & Maturity Gates

* **Foundation**: PRs 1–4 establish compatibility promise, parser architecture, tests, runtime command contract.
* **Usable Command Runner**: PRs 5–10 add safe quoting/variables, redirection, pipelines, built-ins, script mode.
* **Interactive Shell**: PRs 12–14 add terminal editing, job control, completion.
* **Compatibility & Hardening**: PRs 15–17 add resource limits, conformance evidence, documentation, truthful capability status.

Keep shell marked **Prototype** until the supported boot path can run commands, scripts, redirection, pipelines end to end. Mark **Integrated** only after those paths work with OS process and file-descriptor mechanisms. Mark **Supported** only after repeatable regression and PTY testing agree with observed behavior.
