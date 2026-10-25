# SigmaOS Shell POSIX Conformance Subset

## Supported POSIX Subset

SigmaOS shell implements a documented, safe POSIX-compatible subset:

### Simple Commands
- Command execution with whitespace-separated arguments
- Absolute and relative path resolution
- Path searching via `PATH` environment variable

### Quoting & Escaping
- Single quotes (`'...'`): Preserves literal value of all characters
- Double quotes (`"..."`): Suppresses word splitting and globbing while allowing parameter expansion
- Backslash (`\`): Escapes single characters

### Pipelines & Redirections
- Sequential pipelines: `cmd1 | cmd2 | cmd3`
- Input redirection: `< file`
- Output redirection: `> file`
- Append redirection: `>> file`
- Exit status propagation: Pipeline exit status equals status of final command

### Environment Variables & Expansions
- Variable expansion: `$VAR` and `${VAR}`
- Exit status query: `$?`
- Tilde expansion: `~` and `~/path`
- Basic arithmetic expansion: `$((a + b))`

### Control Flow
- Compound statements: `if ... then ... else ... fi`
- Iteration loops: `while ... do ... done`, `for var in list; do ... done`
- Case matching: `case word in pattern) ... ;; esac`

### Built-in Commands
- `cd`, `pwd`, `echo`, `export`, `exit`, `alias`, `set`

### Signals
- SIGINT (Ctrl-C): Clears current line and prompt
- EOF (Ctrl-D): Exits interactive session when line buffer is empty

## Non-Supported Extensions (Explicitly Scoped Out)
- Process substitution `<(cmd)` (unless explicitly enabled)
- Complex array slicing syntax
- Extended globbing patterns `?()`, `*()`
