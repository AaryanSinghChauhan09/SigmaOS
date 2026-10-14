# SigmaOS Interactive Shell Baseline

## Features

1. **Prompt Rendering (`PS1` / `PS2`)**
   - Default `PS1`: `\u@\h:\w\$ ` (renders user, host, current dir, and `#` for root or `$` for user).
   - Continuation `PS2`: `> ` for incomplete multiline commands.

2. **Line Editing Shortcuts**
   - `Ctrl-A`: Jump cursor to beginning of line.
   - `Ctrl-E`: Jump cursor to end of line.
   - `Ctrl-U`: Clear line before cursor.
   - Printable ASCII characters insert at current cursor position.

3. **Command History**
   - Maximum history buffer size: default 1000 items.
   - Deduplication: Consecutive duplicate commands are suppressed.

4. **Signal Handling**
   - `Ctrl-C` (`SIGINT`): Clears current input buffer, resets prompt to new line, returns `SignalEvent::Interrupt`.
   - `Ctrl-D` (`EOF`): Returns `SignalEvent::Eof` when buffer is empty to terminate session.

5. **Tab Completion**
   - Basic prefix matching for commands in path and directory targets.

6. **Multiline Input**
   - Automatic detection of unclosed single quotes or double quotes on Enter. Switches prompt to `PS2` until quotes are closed.
