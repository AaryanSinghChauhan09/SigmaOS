# SigmaOS Shell Conformance & POSIX Subset Specification

## Overview

SigmaOS shell adheres to an explicit, documented POSIX shell (`sh`) compatibility subset. Claims of compliance are strictly limited to the documented subset validated by test suites in `tests/shell/conformance/` and `tests/shell/posix/`.

## Compatibility Subset Dimensions

### 1. Simple Commands
- Command name evaluation followed by argument list evaluation.
- Field splitting applied to unquoted expansions.
- Path lookup using `PATH` environment variable.

### 2. Arguments and Quoting
- **Single quotes (`'...'`)**: Preserves the literal value of all characters within quotes. No parameter expansion, command substitution, arithmetic expansion, or globbing occurs.
- **Double quotes (`"..."`)**: Suppresses word splitting and pathname expansion while allowing parameter expansion (`$VAR`), command substitution (`$(cmd)`), and escape sequences (`\"`, `\$`, `\\`, `\``).
- **Backslash (`\`)**: Escapes the immediately following character.

### 3. Expansion Order
Must follow strict POSIX ordering:
1. Brace Expansion (`{a,b}`)
2. Tilde Expansion (`~`, `~user`)
3. Parameter Expansion (`$VAR`, `${VAR}`)
4. Command Substitution (`$(cmd)`, `` `cmd` ``)
5. Arithmetic Expansion (`$((exp))`)
6. Word Splitting (based on `IFS`)
7. Pathname Expansion / Globbing (`*`, `?`, `[...]`)
8. Quote Removal

### 4. Pipelines & Execution
- Sequential pipeline execution (`cmd1 | cmd2 | cmd3`).
- Pipeline exit status defaults to the exit status of the final command, or `pipefail` when enabled.

### 5. Redirections
- Input redirection (`< file`).
- Output redirection (`> file`, `>> file`).
- Standard error redirection (`2> file`, `2>&1`).
- File descriptor validation and restoration on failure.

### 6. Environment Variables
- Environment inheritance on `fork`/`exec`.
- Exporting variables via `export KEY=VAL`.
- Pre-command assignment (`KEY=VAL command`).

### 7. Control Structures
- `if ... then ... else ... fi`
- `while ... do ... done`
- `for var in ... ; do ... done`
- `case ... in ... esac`

### 8. Functions
- Function definition syntax (`func() { ... }`).
- Positional parameters (`$1`, `$2`, `$@`, `$*`) scoped within functions.

### 9. Built-in Commands
- Core POSIX built-ins: `cd`, `pwd`, `echo`, `export`, `exit`, `set`, `alias`, `unalias`, `unset`, `shift`, `read`, `eval`, `exec`.

### 10. Signal Safety
- Signal-safe script termination on `SIGINT`, `SIGTERM`, `SIGQUIT`, and `SIGPIPE`.
