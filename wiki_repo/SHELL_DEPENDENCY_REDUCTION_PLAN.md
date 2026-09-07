# Shell Language Dependency Reduction Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                        SigmaOS Master CLI (sigma_cli.rs)                        |
|        (sigma init, sigma build, sigma run, sigma attest, sigma publish)         |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                   WASM Hostcall Runtime & Native Rust Fast-Paths                |
|       (Dilithium-5 Attestation, ISO Generator, Package Shard Compiler)         |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
| Zsh/Bash Shell REPL   |   | Native Rust Harness   |   | SovereignPipe IPC     |
| (RedirectionEngine)   |   | (cargo test / rustc)  |   | (Zero-Copy Streams)   |
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                         SigmaOS Kernel & VFS Drivers                            |
|             (Pure Rust Kernel, Zero-Dependency klib, Multi-Arch HAL)            |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **Master Rust CLI (`src/tools/sigma_cli.rs`)**:
   - Replaces bash build/release/packager scripts with native, type-safe Rust commands (`sigma build`, `sigma run`, `sigma attest`, `sigma publish`).

2. **In-Kernel Shell REPL (`src/userland/shell.rs`)**:
   - Implements parameter expansion, alias mapping, wildcard globbing, and stream redirection in Rust without invoking `/bin/sh`.

3. **Native Rust Test Execution**:
   - Replaces external shell test wrappers with native Rust test harnesses (`cargo test`, standalone `rustc --test`).

4. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
