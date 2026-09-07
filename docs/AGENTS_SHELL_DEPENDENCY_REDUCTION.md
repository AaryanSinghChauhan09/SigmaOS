# AI Agent Shell Dependency Reduction Specification for SigmaOS

This document specifies operational standards for AI agents reducing Shell programming language dependencies and migrating build/test scripts to pure Rust in **SigmaOS**.

---

## 1. Shell Script Reduction Guidelines

AI agents refactoring or replacing shell scripts (`.sh`, `.bash`) must follow these rules:

1. **Rust First Automation**:
   - Implement build, packaging, testing, and ISO staging logic in pure Rust (`src/`, `src/bin/`, `build.rs`).

2. **REPL Builtins**:
   - Integrate shell userland command builtins into `src/shell/repl.rs` and `src/shell/zsh_bash_parity.rs`.

3. **Safe Subprocess Invocation**:
   - Avoid `sh -c` string execution. Pass structured argument arrays via `Command::new()`.

4. **Path & Environment Safety**:
   - Use `PathBuf` for cross-platform filesystem path manipulation.

---

## 2. Verification Protocol

- Run `./run_sigma_tests.sh` and verify all Cargo build targets pass without shell script dependencies.

---

*Maintained by the SigmaOS Core Architecture Committee.*
