# SigmaOS Shell Maintenance Program

## Maintenance Objective

> **Maintain SigmaOS shell as a safe, predictable, script-compatible command interpreter that boots, runs real commands, handles failures cleanly, and remains testable in CI.**

The near-term priority is validating the **boot-to-interactive-shell path**, establishing the shell as the observable endpoint of kernel and system initialization.

## Task Calendar

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

## Scope & Components

| Component | Responsibility |
|---|---|
| Lexer | Converts input into tokens |
| Parser | Builds commands, pipelines, lists, and control-flow structures |
| Expander | Performs variable, glob, tilde, command, arithmetic, and quote expansion |
| Executor | Runs built-ins, external binaries, pipelines, redirections, and jobs |
| Job control | Manages foreground/background processes, signals, and terminal state |
| Line editor | Handles prompt, history, cursor movement, completion, and multiline input |
| Built-ins | Implements `cd`, `export`, `exit`, `echo`, `pwd`, `set`, `alias`, etc. |
| Script runner | Executes non-interactive scripts with deterministic exit behavior |
| Diagnostics | Produces useful parse, runtime, and exit-status errors |

## Weekly Triage Template

Classify every shell issue using:

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

## First 12 Shell PRs Pipeline

1. `docs(shell): add shell maintenance calendar`
2. `docs(shell): define POSIX compatibility subset`
3. `docs(shell): add grammar and expansion-order specification`
4. `test(shell): add lexer and parser unit tests`
5. `feat(shell): implement deterministic script execution`
6. `feat(shell): add exit-code policy and tests`
7. `feat(shell/builtins): add cd, pwd, echo, export contracts`
8. `test(shell): add pipeline and redirection conformance tests`
9. `feat(shell): add interactive prompt and Ctrl-C handling`
10. `feat(fuzz): add lexer, parser, and expansion fuzz targets`
11. `test(boot): add boot-to-shell smoke validation`
12. `docs(release): add shell release gates`

## Acceptance Gates (G0 - G8)

- **G0**: `cargo check --lib`, formatting, and unit tests pass
- **G1**: Shell unit and parser tests pass
- **G2**: Documented POSIX subset tests pass
- **G3**: Fuzz smoke passes without crashes
- **G4**: Boot reaches shell or defined diagnostic state
- **G5**: Interactive shell restores terminal state
- **G6**: Security review complete for expansion, redirection, and execution
- **G7**: Built-ins have documented contracts and tests
- **G8**: Release claims match tested capability

## Health Metrics

- Shell CI pass rate: 100%
- Documented POSIX subset pass rate: 100%
- Parser fuzz crash rate: Zero known reproducible crashes
- Built-ins with contracts and tests: 100%
- Exit-code behavior documented: 100%
- Boot-to-shell smoke pass rate: 100%
- Shell claims linked to evidence: 100%
- Security-sensitive shell PRs reviewed: 100%
