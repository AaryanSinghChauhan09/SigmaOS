# SigmaOS Shell POSIX Conformance Specification

## Overview
SigmaOS Shell (`sigma-sh`) implements a explicitly defined subset of POSIX IEEE Std 1003.1-2017 (Shell Command Language). Full POSIX conformance is not claimed until all compliance targets in this specification pass in CI.

## Supported POSIX Compatibility Subset

### 1. Simple Commands
- Command word lookup: built-ins, functions, and standard binary executables.
- Argument vector construction (`argv`).
- Environment inheritance and local assignment syntax (`VAR=val command`).

### 2. Quoting Rules
- Single quotes (`'...'`): Preserve the literal value of all enclosed characters.
- Double quotes (`"..."`): Preserve literal value except for `$`, `` ` ``, `\`, and `!`.
- Escape character (`\`): Escapes the following character.

### 3. Pipeline & Command Separation
- Sequential execution: `;` and newline.
- Asynchronous execution: `&`.
- Pipelines: `|` (exit code is status of rightmost command).
- Logical AND / OR: `&&` and `||`.

### 4. Redirections
- Input redirection: `< file`
- Output redirection: `> file` (truncates) and `>> file` (appends)
- File descriptor duplication: `2>&1`, `1>&2`
- Here-documents: `<< EOF`

### 5. Parameter & Variable Expansion
- Basic variable expansion: `$VAR` and `${VAR}`
- Exit status parameter: `$?`
- Process ID parameter: `$$`
- Default value assignment: `${VAR:-default}`, `${VAR:=default}`

### 6. Control Flow Syntax
- `if condition; then list; elif list; else list; fi`
- `while condition; do list; done`
- `for var in list; do list; done`
- `case word in pattern) list ;; esac`

### 7. Core Built-In Commands
- `cd`, `pwd`, `echo`, `export`, `unset`, `exit`, `alias`, `unalias`, `set`, `read`, `eval`, `exec`.

## Non-Goal Extensions Excluded from POSIX Baseline
- Process substitution (`<(cmd)`)
- Extended globbing (`+(...)`)
- Advanced array operations (POSIX `sh` does not support indexed arrays)
