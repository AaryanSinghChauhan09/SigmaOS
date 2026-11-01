# SigmaOS Shell Exit Codes

SigmaOS shell enforces a deterministic exit-code policy modeled on POSIX and standard Unix shells:

- `0`: Successful completion of command or script.
- `1`: General command execution failure or built-in error.
- `2`: Shell parse error or syntax error (e.g. unclosed block, invalid token).
- `126`: Command invoked cannot execute (permission denied or not executable binary).
- `127`: Command not found in system `$PATH` or internal built-in registry.
- `128 + N`: Process terminated by fatal signal `N` (e.g., `130` for SIGINT, `137` for SIGKILL, `143` for SIGTERM).

## Exit Propagation Rules

1. **Script Termination**: The exit status of a script is the exit status of the last command executed within it, unless interrupted or aborted early by `set -e`.
2. **Pipelines**: By default, the exit status of a pipeline is the exit status of the last command in the pipeline sequence. When `set -o pipefail` is active, the exit status is the value of the last (rightmost) command to exit with a non-zero status, or zero if all commands succeeded.
3. **Parse Errors**: Parse errors in non-interactive mode immediately abort execution and yield exit code `2`. No partial command execution occurs.
4. **Signal Interrupts**: If a shell process receives `SIGINT` (Ctrl-C), it exits with status code `130` (`128 + 2`).
