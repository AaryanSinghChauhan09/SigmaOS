# SigmaOS Shell Exit-Code Policy

## Standard Exit Codes

SigmaOS shell enforces a deterministic exit-code policy matching POSIX standards and standard Unix shell conventions.

| Exit Code | Meaning | Example Cause |
|---|---|---|
| `0` | Success | Command completed without error |
| `1` | General Runtime Error | Operation failure, file error, invalid `cd` |
| `2` | Parse / Syntax Error | Unclosed quote, syntax misuse, bad flag to `set` |
| `126` | Command Not Executable | Permission denied on script binary, no exec permissions |
| `127` | Command Not Found | Executable name not found in `$PATH` |
| `128 + N` | Terminated by Signal N | Process killed by signal N (e.g. `130` for SIGINT / Ctrl-C) |

## Propagation Rules

1. **Simple Commands**: The exit status of a simple command is its process termination status or built-in exit code.
2. **Pipelines**: By default, the exit status of a pipeline is the exit status of the **last command** in the pipeline.
3. **Control Flow Statements**: The exit status of `if`, `while`, or `for` is the exit status of the last command executed in the body (or `0` if no body command executed).
4. **Scripts**: Scripts return the exit status of the last executed command unless terminated prematurely by an unhandled error or explicit `exit`.
5. **Parse Errors in Scripts**: A parse error in a non-interactive script immediately halts execution with exit code `2`.
