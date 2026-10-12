# Study: Shell word splitting

## References
- POSIX: IEEE Std 1003.1-2017 Section 2.6.5 Word Splitting
- Bash: Bash Reference Manual Section 3.5.7 Word Splitting
- dash: dash(1) manual page
- BusyBox ash: BusyBox source `shell/ash.c`
- FreeBSD sh: FreeBSD sh(1) manual page

## Rule under study
- Input: Parameters expanded without double quotes (`$VAR` vs `"$VAR"`)
- Expansion order: Parameter expansion occurs before word splitting and pathname expansion.
- Quoting interaction: Double-quoted parameters (`"$VAR"`) suppress word splitting.
- Empty-field behavior: Null fields generated from unquoted parameter expansions that evaluate to empty strings are removed.
- Exit-status behavior: Word splitting itself does not alter `$?`.

## SigmaOS current behavior
- Code: `src/shell/sigma_sh.rs`, `src/shell/posix_shell.rs`
- Tests: `tests/shell_lifecycle_tests.rs`
- Runtime evidence: `test_quoting_and_glob_expansion_rules`
- Gaps: Edge cases in custom `$IFS` field separators.

## Decision
- Adopt: POSIX standard `$IFS` word splitting sequence for unquoted expansions.
- Adapt: Fallback to space/tab/newline when `$IFS` is unset.
- Reject: Non-standard Zsh word splitting defaults in default POSIX mode.

## PR plan
- [x] fix(shell): handle quoted vs unquoted word splitting order
- [x] test(shell): add word splitting regression tests
- [x] docs(shell): add word splitting study documentation
