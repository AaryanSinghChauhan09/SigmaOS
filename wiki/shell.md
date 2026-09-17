# SigmaOS Shell: sigma-sh

## Overview

`sigma-sh` (`src/shell/`) is the SigmaOS native shell. It is:

- Bash/Zsh/Fish compatible at the command level
- Built with zero external dependencies
- Security-hardened with OpenBSD pledge/unveil sandboxing
- Features Fish-style autosuggestions and syntax highlighting

## Features

| Feature | Status | Description |
|---------|--------|-------------|
| Command execution | ✅ | Fork/exec with PATH resolution |
| Pipes `|` | ✅ | Inter-command piping |
| Redirections `>`, `>>`, `<` | ✅ | File I/O redirections |
| Here-documents `<<` | ✅ | Inline input |
| Variable expansion | ✅ | `$VAR`, `${VAR:-default}` |
| Arithmetic `$(( ))` | ✅ | Integer arithmetic |
| Command substitution `$()` | ✅ | Capture command output |
| Job control | ✅ | `bg`, `fg`, `jobs`, `Ctrl+Z` |
| History | ✅ | Persistent history with search |
| Tab completion | ✅ | Command and path completion |
| Autosuggestions | ✅ | Fish-style inline suggestions |
| Syntax highlighting | ✅ | Live input highlighting |
| Aliases | ✅ | Command aliases |
| Functions | ✅ | Shell functions |
| Scripting | ✅ | Sh-compatible scripts |

## Built-in Commands

| Command | Description |
|---------|-------------|
| `cd` | Change directory |
| `pwd` | Print working directory |
| `echo` | Print text |
| `export` | Set environment variable |
| `unset` | Unset variable |
| `alias` | Create command alias |
| `source` / `.` | Execute script in current shell |
| `jobs` | List background jobs |
| `fg` | Bring job to foreground |
| `bg` | Send job to background |
| `kill` | Send signal to process |
| `wait` | Wait for background job |
| `exit` | Exit shell |
| `history` | Show command history |
| `help` | Show built-in help |
| `sigma` | SigmaOS-specific commands |

## Configuration

Shell configuration is in `~/.sigma/sigma-sh.toml` or `/etc/sigma/sigma-sh.toml`:

```toml
[shell]
history_size = 10000
history_file = "~/.sigma/history"
autosuggestions = true
syntax_highlighting = true
vi_mode = false

[prompt]
format = "{user}@{hostname}:{cwd} {git_branch}$ "
color_user = "green"
color_host = "blue"
color_cwd = "cyan"

[completion]
fuzzy = true
case_sensitive = false

[keybindings]
accept_suggestion = "ctrl+e"  # or right arrow
history_search = "ctrl+r"
clear_screen = "ctrl+l"
```

## Scripting

sigma-sh scripts use `.sh` or `.sigma` extension:

```bash
#!/usr/bin/env sigma-sh

# Variables
NAME="world"
echo "Hello, $NAME!"

# Arrays
PACKAGES=(vim git curl)
for pkg in "${PACKAGES[@]}"; do
    sigma-pkg install "$pkg"
done

# Functions
greet() {
    local name="$1"
    echo "Welcome to SigmaOS, $name!"
}
greet "Alice"

# Error handling
set -euo pipefail  # Exit on error, undefined vars, pipe failures

# Conditionals
if sigma-pkg query vim > /dev/null 2>&1; then
    echo "vim is installed"
else
    sigma-pkg install vim
fi
```

## Security: Pledge/Unveil

sigma-sh supports OpenBSD-style capability restrictions:

```bash
# Restrict a command to only read files under /tmp
sigma-sh --pledge "stdio rpath" --unveil "/tmp:r" -- my-command

# Run untrusted script with minimal privileges
sigma-sh --sandbox strict --unveil "/input:r" --unveil "/output:rw" -- ./process.sh
```

## Prompt Customization

```toml
[prompt]
# Available variables: {user}, {hostname}, {cwd}, {cwd_short},
# {git_branch}, {git_status}, {jobs}, {exit_code}, {time}
format = "┌[{user}@{hostname}] {cwd} {git_branch}\n└> "
```
