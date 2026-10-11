# SigmaOS Shell Maintenance Framework

> **Maintain SigmaOS shell as a safe, predictable, script-compatible command interpreter that boots, runs real commands, handles failures cleanly, and remains testable in CI.**

The first shell milestone is a **boot-to-interactive-shell path**: SigmaOS’s near-term priority is validating boot-to-shell, making the shell the observable endpoint of kernel and userspace initialization.

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

## First 12 Shell PR Roadmap

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
| 12 | `docs(release): add shell release gates` | Prevent premature “usable shell” claims |

## Shell Acceptance Gates

| Gate | Requirement |
|---|---|
| G0 | `cargo check --lib`, formatting, and unit tests pass |
| G1 | Shell unit and parser tests pass |
| G2 | Documented POSIX subset tests pass |
| G3 | Fuzz smoke passes |
| G4 | Boot reaches shell or defined diagnostic state |
| G5 | Interactive shell restores terminal state |
| G6 | Security review complete for expansion, redirection, and process execution |
| G7 | Built-ins have documented contracts and tests |
| G8 | Release claims match tested capability |

## Shell Health Metrics

| Metric | Target |
|---|---|
| Shell CI pass rate | 100% |
| Documented POSIX subset pass rate | 100% |
| Parser fuzz crash rate | Zero known reproducible crashes |
| Built-ins with contracts and tests | 100% |
| Exit-code behavior documented | 100% |
| Boot-to-shell smoke pass rate | 100% |
| Shell claims linked to evidence | 100% |
| Security-sensitive shell PRs reviewed | 100% |

## Issue Template

Every shell issue must be classified as: parser / expansion / executor / job control / built-in / UX / security / performance.

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
