# SigmaOS Interactive Shell Baseline Specification

## Primary Features

The interactive shell mode (`sigma-sh` in interactive TTY mode) provides the baseline shell interface for SigmaOS system interactions.

## Prompt Rendering (`PS1`)

- Default prompt format: `sigma@sigmaos:\w\$ `
- Format escapes supported:
  - `\u`: Current username (`sigma`)
  - `\h`: Hostname (`sigmaos`)
  - `\w`: Current working directory (`~` for `$HOME`)
  - `\$`: `#` for root UID 0, `$` for unprivileged users

## Line Editor & Input Handling

- Input buffer with cursor navigation and history retrieval.
- **Ctrl-C (`\x03`)**: Cancels current input buffer, prints `^C`, outputs a fresh prompt, and sets exit code to `130`.
- **Ctrl-D (`\x04`)**: Sends EOF signal. If line buffer is empty, gracefully exits shell with exit status `0`.
- **Tab Completion**: Performs candidate match lookups for built-in commands, executables in `$PATH`, and local directory paths.
- **Multiline Input**: Detects incomplete input (e.g. unclosed single or double quotes, open `if`/`while` blocks) and displays secondary prompt `> ` until completed.

## Terminal State Management

- Configures line discipline mode on startup.
- Automatically restores original terminal state on normal exit, `exit` command, or fatal crash handler trap.
