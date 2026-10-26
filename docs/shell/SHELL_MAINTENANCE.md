# SigmaOS Shell Maintenance & Lifecycle Architecture

## Objectives
Maintain the SigmaOS command interpreter and interactive shell as a deterministic, POSIX-conforming, and safe userland interface.

## Subsystem Architecture
- **Lexer & Parser**: Converts raw command line strings into structured ASTs without shell injection vulnerabilities.
- **Expansion Order**: Strictly enforces brace expansion, tilde expansion, parameter expansion, command substitution, arithmetic expansion, word splitting, pathname expansion, and quote removal.
- **Interactive Shell**: Renders PS1 prompts, handles multiline quotes, and processes SIGINT/SIGQUIT without crashing or orphan state.
- **Diagnostics**: Standardized error messages for unclosed blocks, invalid redirections, and command suggestions.
