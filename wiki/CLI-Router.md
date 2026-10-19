# SigmaOS CLI Router Components

## Overview
SigmaOS CLI command router providing single entry point with subcommands, competitive with Omarchy omarchy command router and Mint mintupdate CLI.

## Features
- **Single entry point**: `sigma` command
- **Subcommands**: sigma pkg, sigma hw, sigma theme, sigma menu, sigma update, sigma help
- **Metadata-driven help generation**
- **Interactive command palette support**
- **Command groups**: Pkg, Hw, Theme, Menu, Update, System, Network, User, Security, Help
- **Command pattern**: Each command has metadata and handler
- **Result types**: Success, Error, Help

## Command Groups

| Group | Description |
|-------|-------------|
| **pkg** | Package management helpers |
| **hw** | Hardware detection |
| **theme** | Theme management |
| **menu** | Sigma menu commands |
| **update** | System updates and migrations |
| **system** | System administration |
| **network** | Network configuration |
| **user** | User management |
| **security** | Security and access control |
| **help** | Help and information |

## Structures

| Structure | Description |
|-----------|-------------|
| **Router** | CLI router with built-in commands |
| **CommandMetadata** | Command metadata (name, group, summary, args, handler) |
| **CommandGroup** | Command category (Pkg, Hw, Theme, etc.) |
| **CommandResult** | Success, Error, Help |

## Usage Examples

```bash
# Package management
sigma pkg install vim
sigma pkg remove firefox
sigma pkg search kernel

# Hardware detection
sigma hw detect
sigma hw info

# Theme management
sigma theme list
sigma theme apply dark
sigma theme create my-theme

# Menu
sigma menu show
sigma menu search

# Updates
sigma update check
sigma update apply
sigma update rollback

# Help
sigma help
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Shell & UX (P1)

## Related Files
- `src/shell/cli_router.rs` - CLI router implementation
- `wiki/Shell-and-Userspace-Components.md` - Shell wiki

## Principles Followed
COMMAND_QUERY_SEPARATION, SINGLE_RESPONSIBILITY, MINIMISE_COUPLING,
MAXIMISE_COHESION, LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID,
DEMETER_LAW, DESIGN_PATTERN, COMMAND_PATTERN