# Study: Shell word splitting

## References
- POSIX: Issue 7, Section 2.6.5 Field Splitting
- Bash: Section 3.5.7 Word Splitting
- dash: `src/expand.c` (`ifsbreakup`)
- BusyBox ash: `shell/ash.c` (`evalvar`)
- FreeBSD sh: `bin/sh/expand.c`

## Rule under study
- Input: Parameter expansions, command substitutions, arithmetic expansions when unquoted.
- Expansion order: Occurs after parameter/command/arithmetic expansion, before pathname expansion (globbing).
- Quoting interaction: Double-quoted expansions (`"$VAR"`) suppress word splitting entirely.
- Empty-field behavior: Null fields are discarded unless enclosed in quotes.
- Exit-status behavior: Does not modify exit code.

## SigmaOS current behavior
- Code: `src/shell/sigma_sh.rs`, `src/shell/zsh_bash_parity.rs`
- Tests: `tests/shell_lifecycle_tests.rs`
- Runtime evidence: `test_expansion_order_sequence`
- Gaps: Edge cases with custom `$IFS` field separators.

## Decision
- Adopt: POSIX standard field splitting on `$IFS` (default space, tab, newline).
- Adapt: High-performance split iterator avoiding unnecessary heap reallocations.
- Reject: Non-standard word splitting on single-quoted strings.

## PR plan
- [x] fix(shell): enforce IFS word splitting in expansion engine
- [x] test(shell): add word splitting conformance unit tests
- [x] docs(shell): add 2026-W41 study note
