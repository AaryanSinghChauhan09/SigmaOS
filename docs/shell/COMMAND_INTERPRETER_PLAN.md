# SigmaOS Command Interpreter Architecture & Development Plan

## Overview
SigmaOS command interpreter plan provides a small, usable native command interpreter first, transitioning into an explicitly scoped POSIX-like shell.

## Current Prototype Status
SigmaOS lists `src/userspace/shell.rs` as a prototype with a command parser and `echo`, `pwd`, and `ls` built-ins. The next milestone is pipelines, redirection, and job control. Code presence or unit tests do not prove runtime integration or a usable OS; capabilities must earn maturity status through end-to-end evidence.

## Product and Compatibility Rules
1. **Parse before executing**: Syntax errors must be identified before any part of the command executes where grammar allows. Never execute a valid prefix of a malformed pipeline or compound command.
2. **Preserve arguments exactly**: Represent arguments as structured values. Do not re-split expanded strings or invoke another shell. Quote-aware tokenization and expansion are required.
3. **Use an AST, not ad hoc string splitting**: Separate lexer, parser, expansion, command resolution, execution, and interactive terminal handling.
4. **Keep script and interactive modes distinct**: No prompts, history expansion, line editing, or interactive startup hooks in non-interactive scripts.
5. **Define behavior for unsupported syntax**: Emit a clear error and non-zero status; never silently reinterpret unsupported syntax.
6. **Treat input and environment as untrusted**: Bound command length, token count, nesting depth, expansion size, environment size, and history size.
7. **No compatibility claims based on examples alone**: Link every claimed feature to parser, execution, and integration tests.
8. **Keep commands and privileged operations separate**: Built-ins changing system state must use typed interfaces and permissions; never pass raw shell text into a privileged service.

## Proposed 17-PR Sequence Roadmap
| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(shell): define command interpreter goals and compatibility profile` | Document target users, interactive vs script use, supported syntax, sh/POSIX status, built-in policy, limitations. | Compatibility statement published; POSIX shell not claimed without test criteria. |
| 2 | `refactor(shell): split lexer parser expander and executor modules` | Replace monolithic parsing logic with distinct components and AST. | Parser and executor tested independently; malformed syntax produces structured errors. |
| 3 | `test(shell): add golden lexer and parser regression suite` | Fixtures for whitespace, quotes, escapes, operators, comments, multiline, malformed cases, fuzz targets. | Exact token/AST output asserted; fuzz cannot panic or hang. |
| 4 | `feat(shell): implement command lookup and process execution contract` | Environment, directory, PATH lookup, executable invocation, streams, exit status. | Built-ins work through documented command model; command-not-found distinct. |
| 5 | `feat(shell): add variables assignments and safe quoting` | Assignments, quotes, escapes, positional/environment variables. | Tests cover spaces, quotes, variable boundaries; argument arrays intact. |
| 6 | `feat(shell): implement POSIX-style redirection primitives` | Input, output, append, error redirection ordering and descriptor semantics. | Missing paths, permissions, descriptor cleanup tested; fail before execution. |
| 7 | `feat(shell): implement pipelines with bounded process and pipe lifecycle` | `cmd1 \| cmd2` with input/output wiring, close-on-exec, failure cleanup. | Data flow, EOF, broken pipe tested; no deadlocks. |
| 8 | `feat(shell): define exit status and command-list operators` | Sequencing, status model, `&&`/`\|\|` short-circuiting. | Short-circuiting and signal-derived status proven. |
| 9 | `feat(shell): add environment and PATH built-ins` | Current-shell state built-ins: `cd`, `export`, `unset`, `exit`. | State-changing built-ins affect current shell; path edge cases covered. |
| 10 | `feat(shell): add script mode and startup policy` | File and `-c` execution, non-interactive behavior, script errors. | Deterministic status; no prompt in script mode. |
| 11 | `feat(shell): add control-flow subset for shell scripts` | `if`, loops, functions, `case` in scoped PRs. | Parser, execution, error tests for each construct. |
| 12 | `feat(shell): add interactive line editor and bounded history` | Terminal input, cursor editing, history navigation, multiline prompt. | Terminal attached required; non-interactive unaffected; history bounded. |
| 13 | `feat(shell): add foreground job control and signal handling` | Process groups, terminal ownership, interrupt/suspend behavior. | Ctrl-C, suspend/resume, PTY/QEMU tested. |
| 14 | `feat(shell): add command completion and help metadata` | Completion from metadata, filesystem entries, non-executing. | Non-executing, bounded, works with quoted input. |
| 15 | `security(shell): enforce parser and expansion resource limits` | Limits for source size, nesting, tokens, expansion, environment, pipelines. | Diagnostics on limit breach; no kernel panic or partial privileged action. |
| 16 | `test(shell): add POSIX subset and Linux BSD interoperability matrix` | Differential tests against reference shells. | Declared standard/profile per test; results versioned. |
| 17 | `docs(shell): publish command reference and maturity evidence` | Document grammar, built-ins, variables, status semantics, scripting, gaps. | Feature links to tests; Prototype/Integrated/Supported status matches evidence. |

## Periodic Maintenance Tasks
- **Every PR**: Formatting, compile/tests, parser regression tests, fuzz or integration checks.
- **Weekly**: Triage fuzz crashes and parser regressions.
- **Weekly**: Reconcile feature status with implementation.
- **Monthly**: Security review of command execution and expansion boundaries.
- **Monthly**: Run script compatibility corpus.
- **Monthly**: Review terminal, signal, and child-process cleanup.
- **Quarterly**: Review POSIX and reference-shell alignment.
- **Each Release Candidate**: Certify shell support claims in clean boot environment.
- **After Incident**: Root cause analysis and regression test creation.

## Rules for Shell Components
- **Lexer/Parser**: Preserve source locations; no lossy token splitting; enforce nesting limits.
- **Expansion**: Document expansion order (brace, tilde, parameter, command, arithmetic, word splitting, glob). Never convert data to syntax.
- **Command Resolver**: Define PATH lookup, working directory, environment inheritance, error precedence.
- **Redirection/Pipelines**: Validate parse structure first; close descriptors on success and failure.
- **Built-ins**: Mark state-changing vs read-only; typed OS APIs with authorization checks.
- **Interactive Terminal**: Separate terminal editing from parsing and execution.
- **History/Completion**: Bound and protect history; no-history mode; secret filtering.
- **Jobs and Signals**: Process group, foreground ownership, status collection, shell exit policy.

## PR Review and Merge Gates
Template required for each shell PR:
```markdown
## Summary
## Scope
## Compatibility profile
## Design
## Security and failure behavior
## Tests
## Status and docs
## Checklist
```
Requires one maintainer approval for ordinary changes and two reviewers for execution/expansion/job control/privileged built-ins.

## Rollout and Maturity Gates
- **Proposed**: Architectural definition and grammar specification published.
- **Prototype**: Unit and parser tests pass.
- **Integrated**: Boot path executes commands, redirection, and pipelines end-to-end with kernel process/FD mechanisms.
- **Supported**: Repeatable regression and PTY testing, documented limits, and matching compatibility profile.
