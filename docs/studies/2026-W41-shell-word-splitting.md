# Study: Shell word splitting

## References
- POSIX: POSIX.1-2017 Vol 3, Section 2.6.5 (Field Splitting)
- Bash: Bash Reference Manual Section 3.5.7 (Word Splitting)
- dash: dash(1) man page, Expansion section
- BusyBox ash: BusyBox ash shell expansion engine
- FreeBSD sh: FreeBSD sh(1) field splitting implementation

## Rule under study
- Input: Unquoted parameter expansions (`$var`), command substitutions (`$(cmd)`), and arithmetic expansions (`$((expr))`).
- Expansion order: Occurs after parameter, command, and arithmetic expansion, but before pathname expansion (globbing).
- Quoting interaction: Quoted expansions (`"$var"`) undergo zero word splitting.
- Empty-field behavior: `IFS` characters (space, tab, newline by default) delimit fields. Null fields at start or end are discarded unless double-quoted.
- Exit-status behavior: Word splitting does not modify exit status of the underlying expansion.

## SigmaOS current behavior
- Code: `src/shell/sovereign_command_interpreter.rs` & `src/shell/posix_shell.rs`
- Tests: `tests/shell.rs`, `tests/shell/conformance/conformance_test.rs`
- Runtime evidence: `test_posix_conformance_subset_expansion_order` passes step order checks.
- Gaps: Unquoted variable expansion inside custom script loops requires strict handling of custom `IFS` delimiters.

## Decision
- Adopt: Standard POSIX `IFS` field splitting rules for unquoted expansions.
- Adapt: Preserve explicit field boundary markers in token representations.
- Reject: Non-standard Bash extensions to word splitting that diverge from POSIX sh.

## PR plan
- [x] fix(shell): preserve quoted field boundaries during expansion
- [x] test(shell): add word splitting POSIX conformance test vectors
- [x] docs(shell): document expansion order in docs/shell/GRAMMAR.md
