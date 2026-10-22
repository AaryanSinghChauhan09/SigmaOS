# Shell POSIX Conformance & Specification Matrix

## POSIX IEEE Std 1003.1-2017 Conformance Status
- **Simple Commands & Pipelines**: Supported (`cmd1 | cmd2`).
- **Redirection**: Supported (`<`, `>`, `>>`, `2>&1`).
- **Variable Expansion**: Parameter expansion (`$VAR`, `${VAR}`), exit status (`$?`), PID (`$$`).
- **Quote Handling**: Single quotes `'...'` (literal), double quotes `"..."` (variable expansion), backslash escape `\`.
- **Exit Codes**:
  - `0`: Success.
  - `1`: General execution error.
  - `2`: Syntax or parse error.
  - `126`: Command found but not executable.
  - `127`: Command not found.
  - `128+N`: Terminated by signal N.
