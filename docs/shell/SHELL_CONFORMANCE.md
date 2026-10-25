# SigmaOS Shell POSIX Conformance Baseline

## Overview

SigmaOS shell (`sigma-sh`) target compatibility model is an explicitly defined, test-backed subset of POSIX IEEE Std 1003.1-2024 Shell & Utilities (`sh`). Claims of compatibility apply solely to the documented baseline suite under `tests/shell/posix/` and `tests/shell/conformance/`.

## Supported POSIX Subset

### 1. Command Execution
- **Simple Commands**: Arguments, environment variable assignments (`VAR=val cmd`).
- **Pipelines**: Single (`cmd1 | cmd2`) and multi-stage pipelines with exit status inherited from the final pipeline command.
- **Redirections**: Output truncation (`>`), output append (`>>`), input redirection (`<`), and file descriptor duplication (`2>&1`).
- **Sequential Execution**: Command lists (`cmd1 ; cmd2`), conditional AND (`cmd1 && cmd2`), conditional OR (`cmd1 || cmd2`).

### 2. Parameter & Word Expansion Order
1. **Brace Expansion** (Extension)
2. **Tilde Expansion** (`~`, `~user`)
3. **Parameter Expansion** (`$VAR`, `${VAR}`)
4. **Command Substitution** (`$(cmd)`, `` `cmd` ``)
5. **Arithmetic Expansion** (`$((1 + 2))`)
6. **Word Splitting** (IFS whitespace splitting on unquoted parameters)
7. **Pathname Expansion** (Glob patterns `*`, `?`, `[...]` on unquoted fields)
8. **Quote Removal** (Removal of outer `'` and `"`)

### 3. Control Flow Constructs
- `if condition; then list; elif condition; then list; else list; fi`
- `while condition; do list; done`
- `for var in list; do list; done`
- `case word in pattern) list ;; esac`

### 4. Quoting Integrity
- **Single Quotes (`'...'`)**: Preserves the literal value of all characters. No parameter, glob, or command expansion occurs.
- **Double Quotes (`"..."`)**: Preserves whitespace and literal text while allowing parameter expansion (`$VAR`), command substitution (`$(cmd)`), and arithmetic expansion.
- **Backslash (`\`)**: Escapes the immediately following character.

### 5. Signals & Script Safety
- **Non-Interactive Mode**: Parse errors abort execution immediately without partial execution. Nonzero exit statuses are preserved.
- **Interactive Mode**: Traps SIGINT (`Ctrl-C`) to clear current prompt/line buffer and returns a clean new prompt.

## Test Suite Structure

- `tests/shell/conformance/`: Core pipeline, redirection, and execution conformance tests.
- `tests/shell/posix/`: POSIX quoting, parameter expansion, and control-flow tests.
- `tests/shell/regression/`: Regressions and bug fix verifications.
- `tests/shell/interactive/`: Terminal line buffer, history, and signal tests.
