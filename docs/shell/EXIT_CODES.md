# SigmaOS Shell Exit-Code Policy

## Principles

1. `0` means success.
2. Nonzero values (`1`..`255`) indicate failure or abnormal termination.
3. Every command path defines success, failure, signal, and parse-error exit behavior.
4. Non-interactive scripts preserve the final command's exit status unless explicitly overridden.

## Standard Exit Codes

| Exit Code | Meaning | Context / Enforcement |
|---|---|---|
| `0` | Success | Normal command execution completed with success |
| `1` | General error | Runtime error, built-in failure, invalid arguments |
| `2` | Parse error / Misuse of built-in | Syntax error in command input or invalid built-in options |
| `126` | Command invoked cannot execute | Permission denied or target file is not executable |
| `127` | Command not found | Executable target does not exist in `PATH` |
| `128 + N` | Terminated by signal `N` | Fatal signal termination (`SIGINT` = 130, `SIGKILL` = 137, `SIGTERM` = 143) |

## Script Termination Rules

- Parse error in non-interactive script mode stops script execution immediately and returns exit status `2`.
- `set -e` (errexit) causes non-interactive script to exit immediately if any pipeline or command returns a nonzero status, unless part of an `if`, `while`, `until`, `&&`, or `||` test.
