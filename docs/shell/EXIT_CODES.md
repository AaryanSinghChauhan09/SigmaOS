# SigmaOS Shell Exit Code Policy

## Standard Exit Status Codes

| Exit Code | Meaning | Example Scenario |
|---|---|---|
| `0` | Success | Command completed without error |
| `1` | General error | Application or built-in operational failure |
| `2` | Parse error / Misuse | Syntax error, unclosed quote, invalid command option |
| `126` | Command non-executable | Binary found but permissions deny execution |
| `127` | Command not found | Command missing from PATH |
| `128 + N` | Terminated by Signal `N` | `130` for SIGINT (Ctrl-C), `137` for SIGKILL, `143` for SIGTERM |

## Rules
1. Non-interactive script mode propagates the exit code of the final executed command as the script's exit status.
2. In non-interactive mode, any parse error results in immediate exit code `2` without executing partially parsed commands (`SHELL_RULES.toml`: `rule.no_partial_execution_on_parse_error`).
3. Redirection failures return exit code `1`.
