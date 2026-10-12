# SigmaOS Interactive Shell & Line Editor Specification

## Features

### 1. Prompt Rendering
- Standard PS1 variable expansion (`\u` for username, `\h` for hostname, `\w` for working directory, `\$` for user/root indicator).
- Dynamic prompt updates on working directory change or environment modification.

### 2. Line Editing & Cursor Control
- Emacs-style line editing keybindings:
  - `Ctrl-A`: Move cursor to start of line.
  - `Ctrl-E`: Move cursor to end of line.
  - `Ctrl-K`: Kill line from cursor to end.
  - `Ctrl-U`: Clear entire line.
  - `Ctrl-L`: Clear screen while preserving current line buffer.

### 3. Command History
- In-memory ring buffer with configurable maximum entries (`HISTSIZE`, default 1000).
- Up (`UpArrow` / `Ctrl-P`) and Down (`DownArrow` / `Ctrl-N`) history traversal.
- Persistence to `~/.sigma_history` on clean shell termination.

### 4. Signal Handling
- `SIGINT` (`Ctrl-C`): Cancels current input line or interrupting running foreground child process without exiting the shell.
- `SIGEOF` (`Ctrl-D`): On empty line buffer, closes stdin and triggers clean shell termination with status 0.

### 5. Tab Completion
- Context-sensitive tab completion for:
  - Command names in `$PATH` and built-ins.
  - VFS path names and directories.
  - Shell variable names (`$`).

### 6. Terminal State Preservation
- Restores original terminal raw/canonical modes and signal masks upon shell exit or panic recovery.
