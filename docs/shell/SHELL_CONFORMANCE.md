# POSIX Shell Conformance Specification

## Compatibility Target

SigmaOS shell implements a documented, testable subset of the POSIX.1-2017 Shell Specification (`sh`).

> **Note:** Full POSIX compliance is deferred until all subset tests in `tests/shell/conformance/` pass consistently.

## Documented POSIX Subset

1. **Simple Commands**: Name, argument lists, and environment assignments (`VAR=val cmd arg`).
2. **Quoting and Escaping**: Single quotes (`'...'`), double quotes (`"..."`), backslashes (`\`).
3. **Pipes and Redirections**: Sequential pipelines (`cmd1 | cmd2`), output redirection (`>`), append (`>>`), input redirection (`<`).
4. **Environment Variables**: Local vs exported environment variables, `export`, `set`.
5. **Exit Statuses**: Explicit exit codes for success (`0`), syntax error (`2`), command not found (`127`), and signal interruption (`128+N`).
6. **Compound Commands & Control Flow**:
   - `if list; then list; elif list; then list; else list; fi`
   - `while list; do list; done`
   - `for var in list; do list; done`
   - `case word in pattern) list ;; esac`
7. **Functions**: Function declarations `name() { list; }`.
8. **Built-in Commands**: `cd`, `pwd`, `echo`, `export`, `set`, `alias`, `exit`.
9. **Signal-Safe Script Execution**: Script processing halts immediately on non-recovered fatal signals or parse errors.

## Expansion Order

Parameter and word expansion MUST follow the exact POSIX sequence:

1. **Brace Expansion** (`{a,b}`)
2. **Tilde Expansion** (`~` / `~user`)
3. **Parameter Expansion** (`$VAR`, `${VAR}`)
4. **Command Substitution** (`$(cmd)` or `` `cmd` ``)
5. **Arithmetic Expansion** (`$((expr))`)
6. **Word Splitting** (Field splitting via `$IFS`)
7. **Pathname / Glob Expansion** (`*.rs`, `?`)
8. **Quote Removal** (Stripping unquoted `'`, `"`, `\`)

## Quoting Invariants

- **Single Quotes (`'...'`)**: Preserve the literal value of all characters inside the quotes. No parameter expansion, command substitution, arithmetic expansion, or globbing occurs.
- **Double Quotes (`"..."`)**: Preserve literal value except for `$`, `` ` ``, `\`, and `!`. Parameter and command expansions inside double quotes do NOT undergo word splitting or glob expansion.

## Conformance Verification

All conformance rules are validated via `tests/shell/conformance/conformance_test.rs` and `tests/shell/posix/posix_test.rs`.
