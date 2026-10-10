# Study: Shell word splitting

## References
- POSIX: IEEE Std 1003.1-2017, Section 2.6.5 Word Splitting
- Bash: Bash Reference Manual 3.5.7 Word Splitting
- dash: dash(1) Parameter Expansion & Word Splitting
- BusyBox ash: busybox ash word splitting implementation
- FreeBSD sh: sh(1) Field Splitting

## Rule under study
- Input: Unquoted parameter expansion `${VAR}` where `IFS` is default (`<space><tab><newline>`).
- Expansion order: Occurs AFTER parameter, command, and arithmetic expansion, but BEFORE pathname expansion.
- Quoting interaction: Unquoted expansion results are split into fields. Quoted expansion results (`"${VAR}"`) retain internal whitespace and are NOT split.
- Empty-field behavior: Null fields generated at start/end or consecutive internal whitespace delimiters are discarded.
- Exit-status behavior: Word splitting does not modify exit status directly.

## SigmaOS current behavior
- Code: `src/shell/zsh_bash_parity.rs`, `src/shell/sigma_sh.rs`
- Tests: `tests/shell_lifecycle_tests.rs`
- Runtime evidence: Verified word splitting behavior in parameter expansion tests.
- Gaps: Edge cases in custom non-standard `$IFS` delimiter characters.

## Decision
- Adopt: POSIX field splitting rules for default `$IFS` space, tab, and newline delimiters.
- Adapt: Fallback to single-space delimiter splitting when `$IFS` is unset.
- Reject: Non-standard Bash string manipulation side-effects during splitting.

## PR plan
- [x] fix(shell): preserve quoted whitespace in parameter expansion
- [x] test(shell): add word splitting conformance unit test
- [x] docs(shell): record word splitting study
