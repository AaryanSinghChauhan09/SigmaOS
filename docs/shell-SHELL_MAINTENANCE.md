# SigmaOS Shell Maintenance Program

## Maintenance Objective

> **Maintain SigmaOS shell as a safe, predictable, script-compatible command interpreter that boots, runs real commands, handles failures cleanly, and remains testable in CI.**

The near-term priority is validating the **boot-to-interactive-shell path**, providing an observable endpoint for kernel and userspace initialization.

## Shell Task Calendar

| Frequency | Shell Task | Output | Required PR |
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

## Shell Scope

The shell is divided into modular components:

| Component | Responsibility |
|---|---|
| Lexer | Converts input strings into structured token streams |
| Parser | Builds commands, pipelines, lists, and control-flow ASTs |
| Expander | Performs variable, glob, tilde, command, arithmetic, and quote expansion |
| Executor | Executes built-ins, external binaries, pipelines, redirections, and background jobs |
| Job Control | Manages foreground/background process groups, signals, and terminal control |
| Line Editor | Handles PS1 prompt, history, cursor navigation, completion, and multiline input |
| Built-ins | Implements `cd`, `export`, `exit`, `echo`, `pwd`, `set`, `alias`, etc. |
| Script Runner | Executes non-interactive scripts deterministically |
| Diagnostics | Formats parse errors, unknown command suggestions, and runtime failure messages |

## Recurring Development Tasks

### Weekly Triage Format

Issues must be classified as:
- Parser correctness
- Expansion correctness
- Process/pipe behavior
- Redirection
- Signal handling
- Built-in behavior
- Interactive UX
- Script compatibility
- Security/sandboxing
- Performance

```markdown
## Shell Issue

**Area:** parser / expansion / executor / job control / built-in / UX
**Input:** `<command>`
**Expected:** `<stdout, stderr, exit code>`
**Actual:** `<observed behavior>`
**POSIX/reference behavior:** `<explanation>`
**Acceptance test:** `<test name>`
```

### Weekly Compatibility Study

A weekly study note must analyze one narrow shell construct across POSIX, Bash, Zsh, dash, BusyBox ash, and FreeBSD sh, saved to `docs/studies/YYYY-Wxx-topic.md`.

## Acceptance Gates

| Gate | Requirement |
|---|---|
| G0 | `cargo check --lib`, formatting, and unit tests pass |
| G1 | Shell unit and parser tests pass |
| G2 | Documented POSIX subset tests pass |
| G3 | Fuzz smoke passes (`fuzz/fuzz_targets/shell_lexer_parser.rs`) |
| G4 | Boot reaches shell or defined diagnostic state |
| G5 | Interactive shell restores terminal state on exit |
| G6 | Security review complete for expansion and redirection |
| G7 | Built-ins have documented contracts and tests |
| G8 | Release claims match tested capability |

## Health Metrics

| Metric | Target |
|---|---|
| Shell CI pass rate | 100% |
| Documented POSIX subset pass rate | 100% |
| Parser fuzz crash rate | Zero known reproducible crashes |
| Built-ins with contracts and tests | 100% |
| Exit-code behavior documented | 100% |
| Boot-to-shell smoke pass rate | 100% |
| Shell claims linked to evidence | 100% |
