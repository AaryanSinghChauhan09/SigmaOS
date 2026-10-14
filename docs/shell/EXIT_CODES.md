# SigmaOS Shell Exit-Code Policy

## Standards

`sigma-sh` strictly complies with standard UNIX/POSIX exit code conventions:

| Exit Code | Meaning | Example Condition |
|---|---|---|
| `0` | Success | Command completed successfully |
| `1` | General Failure | Built-in failure, invalid arguments |
| `2` | Parse / Syntax Error or Misuse | Unclosed quote, invalid builtin option |
| `126` | Permission Denied / Cannot Execute | Binary found but not executable |
| `127` | Command Not Found | Program not in `$PATH` |
| `128 + N` | Fatal Signal `N` | `130` for `SIGINT` (128 + 2), `137` for `SIGKILL` (128 + 9) |

## Rules

1. **Non-Interactive Script Exit Behavior**:
   - The exit status of a script is the exit status of the last command executed in the script unless overridden by an explicit call to `exit`.
   - On parse error, script execution aborts immediately with exit status `2`.

2. **Pipeline Exit Behavior**:
   - By default, the pipeline exit status is that of the rightmost command in the pipeline.
   - If `set -o pipefail` is set, the pipeline exit status is that of the rightmost command that failed with non-zero, or zero if all succeeded.

3. **Subshell Exit Behavior**:
   - Subshells `( ... )` capture and return the exit code of their internal command list.
