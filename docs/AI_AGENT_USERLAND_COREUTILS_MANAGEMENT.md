# AI Agent Userland Coreutils Management Guidelines

## 1. Overview & Architecture
This document details AI agent procedures for maintaining and extending native userland replacements, GNU Coreutils equivalents (`ls`, `cp`, `mv`, `rm`, `cat`, `grep`, `mkdir`), and process diagnostic utilities (`ps`, `top`, `free`, `uptime`) in SigmaOS (`src/tools/native_userland_replacements.rs`).

---

## 2. Operational Directives for AI Agents

### 2.1 Coreutils Parity Standard
- **Pure Memory-Safe Implementations**: All userland replacements must be written in 100% pure, memory-safe Rust with zero unsafe code blocks and zero C library dependencies.
- **ANSI Color & Terminal Support**: Utilities formatting output for stdout (`ls`, `ps`, `top`) must support standard ANSI terminal color palettes and column alignments.

### 2.2 Diagnostic & Monitoring Tools
- **Process Table Diagnostics**: `ps` and `top` implementations must query kernel process control blocks (`PCB`) and compute CPU/memory usage metrics accurately.
- **POSIX Exit Code Standard**: All core utilities must return standard POSIX exit codes (`0` for success, non-zero for error conditions).

---

## 3. Related Files
- `src/tools/native_userland_replacements.rs`
- `docs/LINUX_DISTRO_PARITY_CHECKLIST.md`
