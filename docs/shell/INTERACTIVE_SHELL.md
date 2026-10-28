# SigmaOS Interactive Shell Specification

## Baseline Features

### 1. Prompt Rendering
- Default prompt string (`PS1`): `\u@\h:\w\$ ` (e.g. `sigma@sigmaos:~$ `).
- Supports prompt escape codes: `\u` (username), `\h` (hostname), `\w` (working directory), `\$` (`#` for root, `$` for unprivileged user).

### 2. Line Editing & Buffer Management
- In-memory line buffer supporting character insertion, deletion, backspace, and cursor positioning.
- Command history stack with up/down navigation.

### 3. Signal Handling
- **Ctrl-C (`SIGINT`)**: Cancels current input buffer, prints `^C`, re-renders prompt on a new line without exiting shell.
- **Ctrl-D (`EOF`)**: On empty line buffer, exits shell gracefully with exit code `0`. On non-empty line buffer, deletes character under cursor.

### 4. Tab Completion
- Basic tab completion for executable commands in `PATH`, built-in commands, and filesystem paths.

### 5. Multiline Input
- Renders continuation prompt (`PS2`: `> `) when input line contains unclosed single quotes, double quotes, or incomplete control block structures (`if`, `while`, `for`, `case`).

### 6. Terminal State Preservation & Restoration
- Restores original terminal attributes (raw/cooked mode, echo settings) on exit, interruption, or crash.
