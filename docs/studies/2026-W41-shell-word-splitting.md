# Study: Shell word splitting

## References
- POSIX: IEEE Std 1003.1-2017, Section 2.6.5 Field Splitting
- Bash: Bash Reference Manual, Section 3.5.7 Word Splitting
- dash: dash(1) manual, Word Splitting
- BusyBox ash: ash shell word splitting implementation
- FreeBSD sh: sh(1) Field Splitting

## Rule under study
- Input: Unquoted parameter expansion `$VAR` or command substitution `$(cmd)`
- Expansion order:
  1. Brace Expansion
  2. Tilde Expansion
  3. Parameter Expansion
  4. Command Substitution
  5. Arithmetic Expansion
  6. **Word Splitting (Field Splitting)**
  7. Pathname Expansion (Globbing)
  8. Quote Removal
- Quoting interaction: Fields enclosed in double quotes `"$VAR"` do NOT undergo word splitting. Single quotes `'...'` prevent parameter expansion altogether.
- Empty-field behavior: If expansion results in an empty string and is unquoted, it is removed. If quoted `""`, an empty string field is preserved.
- Exit-status behavior: Word splitting does not modify exit status directly, but affects argument counts passed to utility execution.

## SigmaOS current behavior
- Code: `src/shell/sigma_sh.rs`, `src/shell/zsh_bash_parity.rs`
- Tests: `tests/shell/conformance/conformance_test.rs`
- Runtime evidence: Word splitting adheres to standard IFS whitespace delimiters (`<space>`, `<tab>`, `<newline>`).
- Gaps: Custom non-whitespace IFS character splitting requires extended buffer tracking.

## Decision
- Adopt: POSIX field splitting rules using default `IFS=$' \t\n'`.
- Adapt: Ensure zero allocation overhead during default whitespace splitting in `#![no_std]` runtime environments.
- Reject: Non-standard Bash double-quote splitting quirks.

## PR plan
- [x] fix(shell): preserve quoted field boundaries during expansion
- [x] test(shell): add word splitting test cases for default IFS
- [x] docs(shell): document IFS field splitting in POSIX conformance guide
