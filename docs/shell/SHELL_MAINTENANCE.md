# SigmaOS Shell Subsystem Maintenance and Conformance Policy

## Overview

SigmaOS Shell Maintenance and Conformance Policy enforces POSIX shell conformance, interactive terminal features, diagnostics, signal handling, and expansion ordering across the shell lifecycle.

## Key Policies

1. **Deterministic Execution**: Shell scripts must execute deterministically and reject invalid syntax fail-closed.
2. **Expansion Order**: Brace expansion -> Tilde expansion -> Parameter expansion -> Command substitution -> Arithmetic expansion -> Word splitting -> Pathname expansion -> Quote removal.
3. **Exit Codes**: Standard exit codes (0 = Success, 2 = Misuse / Parse error, 126 = Cannot execute, 127 = Command not found, 128+N = Signal N).
4. **Interactive Mode**: Signal safety (Ctrl+C interrupts foreground, Ctrl+D exits prompt), tab completion, and multi-line quote detection.
