# SigmaOS Shell Exit Code Policy

## Overview
To enforce `[rule.explicit_exit_status]`, every execution path defines explicit, observable exit status behavior.

## Exit Status Table

| Exit Code | Cause / Category | Description |
|---|---|---|
| `0` | Success | Command completed without errors |
| `1` | General Error | Catch-all for built-in or runtime command failure |
| `2` | Built-in Misuse / Parse Error | Invalid shell syntax or wrong built-in flags |
| `126` | Command Cannot Execute | Command file found, but not executable (permission denied) |
| `127` | Command Not Found | Specified command or binary does not exist in `$PATH` |
| `128 + N` | Signal Termination | Command terminated by fatal signal number `N` (e.g., `130` for SIGINT, `137` for SIGKILL, `143` for SIGTERM) |

## Propagation Rules

1. **Simple Commands**: Retains exit code returned by binary process or built-in function.
2. **Pipelines**:
   - Standard POSIX: Pipeline exit status is that of the LAST command in the pipeline.
   - `set -o pipefail`: Exit status is that of the last (rightmost) command to exit with non-zero status, or `0` if all succeed.
3. **Control Flow**:
   - `if` / `while` / `for`: Exit status is that of the last executed command inside the body, or `0` if condition was false/loop didn't run.
4. **Scripts**:
   - Non-interactive script preserves the exit code of the final executed statement.
   - Parse error aborts script execution immediately with exit code `2`.
