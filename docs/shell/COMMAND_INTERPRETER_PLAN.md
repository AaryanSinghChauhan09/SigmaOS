# SigmaOS Command Interpreter Plan

## Status and Context
SigmaOS lists `src/userspace/shell.rs` as a prototype with a command parser and echo, pwd, and ls built-ins; its stated next milestone is pipelines, redirection, and job control. The broader project cautions that unit tests or code presence do not prove runtime integration or a usable OS, so this plan makes shell capabilities earn their maturity status through end-to-end evidence.

The plan is organized as proposed PRs for a small, usable command interpreter first, then an explicitly scoped POSIX-like shell. POSIX defines shell processing as tokenization, parsing, expansion, redirection, execution, and status collection; quoting and expansion order are central to correct behavior. Bash and BSD shells add interactive conveniences such as line editing, history, and job control, but those should not be confused with baseline script-language compatibility.

## Product and Compatibility Rules
Before feature work, decide and document whether SigmaOS's shell is:
1. A small native command runner: a limited syntax for interactive system management, with no promise to run POSIX shell scripts; or
2. A POSIX-oriented shell: a staged implementation of the standardized shell language and utilities, with compatibility measured against a declared standard and test suite.

**Recommended approach:** begin with a safe native command runner, then add a documented POSIX subset in phases. Do not advertise sh or POSIX compatibility until the corresponding syntax, expansion, execution, and conformance tests pass.

### Core Rules for Shell PRs
- **Parse before executing:** Syntax errors must be identified before any part of the command executes where the defined grammar allows it. Never execute a valid prefix of a malformed pipeline or compound command.
- **Preserve arguments exactly:** Represent arguments as structured values; do not re-split expanded strings or invoke another shell to execute commands. Quote-aware tokenization and expansion are required for correct argument boundaries.
- **Use an AST, not ad hoc string splitting:** Separate lexer, parser, expansion, command resolution, execution, and interactive terminal handling.
- **Keep script and interactive modes distinct:** No prompts, history expansion, line editing, or interactive startup hooks in noninteractive scripts unless explicitly specified. Bash documents interactive behavior separately, including job control, line editing, history, and prompt handling.
- **Define behavior for unsupported syntax:** Emit a clear error and nonzero status; never silently reinterpret unsupported syntax as a different command.
- **Treat input and environment as untrusted:** Bound command length, token count, nesting depth, expansion size, environment size, and history size. Never execute a string produced by an expansion as shell syntax unless command substitution is an explicitly supported language construct.
- **No compatibility claims based on examples alone:** Every claimed feature must link to parser, execution, and integration tests.
- **Keep commands and privileged operations separate:** Built-ins that change system state must use typed interfaces and permissions; never pass raw shell text into a privileged service.

## Proposed PR Sequence

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(shell): define command interpreter goals and compatibility profile` | Document target users, interactive vs script use, supported syntax, intended sh/POSIX status, built-in policy, limitations, and terminal assumptions. | A compatibility statement is published; "POSIX shell" is not claimed unless test criteria are defined; current prototype status and missing features are listed. |
| 2 | `refactor(shell): split lexer parser expander and executor modules` | Replace monolithic parsing logic with distinct, testable components; introduce an AST for simple commands and operators. | Parser and executor can be tested independently; malformed syntax produces structured errors; existing built-ins continue to work without regression. |
| 3 | `test(shell): add golden lexer and parser regression suite` | Add fixtures for whitespace, quotes, escapes, operators, comments, multiline input, and malformed cases. Add property/fuzz targets for lexer/parser input. | Tests assert exact token/AST output; fuzz input cannot panic, hang, or exceed declared resource limits; every fixed parser bug becomes a regression case. |
| 4 | `feat(shell): implement command lookup and process execution contract` | Define environment, current directory, PATH lookup, executable invocation, standard streams, exit status, and error behavior. | echo, pwd, and ls work through the same documented command model; command-not-found and permission errors are distinct; process and file descriptor cleanup are tested. |
| 5 | `feat(shell): add variables assignments and safe quoting` | Add simple assignments, single/double quotes, escapes, and positional/environment variable handling according to the selected profile. | Tests cover empty values, spaces, quotes, variable boundaries, unset values, and expansion errors; argument arrays remain intact through execution. |
| 6 | `feat(shell): implement POSIX-style redirection primitives` | Add input, output, append, and error redirection in a staged design; define ordering and descriptor semantics. | Tests cover missing paths, permissions, descriptor cleanup, redirection order, and failure before command execution; unsupported redirection forms fail explicitly. |
| 7 | `feat(shell): implement pipelines with bounded process and pipe lifecycle` | Implement cmd1 \| cmd2 with correct input/output wiring, close-on-exec semantics, failure cleanup, and pipeline status policy. | Multi-command tests verify data flow, EOF, broken pipe, command failure, and resource cleanup; no deadlocks under the documented limits. POSIX specifies connecting each pipeline command's output to the next command's input. |
| 8 | `feat(shell): define exit status and command-list operators` | Add command sequencing and a clearly documented status model; add &&/\|\| only if included in the selected profile. | Tests prove short-circuiting, last-command/pipeline status semantics, and signal-derived status behavior; no ambiguous fallback parsing. |
| 9 | `feat(shell): add environment and PATH built-ins` | Add required shell-state built-ins such as cd, export, unset, and exit; define which commands must execute in the current shell process. | State-changing built-ins affect the current shell; invalid options and arguments return documented statuses; path/environment edge cases are covered. |
| 10 | `feat(shell): add script mode and startup policy` | Add file and -c execution modes, noninteractive behavior, script errors, and a cautious startup-file policy. | Script mode emits no prompt or interactive history behavior; failures have deterministic status and line/column diagnostics; startup files are not sourced implicitly from unsafe locations. |
| 11 | `feat(shell): add control-flow subset for shell scripts` | Add if, loops, functions, and/or case only in separate scoped PRs, with grammar and execution semantics fixed first. | Each construct has parser, execution, error, and integration tests; nested and malformed forms are handled predictably; supported subset is documented. |
| 12 | `feat(shell): add interactive line editor and bounded history` | Add terminal-aware input, cursor editing, history navigation/search, multiline prompts, and history persistence policy. | Works only when attached to a terminal; noninteractive mode is unaffected; history is size-bounded, permission-restricted, and can avoid recording sensitive commands. Bash's interactive behavior offers a useful feature reference, not a requirement to copy its implementation. |
| 13 | `feat(shell): add foreground job control and signal handling` | Implement foreground process groups, terminal ownership, interrupt/suspend behavior, and job table; add background jobs only after foreground control is stable. | Tests cover Ctrl-C, suspend/resume, terminal restoration, shell exit with active jobs, and child cleanup in a PTY/QEMU environment; unsupported job-control operations fail clearly. |
| 14 | `feat(shell): add command completion and help metadata` | Build completion from command/builtin metadata, filesystem entries, and documented argument providers; avoid executing candidate commands. | Completion is non-executing, bounded, permission-aware, and works with quoted/incomplete input; help text is generated from the same command metadata. |
| 15 | `security(shell): enforce parser and expansion resource limits` | Add limits for source size, nesting, tokens, expansion output, environment, pipeline length, and open descriptors; audit command substitution and path resolution. | Exceeding a limit returns a diagnostic without kernel panic or partial privileged action; regression tests cover exhaustion and expansion attacks. |
| 16 | `test(shell): add POSIX subset and Linux BSD interoperability matrix` | Add differential tests for the declared subset against selected reference shells; classify differences as intentional or bugs. | Every test has a declared standard/profile; results are versioned; no broad compatibility claim is made from a small smoke-test set. POSIX conformance suites exist for shell and utilities; use one only if licensing and target-platform fit are reviewed. |
| 17 | `docs(shell): publish command reference and maturity evidence` | Document grammar, built-ins, variables, status semantics, scripting, terminal behavior, security limits, and known gaps. Update capability inventory and component wiki. | Every listed feature links to relevant tests; Prototype/Integrated/Supported status matches evidence; examples are executed in CI where practical. |

## Periodic Maintenance PRs

| Cadence | Proposed recurring PR/task | Required output |
|---|---|---|
| Every PR | `ci(shell): run shell-specific checks for changed code` | Formatting, compile/tests, parser regression tests, and relevant fuzz or integration checks; skipped checks explained. |
| Weekly | `test(shell): review fuzz crashes and parser regressions` | Each crash is fixed, converted to a tracked issue with severity/owner, or documented as non-actionable; resolved crashes become regression tests. |
| Weekly | `docs(shell): reconcile feature status with implementation` | Feature inventory and shell docs reflect tested behavior, not code presence; stale compatibility claims are removed. |
| Monthly | `security(shell): review command execution and expansion boundaries` | Review token/AST handling, executable lookup, environment, path resolution, redirection, command substitution if implemented, and privileged built-ins. |
| Monthly | `test(shell): run script compatibility corpus` | Deterministic tests for the declared language subset; differences against Linux/BSD reference shells are classified and documented. |
| Monthly | `maint(shell): review terminal, signal, and child-process cleanup` | PTY/integration tests for interrupted commands, terminal restoration, orphaned children, and descriptor leaks. |
| Quarterly | `docs(shell): review POSIX and reference-shell alignment` | Recheck the chosen POSIX edition and documented deviations; review FreeBSD sh and GNU Bash as behavior references without implying equivalent scope. |
| Each release candidate | `release(shell): certify shell support claims` | Clean boot or supported test environment, interactive and script-mode smoke tests, regression suite, known limitations, and exact target configuration. |
| After each shell incident | `fix(shell): prevent recurrence of <bug>` | Root cause, minimized reproducer, regression test, impact assessment, and update to docs or limits if needed. |

## Rules for Shell-Related Components
- **Lexer/parser:** Preserve source locations for diagnostics; never use lossy token splitting; enforce nesting and input limits. POSIX describes token recognition and grammar as separate steps, with substitutions handled according to context—use that as a reference for the selected profile.
- **Expansion:** Specify expansion order and quote behavior before implementing variables, globbing, arithmetic, or command substitution. Expansion must not accidentally turn data into syntax. POSIX and Bash both document expansion sequencing and quoting rules.
- **Command resolver:** Define PATH lookup, executable format handling, working directory, environment inheritance, and error precedence. Do not fall back to a host shell or implicitly execute scripts through an unspecified interpreter.
- **Redirection/pipelines:** Treat file creation and truncation as side effects. Validate parse structure first; ensure every descriptor is closed on both success and failure; test error cases and resource exhaustion.
- **Built-ins:** Mark each built-in as state-changing or read-only. `cd`, `export`, `unset`, and `exit` generally need current-shell semantics; administrative commands must call typed OS APIs with authorization checks.
- **Interactive terminal:** Separate terminal editing from parsing and execution. Handle EOF, terminal resize, signals, Unicode/display width, and non-TTY input without corrupting command bytes.
- **History/completion:** Bound and protect history; provide a no-history mode; avoid storing secrets by default where possible. Completion must inspect metadata/filesystem state, never execute commands to discover candidates.
- **Jobs and signals:** Define process group, terminal foreground ownership, status collection, shell-exit policy, and cleanup. Test with a pseudo-terminal rather than only unit-level models.
- **Diagnostics:** Send errors to stderr, include source line/column for scripts, use stable exit-status rules, and avoid exposing secrets from environment or command history.
- **User documentation:** Provide a command reference, scripting guide, compatibility statement, and troubleshooting guide. FreeBSD's handbook illustrates shells as an interactive command interface that also composes commands with redirection and pipelines.

## PR Review and Merge Gates

Use this template for each shell PR:

```markdown
## Summary
What user or developer problem does this PR solve?

## Scope
Parser, expansion, execution, terminal, built-in, tests, or documentation.
State what this PR does not implement.

## Compatibility profile
Native SigmaOS subset | POSIX-oriented subset | Interactive-only feature
Cite the intended behavior and list intentional deviations.

## Design
Lexer/parser/AST behavior, argument representation, process/descriptor handling,
state changes, and error/status semantics.

## Security and failure behavior
Input/resource limits, permission boundaries, partial-side-effect behavior,
cleanup on failure, and any command-execution risks.

## Tests
Unit, parser golden, fuzz/property, script, PTY/QEMU, and reference-shell tests.
List commands run and anything not tested.

## Status and docs
Proposed | Prototype | Integrated | Supported
Explain evidence; update the capability matrix and shell docs.

## Checklist
- [ ] Focused change; no unrelated refactor
- [ ] Tests cover success, syntax errors, and failure cleanup
- [ ] No shell-string re-splitting or implicit host-shell execution
- [ ] Resource limits and side effects considered
- [ ] CI passes; review conversations resolved
- [ ] Compatibility claims match tested behavior
```

Require one maintainer approval for ordinary shell changes and two reviewers for command execution, expansion, job control, privileged built-ins, or compatibility-sensitive behavior. Protect the default branch with required reviews and passing checks.

## Recommended Rollout and Maturity Gates
- **Foundation:** PRs 1–4 establish the compatibility promise, parser architecture, tests, and runtime command contract.
- **Usable command runner:** PRs 5–10 add safe quoting/variables, redirection, pipelines, built-ins, and script mode in a controlled order.
- **Interactive shell:** PRs 12–14 add terminal editing, job control, and completion after process execution and signal semantics work.
- **Compatibility and hardening:** PRs 15–17 add resource limits, conformance evidence, documentation, and truthful capability status.

Keep the shell marked **Prototype** until the same supported boot path can run commands, scripts, redirection, and pipelines end to end. Mark it **Integrated** only after those paths work with the OS's process and file-descriptor mechanisms. Use **Supported** only after repeatable regression and PTY testing, documented limitations, and the declared compatibility profile all agree with observed behavior.
