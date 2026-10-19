# SigmaOS Unified CLI Components

## Overview
SigmaOS unified CLI router providing a single `sigma` command entry point, competitive with Omarchy's `omarchy` command router.

## Features
- **Single entry point**: `sigma` command
- **Subcommands**: sigma pkg, sigma theme, sigma menu, sigma notify, sigma clipboard, sigma emoji, sigma ai, sigma keybindings, sigma update, sigma hw, sigma help
- **Command aliases**: shorthand aliases (p, t, m, n, c, e, a, kb, u, h)
- **Metadata-driven help generation** with emoji icons
- **Interactive command palette** support
- **Command groups**: Pkg, Hw, Theme, Menu, Update, System, Network, User, Security, Notify, Clipboard, Emoji, Ai, Keybindings, Help

## Command Groups

| Group | Emoji | Description |
|-------|-------|-------------|
| pkg | 📦 | Package management helpers |
| hw | 🔧 | Hardware detection |
| theme | 🎨 | Theme management |
| menu | 🔍 | Unified search & menu |
| update | 🔄 | System updates & rollbacks |
| system | ⚙️ | System administration |
| network | 🌐 | Network configuration |
| user | 👤 | User management |
| security | 🔒 | Security & access control |
| notify | 🔔 | Notifications & history |
| clipboard | 📋 | Clipboard manager |
| emoji | 😀 | Emoji picker |
| ai | 🤖 | AI agent configuration |
| keybindings | ⌨️ | Keybindings viewer |
| help | ❓ | Help & information |

## Structures

| Structure | Description |
|-----------|-------------|
| **SigmaRouter** | Unified CLI router with built-in commands |
| **CommandMetadata** | Command metadata (name, group, summary, args, aliases, handler) |
| **CommandGroup** | Command category with emoji and description |
| **CommandResult** | Success, Error, Help, Interactive |

## Usage Examples

```bash
# Package management
sigma pkg install vim
sigma pkg remove firefox
sigma pkg search kernel
sigma pkg update

# Theme management
sigma theme list
sigma theme apply dark
sigma theme create my-theme
sigma theme switch light

# Unified menu (SUPER+SPACE equivalent)
sigma menu show
sigma menu search terminal
sigma menu launch browser

# Notifications (with history and DND)
sigma notify list
sigma notify dismiss 3
sigma notify dnd on
sigma notify replay 10

# Clipboard manager (with image previews)
sigma clipboard list
sigma clipboard paste 2
sigma clipboard clear
sigma clipboard toggle

# Emoji picker
sigma emoji list
sigma emoji search smile
sigma emoji insert 😀

# AI agent (configurable default coding agent)
sigma ai set-agent claude-code
sigma ai status
sigma ai diagnose

# Keybindings viewer
sigma keybindings list
sigma keybindings search terminal
sigma keybindings export

# System updates
sigma update check
sigma update apply
sigma update rollback
sigma update status

# Hardware detection
sigma hw detect
sigma hw info
sigma hw sensors

# Help
sigma help
sigma pkg --help
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Shell & UX (P1)

## Related Files
- `src/shell/sigma_cli.rs` - CLI router implementation
- `wiki/Unified-CLI.md` - Wiki documentation

## Principles Followed
COMMAND_QUERY_SEPARATION, SINGLE_RESPONSIBILITY, MINIMISE_COUPLING, MAXIMISE_COHESION,
LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, SIMPLE_USER_INTERFACE, DISCOVERABILITY,
DESIGN_PATTERN, COMMAND_PATTERN, DEMETER_LAW, LEVEL_OF_ABSTRACTION, SIMPLE_API,
USER_FRIENDLY, COMMAND_QUERY_SEPARATION, COMPOSITION_OVER_INHERITANCE, MODULARITY