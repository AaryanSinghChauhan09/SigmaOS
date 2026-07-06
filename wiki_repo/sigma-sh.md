# sigma-sh — SigmaOS Shell Specification

**Status:** Draft · Target: v0.2
**Owner:** userland/shell team
**Canonical source:** `userland/sigma-sh/`

---

## Overview

sigma-sh is the default interactive shell and script interpreter for SigmaOS. It provides a POSIX-compatible REPL with SigmaOS-specific extensions, tight integration with sigma-bus IPC, and mandatory capability gating via `sigma_pledge`.

## Goals

- Minimal binary: < 256 KB stripped, zero dynamic library dependencies in minimal profile

- Full POSIX sh compatibility for script portability

- First-class tab completion and history with privacy-respecting defaults

- Native pipe/redirect to sigma-bus sockets, not just file descriptors

- Safe default: `sigma_pledge("stdio rpath proc exec")` on startup

---

## Architecture / Design

### REPL Loop

```
read_line() → tokenize() → expand() → parse_ast() → execute() → print_result()
        ↑______________________________________________↓  (loop)
```

1. **read_line**: readline-compatible line editor with Emacs key bindings

2. **tokenize**: splits on IFS, handles quoting (`'`, `"`, `\`)

3. **expand**: `$VAR`, `${VAR:-default}`, `$(cmd)`, `$((arith))`, glob `*?[]`

4. **parse_ast**: recursive-descent parser → AST nodes (cmd, pipe, redir, if, for, while, func)

5. **execute**: walks AST; fork/exec for external commands; builtin dispatch table for internals

### Fork / Exec Model

- `fork()` + `execvpe()` for external commands; PATH lookup via `$PATH` with hash cache

- Pipes: `pipe(2)` pairs wired before fork; both ends closed in correct child after dup2

- Signal mask reset in child before exec (inherits none of parent's masked signals)

### Builtins

| Builtin | Behaviour |
|---------|-----------|
| `cd [dir]` | chdir; updates `$OLDPWD`, `$PWD`; no-arg → `$HOME` |
| `exit [n]` | flush history, close IPC sockets, _exit(n) |
| `export VAR=val` | add/update process environment |
| `echo [-n] [-e]` | print args; `-e` enables escape sequences |
| `help [cmd]` | print builtin help or man-page stub |
| `source / .` | execute script in current shell context |
| `alias/unalias` | string substitution before tokenise |
| `read` | read one line into variable |
| `set/unset` | shell option flags (`-e -u -x -o pipefail`) |
| `jobs/fg/bg` | job control via process groups |

### Pipes and Redirects

- `cmd1 | cmd2` — anonymous pipe; last command's exit status is `$PIPESTATUS[-1]`

- `cmd > file`, `cmd >> file`, `cmd < file`, `cmd 2>&1`

- `cmd |& sigma-bus://service/endpoint` — native IPC pipe extension

- Here-doc `<<EOF` and here-string `<<<word`

---

## Signal Handling

| Signal | Action |
|--------|--------|
| `SIGINT` (Ctrl+C) | cancel current foreground job; redisplay prompt |
| `SIGQUIT` (Ctrl+\) | ignored in interactive mode |
| `SIGEOF` (Ctrl+D) | if line empty: exit; else: ignore |
| `SIGWINCH` | reflow line editor on terminal resize |
| `SIGCHLD` | reap zombie children; update job table |

---

## Tab Completion

Engine: `libedit`-compatible callback API.

1. Executable completion: scan `$PATH` entries, cache with inotify watch

2. File path completion: `readdir` relative to current token prefix

3. Builtin/alias completion: static table lookup

4. sigma-shard completion: query `sigma-bus://registry/list` for installed shard names

---

## History

- File: `~/.sigma_history` (configurable via `$SIGMA_HISTFILE`)

- Format: plain text, one command per line; optionally prefixed with `#timestamp`

- Max entries: 10 000 (configurable `$SIGMA_HISTSIZE`)

- Privacy: commands starting with a space are NOT recorded

- Shared across sessions via atomic append (O_APPEND)

---

## Environment Variables

`$PATH`, `$HOME`, `$USER`, `$SHELL=/usr/bin/sigma-sh`, `$SIGMA_HISTFILE`, `$SIGMA_HISTSIZE`, `$PS1`, `$PS2`, `$IFS`, `$OLDPWD`, `$PWD`, `$?`, `$!`, `$$`, `$0`

---

## Implementation Plan

- [ ] 1. Tokenizer + quoting engine (`src/lex.c`)

- [ ] 2. Recursive-descent parser → AST (`src/parse.c`)

- [ ] 3. Variable expansion engine (`src/expand.c`)

- [ ] 4. Fork/exec + PATH lookup (`src/exec.c`)

- [ ] 5. Pipe + redirect wiring (`src/redir.c`)

- [ ] 6. Builtin dispatch table (`src/builtins/`)

- [ ] 7. Job control (SIGCHLD handler, `jobs`/`fg`/`bg`)

- [ ] 8. Signal handler setup (SIGINT/SIGWINCH/SIGEOF)

- [ ] 9. Line editor integration (`src/readline.c`)

- [ ] 10. Tab completion callbacks

- [ ] 11. History load/save with privacy filter

- [ ] 12. `sigma_pledge` self-restriction on startup

- [ ] 13. sigma-bus pipe extension (`|&` operator)

- [ ] 14. Test suite: POSIX sh compliance (busybox test set), signal tests, history tests

---

## Status

| Milestone | State |
|-----------|-------|
| Tokenizer | ⬜ Not started |
| Parser | ⬜ Not started |
| Builtins | ⬜ Not started |
| Job control | ⬜ Not started |
| Tab completion | ⬜ Not started |
| History | ⬜ Not started |
| sigma-bus pipe | ⬜ Not started |
