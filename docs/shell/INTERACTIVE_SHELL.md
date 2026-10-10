# Interactive Shell Baseline Specification

## Core Principles
The interactive shell provides a predictable, resilient interface for user interaction in terminal and console sessions.

## Features

### 1. Prompt Rendering
- Standard primary prompt (`PS1`, default `sigma-sh$ `) and secondary prompt (`PS2`, default `> `).
- Supports prompt escape sequences (`\u` user, `\h` host, `\w` working dir, `\$` UID symbol).

### 2. Line Editing & Cursor Navigation
- Emacs-style keybindings:
  - `Ctrl-A`: Move cursor to start of line.
  - `Ctrl-E`: Move cursor to end of line.
  - `Ctrl-B` / Left: Move cursor left.
  - `Ctrl-F` / Right: Move cursor right.
  - `Ctrl-K`: Kill line from cursor to end.
  - `Ctrl-U`: Clear entire line.
  - `Ctrl-L`: Clear screen and redraw line.

### 3. Command History
- In-memory history ring buffer with configurable capacity (`HISTSIZE`, default 1000).
- Up/Down arrow key history traversal.
- `HistoryExpansionEngine` (`!n`, `!!`, `!prefix`).

### 4. Signal Handling
- `Ctrl-C` (`SIGINT`): Cancels current input line or terminates running foreground job, printing new line and prompt without exiting shell.
- `Ctrl-D` (`EOF`): On empty line, terminates interactive shell session with exit status of last command.

### 5. Completion & Multiline Input
- Tab key completion for executables in `$PATH` and local file paths.
- Auto-detects open quotes or incomplete construct and enters multiline editing mode with `PS2` prompt.

### 6. Terminal State Restoration
- Restores original terminal termios modes upon shell exit or abnormal signal handling.
