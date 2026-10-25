# SigmaOS Shell Maintenance & Operating Model

## Objective

Maintain SigmaOS shell (`sigma-sh`) as a safe, predictable, script-compatible command interpreter that boots, runs real commands, handles failures cleanly, and remains testable in CI. The primary milestone is validating a robust **boot-to-interactive-shell path**.

## Shell Task Calendar

| Frequency | Shell task | Output | Required PR |
|---|---|---|---|
| Every PR | Build, lint, unit, and integration tests | CI result | `feat`, `fix`, `test`, `docs`, or `chore` PR |
| Daily | CI/nightly failure triage | Root cause + fix | `fix(shell)` or `fix(ci)` PR |
| Weekly | Shell feature/bug triage | Prioritized backlog | Issue + scoped PR |
| Weekly | POSIX/sh compatibility study | Study note | `docs(studies)` PR |
| Biweekly | Parser and lexer audit | Grammar/edge-case findings | `fix(shell)` + `test(shell)` PR |
| Biweekly | Built-in command review | Command contract updates | `feat(shell)` / `test(shell)` PR |
| Monthly | Boot-to-shell validation | Boot log + shell smoke report | `test(shell)` PR |
| Monthly | Script compatibility suite | POSIX test results | `test(shell)` PR |
| Monthly | Fuzzing and crash triage | Crash reports + fixes | `fix(shell)` PR |
| Monthly | Security and sandbox review | Threat-model update | `security(shell)` PR |
| Quarterly | Shell release readiness review | Release checklist | `docs(release)` PR |
| Quarterly | UX and accessibility review | UX findings + fixes | `feat(shell)` / `docs(shell)` PR |
| Quarterly | Performance benchmark review | Benchmark report | `perf(shell)` PR |

## Component Scope

1. **Lexer**: Converts raw string input into tokens.
2. **Parser**: Constructs ASTs, commands, pipelines (`|`), lists (`&&`, `;`), and control structures (`if`, `for`).
3. **Expander**: Implements brace, tilde, parameter, command, arithmetic, word splitting, and pathname expansion.
4. **Executor**: Dispatches built-ins, external binaries, redirections (`>`, `<`, `>>`), and process groups.
5. **Job Control**: Manages foreground/background processes (`&`), job IDs, signals (SIGINT, SIGTSTP), and terminal state.
6. **Line Editor**: Provides interactive prompt, history navigation, cursor movement, multiline input, and tab completion.
7. **Built-ins**: Implements `cd`, `pwd`, `echo`, `export`, `alias`, `set`, `exit`, etc.
8. **Script Runner**: Non-interactive deterministic script execution.
9. **Diagnostics**: Generates parse error line/col details, unknown command suggestions, and redirection errno messages.

## Weekly Triage Classification Template

Each shell issue must follow this format:

```markdown
## Shell issue

**Area:** parser / expansion / executor / job control / built-in / UX
**Input:**
```sh
<exact command or script>
```

**Expected:**
<exact expected stdout, stderr, and exit code>

**Actual:**
<exact observed behavior>

**POSIX/reference behavior:**
<link or explanation>

**Acceptance test:**
<test name or proposed test>
```

## First 12 Shell PR Sequence

| # | PR | Purpose |
|---:|---|---|
| 1 | `docs(shell): add shell maintenance calendar` | Establish recurring shell work |
| 2 | `docs(shell): define POSIX compatibility subset` | Prevent vague compatibility claims |
| 3 | `docs(shell): add grammar and expansion-order specification` | Make parser behavior authoritative |
| 4 | `test(shell): add lexer and parser unit tests` | Establish parser correctness baseline |
| 5 | `feat(shell): implement deterministic script execution` | Ensure repeatable non-interactive behavior |
| 6 | `feat(shell): add exit-code policy and tests` | Make failures observable |
| 7 | `feat(shell/builtins): add cd, pwd, echo, export contracts` | Establish core built-ins |
| 8 | `test(shell): add pipeline and redirection conformance tests` | Validate core execution |
| 9 | `feat(shell): add interactive prompt and Ctrl-C handling` | Establish usable interactive mode |
| 10 | `feat(fuzz): add lexer, parser, and expansion fuzz targets` | Protect against malformed input |
| 11 | `test(boot): add boot-to-shell smoke validation` | Connect shell to SigmaOS boot path |
| 12 | `docs(release): add shell release gates` | Prevent premature "usable shell" claims |

## Release Acceptance Gates (G0 - G8)

- **G0 Code Quality**: `cargo check --lib`, formatting, and unit tests pass.
- **G1 Parser Baseline**: Shell unit and parser tests pass.
- **G2 POSIX Subset**: Documented POSIX subset tests pass.
- **G3 Fuzz Smoke**: Fuzz target passes with zero unhandled panics.
- **G4 Boot-to-Shell**: Boot reaches interactive shell or defined diagnostic state.
- **G5 Terminal Restore**: Interactive shell restores terminal state on exit/crash.
- **G6 Security Review**: Security review complete for expansion, redirection, and execution.
- **G7 Built-in Contracts**: Built-ins have documented contracts and tests.
- **G8 Capability Claims**: Release claims match tested capability.

## Health Metrics Target

- Shell CI Pass Rate: 100%
- Documented POSIX Subset Pass Rate: 100%
- Parser Fuzz Crash Count: 0
- Built-ins with Contracts & Tests: 100%
- Exit-Code Behavior Documented: 100%
- Boot-to-Shell Smoke Pass Rate: 100%
- Shell Claims Evidence Backed: 100%
- Security PRs Reviewed: 100%
