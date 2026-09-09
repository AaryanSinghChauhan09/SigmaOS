# AI Agent Shell & Scripting Engine Management Guidelines

## 1. Overview & Architecture
This document specifies AI agent protocols for developing, maintaining, and testing the `sigma_sh` interactive command line interpreter, POSIX shell scripting execution engine, I/O redirection pipelines, and job control in SigmaOS (`src/shell/`).

---

## 2. Operational Directives for AI Agents

### 2.1 Interactive Shell & REPL
- **Command Dispatching**: AI agents extending `sigma_sh` must support command autocompletion, persistent history, environment variable evaluation (`$PATH`, `$HOME`), and alias expansion.
- **Piping & Redirection Engine**: The shell pipeline processor must handle multi-stage pipes (`cmd1 | cmd2 | cmd3`) and file descriptor redirections (`>`, `>>`, `<`, `2>&1`) cleanly.

### 2.2 Script Execution & Job Control
- **POSIX Script Compliance**: `.sh` script execution must parse standard POSIX shell constructs (`if`/`then`/`else`, `for` loops, `while` loops, functions, and variable assignments).
- **Process Job Control**: Signal forwarding (`SIGINT`, `SIGTSTP`) and background execution (`&`) must be properly wired to child processes.

---

## 3. Related Files
- `src/shell/`
- `docs/AI_AGENT_CLI_OPERATION_MANAGEMENT.md`
- `docs/LINUX_DISTRO_PARITY_CHECKLIST.md`
