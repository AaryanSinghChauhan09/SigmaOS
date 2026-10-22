# SigmaOS Interactive Shell Architecture

## Prompt Rendering
The default prompt format string `\u@\h:\w\$ ` expands dynamically:
- `\u`: Current user login
- `\h`: Hostname
- `\w`: Current working directory (using `~` for home)
- `\$`: `#` for root, `$` for non-root users

## Line Editing and Auto-completion
- Supports Tab auto-completion for pathnames and executable commands.
- Multi-line quote detection prevents executing incomplete strings until closing quotes are matched.
- Signal handling ensures interactive prompt recovery on `SIGINT`.
