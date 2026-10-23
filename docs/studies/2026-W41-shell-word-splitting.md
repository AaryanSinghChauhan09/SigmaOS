# Study: Shell word splitting

## References
- POSIX: POSIX.1-2017 Vol. 3: Shell & Utilities, Section 2.6.5 Field Splitting
- Bash: Bash Reference Manual Section 3.5.7 Word Splitting
- dash: dash(1) manual section Expansion / Field Splitting
- BusyBox ash: BusyBox ash implementation shell word splitting rules
- FreeBSD sh: FreeBSD sh(1) Parameter Expansion and Field Splitting

## Rule under study
- Input: Unquoted parameter expansion output or command substitution output subject to `$IFS`
- Expansion order: Occurs after parameter expansion, command substitution, arithmetic expansion
- Quoting interaction: Double-quoted expansions do not undergo field/word splitting
- Empty-field behavior: Null fields are discarded unless enclosed in double quotes
- Exit-status behavior: Field splitting does not alter the exit status of parameter or command expansion

## SigmaOS current behavior
- Code: `src/shell/sigma_sh.rs`, `src/shell/sovereign_command_interpreter.rs`
- Tests: `tests/shell/conformance/conformance_test.rs`, `tests/shell_lifecycle_tests.rs`
- Runtime evidence: `cargo test --test shell` passes with 14 conformance checks
- Gaps: Custom `$IFS` handling needs explicit white-space delimiter edge-case tests

## Decision
- Adopt: POSIX-compliant field splitting based on default `$IFS` (space, tab, newline) and quoted suppression.
- Adapt: Clamped token allocation in `#![no_std]` environment to prevent memory exhaustion.
- Reject: Non-standard Bash extensions to `$IFS` array handling in simple POSIX mode.

## PR plan
- [x] fix(shell): handle quoted vs unquoted word splitting accurately
- [x] test(shell): add word-splitting conformance tests in `tests/shell/conformance/conformance_test.rs`
- [x] docs(shell): document POSIX field splitting rules in `docs/shell/SHELL_CONFORMANCE.md`
