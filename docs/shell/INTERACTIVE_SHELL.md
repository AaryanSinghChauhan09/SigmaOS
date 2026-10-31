# SigmaOS Interactive Shell

Interactive mode owns prompt rendering, line editing, terminal-state restoration,
signal handling, history tracking, and job-control transitions. Every error path
must restore terminal state before returning to the prompt.

## Minimum Interactive Features

### 1. Prompt Rendering
- Configurable primary prompt (`PS1`) and continuation prompt (`PS2`).
- Dynamic escape sequence expansion:
  - `\u`: Current logged-in username
  - `\h`: Hostname
  - `\w`: Current working directory (`~` replacement for `$HOME`)
  - `\$`: Privilege indicator (`#` for UID 0 root, `$` for unprivileged users)

### 2. Line Editing & History
- Standard ASCII cursor movement and in-line editing (backspace, delete, cursor left/right).
- Scrollable command history buffer bounded by `max_history_size` (default 1000 lines).
- Deduplication of consecutive identical commands.

### 3. Signal Handling (Ctrl-C & Ctrl-D)
- **Ctrl-C (`\x03` / SIGINT)**: Cancels current line entry, clears multiline buffer, outputs a newline, and re-renders prompt. Does not exit the interactive shell.
- **Ctrl-D (`\x04` / EOF)**: When the command line buffer is empty, sends EOF signal and cleanly terminates the interactive session with exit status 0. If buffer is non-empty, treated as delete character.

### 4. Tab Completion
- In-memory completion candidates for built-in commands, external `$PATH` binaries, and filesystem paths.
- Common prefix autocompletion with candidate listing on ambiguous matches.

### 5. Multiline Input
- Detection of unclosed quote delimiters (`'`, `"`) or unclosed compound blocks (`if`, `while`, `for`, `{`).
- Displays `PS2` continuation prompt (default `> `) until syntax is complete.

### 6. Terminal State Restoration
- Guarantees restoring raw terminal mode to standard canonical cooked mode upon shell exit or abnormal termination.
