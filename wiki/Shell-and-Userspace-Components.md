# Shell and Userspace Components

## Overview
Shell and userspace components providing command-line interface, command execution, and userspace tools.

## Features
- Command line parsing and execution
- Built-in commands (cd, pwd, echo, exit, clear, help, history)
- Pipeline support
- Command history management
- Configuration management

## Structures
- **ShellCommand** - Command structure with args and redirections
- **ShellState** - Shell state including cwd, history, prompt
- **ShellBuiltin** - Enumeration of builtin commands
- **ParseResult** - Parse result enumeration

## Status
- **Implementation**: Planned
- **Dependencies**: Init supervisor, VFS, capability model
- **Roadmap Phase**: Init and shell (P1)

## Related Files
- `src/shell/parser.rs` - Command parsing logic
- `wiki/Shell-and-Userspace-Components.md` - Wiki documentation
- `docs/SHELL_RULES.toml` - Rules configuration
