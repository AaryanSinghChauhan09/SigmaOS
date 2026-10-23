# SigmaOS Shell POSIX Conformance Matrix

## Overview
`sigma-sh` implements a deterministic subset of IEEE Std 1003.1-2017 (POSIX.1) shell standards optimized for bare-metal SigmaOS environments.

## Supported POSIX Expansion Order
1. **Brace Expansion**: `{a,b}`
2. **Tilde Expansion**: `~` -> `$HOME`
3. **Parameter Expansion**: `${VAR}`
4. **Command Substitution**: `$(command)`
5. **Arithmetic Expansion**: `$((1 + 1))`
6. **Word Splitting**: `$IFS` breaking
7. **Pathname Expansion**: `*`, `?`, `[...]`
8. **Quote Removal**: Removing outer quote characters

## Conformant Exit Codes
- `0`: Successful execution
- `2`: Command syntax error / parser failure
- `126`: Command found but not executable
- `127`: Command not found
- `128 + N`: Terminated by signal N (e.g., `130` for SIGINT)
