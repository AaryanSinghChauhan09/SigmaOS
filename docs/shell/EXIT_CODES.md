# SigmaOS Shell Exit Code Policy

## Rules

| Exit Code | Meaning |
|---|---|
| `0` | Success / Normal termination |
| `1` | General error (impermissible argument, runtime failure) |
| `2` | Parse error, syntax error, or command misuse |
| `126` | Command found but not executable / permission denied |
| `127` | Command not found |
| `128 + N` | Fatal signal `N` (e.g. `130` for SIGINT, `137` for SIGKILL, `143` for SIGTERM) |

## Propagation
- Non-interactive scripts terminate immediately on parse errors with exit status 2.
- Pipelines propagate the exit status of the rightmost (final) command.
- Scripts preserve the final executed command's exit code upon EOF.
