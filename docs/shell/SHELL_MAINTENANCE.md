# Shell Maintenance Guide & Architectural Guidelines

This document outlines maintenance procedures, code organization, and safety invariants for the SigmaOS shell component (`src/shell/`).

## Architecture & Subsystems
- **Command Interpreter (`src/shell/command.rs`, `src/shell/sovereign_command_interpreter.rs`)**:
  - Handles parsing, execution, redirection (`<`, `>`, `>>`), and pipeline management (`|`).
  - Implements zero-allocation string searching and bulk capacity pre-allocation.
- **Diagnostics (`src/shell/diagnostics/`)**:
  - Formats compiler-style error messages, line numbers, and fuzzy command suggestions.
- **Interactive Shell (`src/shell/interactive/`)**:
  - Controls prompt rendering, signal handling (`SIGINT`, `SIGTSTP`), multiline quotes, and completion candidate generation.

## Testing & Quality Assurance
- Execute shell unit tests via `rustc --test tests/shell_lifecycle_tests.rs --edition=2021 -o build/test_shell_lifecycle && ./build/test_shell_lifecycle`.
- Verify POSIX compliance subset rules and exit code policies before opening PRs.
