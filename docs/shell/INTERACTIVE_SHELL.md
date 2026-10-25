# SigmaOS Interactive Shell Specifications

## Overview

The interactive mode of `sigma-sh` provides a robust, responsive prompt environment backed by line editing, history, completion, signal traps, and multiline input.

## Features

### 1. Dynamic Prompt Rendering
- **`PS1` Formatting**: Expands `\u` (username), `\h` (hostname), `\w` (working directory), and `\$` (`#` for root, `$` for unprivileged user).
- **`PS2` Continuation**: Displays continuation prompt `> ` during multiline construct entry (unclosed quotes or control flow blocks).

### 2. Line Editing Navigation Keys
- `Ctrl-A`: Move cursor to beginning of line.
- `Ctrl-E`: Move cursor to end of line.
- `Ctrl-U`: Clear input line buffer.
- `Left` / `Right` Arrow keys: Move cursor position within buffer.
- `Up` / `Down` Arrow keys: Cycle through command history.

### 3. Command History
- Stores up to 1000 executed commands in circular line buffer.
- Ignores consecutive duplicate commands.

### 4. Interactive Signals
- **`Ctrl-C` (SIGINT)**: Clears current line and multiline buffer, outputs newline, and renders a fresh prompt without exiting the shell.
- **`Ctrl-D` (EOF)**: Returns EOF signal when buffer is empty to cleanly exit interactive session.

### 5. Multiline Input Detection
Automatically shifts to multiline mode when:
- Single quotes (`'`) or double quotes (`"`) remain unclosed at line termination.
- Control constructs (`if`, `for`, `while`, `case`, `{`) remain unclosed.

### 6. Tab Completion
- Basic tab completion for executable binary names in `$PATH` and file/directory paths.

### 7. Terminal Restoration
- Guarantees ANSI terminal raw mode is restored to normal cooked mode on clean exit, `Ctrl-D`, or fatal signal.
