# SigmaOS Shell Maintenance & Subsystem Contract

## Overview
This document specifies maintenance rules, POSIX conformance policies, signal handling, expansion order, and safety guidelines for the SigmaOS shell (`src/userland/shell.rs`).

## Shell Component & Architecture
- **Interactive Engine**: Prompt rendering, history, line editing, multiline quote detection.
- **Diagnostics Engine**: Command not found suggestions, parse error formatting, redirection error formatting.
- **POSIX Conformance**: Quoting rules, field splitting, parameter expansion, exit status mapping.
