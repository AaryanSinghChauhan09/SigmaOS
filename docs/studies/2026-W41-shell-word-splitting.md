# Study: Shell word splitting

## References
- POSIX: POSIX.1-2017 Section 2.6.5 "Field Splitting"
- Bash: Bash Reference Manual Section 3.5.7 "Word Splitting"
- dash: dash(1) Parameter Expansion and Field Splitting
- BusyBox ash: BusyBox ash field splitting semantics
- FreeBSD sh: sh(1) IFS Whitespace and Delimiter Splitting

## Rule under study
- Input: Unquoted parameter expansion `${VAR}` vs quoted parameter expansion `"${VAR}"`
- Expansion order: Occurs strictly after parameter expansion, command substitution, and arithmetic expansion, and prior to pathname expansion (globbing).
- Quoting interaction: Unquoted tokens undergo field splitting on `$IFS` characters (default space, tab, newline); double-quoted or single-quoted strings are exempt from field splitting.
- Empty-field behavior: Null fields at beginning or end of IFS-split results are discarded unless explicitly quoted.
- Exit-status behavior: Field splitting itself does not modify exit status `$?`.

## SigmaOS current behavior
- Code: `src/shell/interactive/` and `src/shell/command.rs`
- Tests: `tests/shell/conformance/conformance_test.rs` and `tests/shell_lifecycle_tests.rs`
- Runtime evidence: `test_conformance_expansion_order` and `test_quoting_and_glob_expansion_rules`
- Gaps: IFS customization via environmental configuration requires further evaluation in script runner mode.

## Decision
- Adopt: POSIX 8-stage expansion sequence with IFS word splitting on unquoted tokens.
- Adapt: Hardcode default whitespace IFS splitting in userspace core while environment IFS is stabilized.
- Reject: Non-standard word splitting on double-quoted strings.

## PR plan
- [x] fix(shell): preserve quoted whitespace and prevent field splitting inside double quotes
- [x] test(shell): add word splitting conformance unit tests
- [x] docs(shell): document 8-stage expansion order in GRAMMAR.md and SHELL_CONFORMANCE.md
