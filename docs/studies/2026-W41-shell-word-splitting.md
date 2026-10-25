# Study: Shell Word Splitting

## References
- POSIX: IEEE Std 1003.1-2024 Shell & Utilities Section 2.6.5
- Bash: Bash Reference Manual Section 3.5.7
- dash: dash(1) Parameter Expansion and Field Splitting
- BusyBox ash: `shell/ash.c` `evalvar` / `ifsbreak`
- FreeBSD sh: `bin/sh/expand.c` `expari` / `argstr`

## Rule under study
- **Input**: Results of parameter expansion, command substitution, and arithmetic expansion.
- **Expansion Order**: Word splitting occurs after arithmetic expansion and before pathname (glob) expansion.
- **Quoting Interaction**: Unquoted expansions undergo word splitting using `IFS` characters. Double-quoted expansions (`"$VAR"`) suppress word splitting.
- **IFS Rules**:
  - Default `IFS` is space, tab, newline (` \t\n`).
  - If `IFS` is null/unset, word splitting is disabled.
  - Non-whitespace `IFS` characters delimit fields strictly.
- **Empty-Field Behavior**: Unquoted empty parameter expansion is removed entirely. Double-quoted empty parameter expansion generates an empty field (`""`).
- **Exit-Status Behavior**: Word splitting does not modify command exit status.

## SigmaOS current behavior
- **Code**: `src/shell/sigma_sh.rs`, `src/shell/command.rs`, `tests/shell/conformance/conformance_test.rs`
- **Tests**: `tests/shell/conformance/conformance_test.rs`
- **Runtime Evidence**: Verified via standalone conformance test `test_conformance_expansion_order`.
- **Gaps**: Custom non-whitespace `IFS` delimiting requires edge-case coverage.

## Decision
- **Adopt**: POSIX standard `IFS` whitespace splitting semantics.
- **Adapt**: Limit default `IFS` splitting to space, tab, and newline in early boot environment.
- **Reject**: Non-standard Bash string manipulation side-effects during field splitting.

## PR plan
- [x] `fix(shell)`: enforce POSIX field splitting order after arithmetic expansion
- [x] `test(shell)`: add unit tests for quoted versus unquoted `IFS` parameter expansion
- [x] `docs(shell)`: document `IFS` word splitting rules in `docs/shell/GRAMMAR.md`
