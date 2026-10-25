# SigmaOS Interactive Shell Baseline

## Features

### Prompt Rendering
- Custom `PS1` and `PS2` prompts.
- Supported prompt escapes:
  - `\u`: Current username
  - `\h`: Current hostname
  - `\w`: Current working directory
  - `\$`: `#` for root, `$` for unprivileged users

### Line Editing
- Standard keybindings:
  - `Ctrl-A`: Move cursor to start of line
  - `Ctrl-E`: Move cursor to end of line
  - `Ctrl-U`: Clear current line
  - `Backspace`: Delete character before cursor

### History
- Bounded history buffer (default limit: 1000 lines).
- Deduplicates consecutive identical commands.

### Signal Handling
- `Ctrl-C` (`SIGINT`): Cancels active input line, displays new prompt.
- `Ctrl-D` (`EOF`): Exits session if line buffer is empty.

### Tab Completion
- Basic tab completion for built-ins, internal aliases, executables in `PATH`, and filesystem paths.

### Multiline Input
- Automatically detects incomplete quotes (`'`, `"`) and multiline blocks, displaying secondary prompt `PS2` (`> `).
