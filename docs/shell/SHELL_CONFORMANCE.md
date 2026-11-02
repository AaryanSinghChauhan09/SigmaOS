# SigmaOS Shell Conformance

Track supported POSIX-inspired behavior, deviations, and evidence here. Each
new behavior requires a focused test, an expected exit status, and a documented
compatibility note before promotion.

Do **not** claim full POSIX compliance until the suite defines and passes a documented subset.

## Target POSIX Compatibility Subset

SigmaOS shell (`sigma-sh`) targets a verified subset of POSIX.1-2017 (Shell & Utilities):

### 1. Simple Commands
- Command word lookup (built-in first, then PATH lookup).
- Argument list assembly without arbitrary truncation.
- Environment variable passing to child processes.

### 2. Arguments and Quoting
- Single quotes (`'...'`): Preserve the literal value of all characters inside quotes. No expansion.
- Double quotes (`"..."`): Suppress word splitting and globbing; permit parameter expansion (`$VAR`), command substitution, and arithmetic expansion.
- Escape character (`\`): Escapes special characters outside and inside double quotes.

### 3. Pipelines
- Syntax: `command1 | command2 | command3`
- Standard output of `command1` connected to standard input of `command2` via kernel pipe.
- Exit status: Deterministically reports the exit status of the last command in the pipeline (or honors `pipefail` when enabled).

### 4. Redirections
- `< filename`: Redirect standard input (fd 0) from file.
- `> filename`: Redirect standard output (fd 1) to file (truncate).
- `>> filename`: Redirect standard output (fd 1) to file (append).
- `2> filename`: Redirect standard error (fd 2) to file.
- `>&2` or `2>&1`: File descriptor duplication.

### 5. Environment Variables & Parameter Expansion
- `$VAR` and `${VAR}` expansion.
- `$?` represents the exit status of the last executed foreground command.
- `$$` represents the shell process ID.
- Variable assignment syntax: `NAME=value [command]`.

### 6. Deterministic Exit Statuses
- `0`: Successful command completion.
- `1`: General command error or non-zero builtin return.
- `2`: Shell parse error / syntax error.
- `126`: Command invoked cannot execute (permission denied / not executable).
- `127`: Command not found in PATH or built-ins.
- `128 + N`: Terminated by fatal signal `N` (e.g. 130 for SIGINT).

### 7. Control Flow Constructs
- `if ... then ... elif ... else ... fi`
- `while ... do ... done`
- `for var in ...; do ... done`
- `case word in pattern) ... ;; esac`

### 8. Functions
- Syntax: `name() { compound-command; }`
- Local vs global scope execution.

### 9. Core Built-ins
- `cd`, `pwd`, `echo`, `export`, `alias`, `set`, `exit`.

### 10. Signal-Safe Script Termination
- SIGINT / SIGTERM traps and clean abort in non-interactive script runners.
- Terminal restoration guaranteed upon exit or interruption.
