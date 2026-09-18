# 💻 AI Agent CLI Operation Management in SigmaOS

## Executive Summary
Command-Line Interface (CLI) tools in SigmaOS provide primary userland control, package management, service supervision, and system administration workflows. Autonomous AI Agents (**Bolt ⚡**, **Palette 🎨**, and **Sentinel 🛡️**) interacting with or extending SigmaOS command-line utilities must adhere to standardized CLI argument parsing, standardized POSIX exit codes, structured JSON output formats, interactive terminal prompt conventions, and zero-dependency input sanitization specifications.

---

## 1. Core CLI Dispatchers & Architecture

SigmaOS provides three primary system CLI entrypoints:

```
+-------------------------------------------------------------+
|             SigmaOS CLI Execution Architecture              |
+-------------------------------------------------------------+
                              │
     ┌────────────────────────┼────────────────────────┐
     ▼                        ▼                        ▼
 `sigma_sh`               `sigpkg`                `sigmctl`
 (Sovereign Shell)     (Universal PM Tool)   (Service Supervisor)
```

1. **`sigma_sh`** (`src/shell/sigma_sh.rs`): Sovereign interactive shell supporting command history, TAB autocompletion, environment variables, job control, and process pipelines (`|`, `>`, `<`).
2. **`sigpkg`** (`src/bin/sigpkg.rs`): Universal package management dispatcher handling cross-distro operations (`install`, `remove`, `search`, `update`, `audit`).
3. **`sigmctl`**: System service supervisor CLI controlling init units and Runit/systemd-free services.

---

## 2. Standardized POSIX Exit Codes

All CLI tools MUST return explicit, predictable exit status codes:

| Code | Constant | Meaning |
|------|----------|---------|
| **`0`** | `EXIT_SUCCESS` | Command completed successfully. |
| **`1`** | `EXIT_FAILURE` | General non-fatal error. |
| **`2`** | `EXIT_INVALID_USAGE` | Invalid command arguments or flags parsed. |
| **`126`**| `EXIT_CANNOT_EXEC` | Command found but not executable. |
| **`127`**| `EXIT_CMD_NOT_FOUND` | Command or executable symbol not found in `$PATH`. |
| **`130`**| `EXIT_TERMINATED_SIGINT` | Terminated via `Ctrl+C` (`SIGINT`). |

---

## 3. Machine-Readable & Human-Friendly Formatting

All administrative CLI utilities MUST support dual output modes:
- **Human Mode (Default)**: Formatted table layout with ANSI 256-color status highlights (Green = Success, Yellow = Warning, Red = Error).
- **Machine Mode (`--json` / `--porcelain`)**: Structured, unformatted JSON payload emitted to `stdout` for programmatic AI Agent parsing and CI/CD automation pipelines.

---

## 4. AI Agent Operational Guidelines

1. **Bolt ⚡ (Performance Optimization)**:
   - Ensure CLI argument parsers operate in single-pass $O(N)$ token evaluation loops without heap reallocations.
   - Use direct buffer writes to `stdout` / `stderr` rather than intermediate string allocations.

2. **Palette 🎨 (UX & Accessibility)**:
   - Provide helpful command usage suggestions on typo errors (e.g. `Did you mean 'install'?`).
   - Respect `NO_COLOR` and `TERM=dumb` environment flags to disable ANSI color codes when requested or when output is piped.

3. **Sentinel 🛡️ (Security & Validation)**:
   - Sanitize all CLI arguments before passing them to shell execution or subprocess invocation (`execve`) to prevent command injection vulnerabilities.
   - Restrict administrative CLI subcommands behind POSIX capability checks or `sudo` authorization validation.
