# SigmaOS Shell Exit Code Policy

## Overview

`sigma-sh` strictly enforces deterministic, observable exit codes for all command execution, parse errors, and signal terminations.

## Standard Exit Code Mappings

| Exit Code | Meaning | Description |
|---:|---|---|
| `0` | Success | Command or script completed successfully |
| `1` | General Failure | Runtime error, built-in error, or failed conditional test |
| `2` | Shell Parse / Syntax Error | Invalid syntax, missing token, unclosed construct, or command misuse |
| `126` | Command Cannot Execute | Target binary found but lacks execution permissions or valid ELF header |
| `127` | Command Not Found | Target executable or built-in command does not exist in `$PATH` |
| `128 + N` | Fatal Signal `N` | Process terminated by signal `N` (e.g. `128 + 2 = 130` for `SIGINT`, `128 + 9 = 137` for `SIGKILL`, `128 + 15 = 143` for `SIGTERM`) |

## Script Exit Behavior

1. **Sequential Execution**: A script's final exit status equals the exit status of the last executed command unless overridden by an explicit `exit n` call.
2. **Parse Errors**: Parse or lexer errors abort script execution immediately without executing any commands, returning exit code `2`.
3. **Errexit (`set -e`)**: When `set -e` is enabled, any pipeline or simple command returning a non-zero exit code (not inside an `if` condition or `||` branch) terminates script execution immediately.
4. **Pipeline Exit Code**: By default, a pipeline returns the exit status of its rightmost command. When `set -o pipefail` is active, the pipeline returns the exit status of the rightmost failed command.
