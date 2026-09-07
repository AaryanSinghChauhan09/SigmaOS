# AI Agent Guidelines for Shell Language Dependency Reduction in SigmaOS

## 1. Overview
SigmaOS enforces a strict policy reducing external shell script dependencies in favor of native Rust tools (`src/tools/sigma_cli.rs`), zero-dependency shell REPL modules (`src/userland/shell.rs`), and native Rust test harnesses.

## 2. Guidelines for AI Agents

### 2.1 Native Rust Tool Execution
- **Prefer `sigma_cli`**: AI agents executing system tasks must invoke `sigma` subcommands (`sigma build`, `sigma run`, `sigma attest`, `sigma publish`) rather than invoking external `.sh` scripts.
- **WASM Hostcall Fast-Paths**: Administrative tasks run via WASM hostcalls inside `sigma_cli` with zero external shell process spawning overhead.

### 2.2 Shell REPL Builtin Redirection (`src/userland/shell.rs`)
- **Native Redirection**: Parameter expansions (`${VAR:-default}`), stream redirections (`> file`, `2>&1`), and pipeline chains operate via `RedirectionEngine` in Rust rather than passing commands to `/bin/sh -c`.

### 2.3 Testing Strategy
- **Rust Harnesses First**: Testing agents invoke `./run_sigma_tests.sh` or `cargo test --lib` directly. New test scenarios must be implemented as Rust tests (`#[test]`) or Python pytest fixtures rather than new bash scripts.

---
*Maintained by the SigmaOS Architecture Steering Committee.*
