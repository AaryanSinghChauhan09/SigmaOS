# SigmaOS Shell Programming Language Dependency Reduction Plan

## 1. Executive Overview
SigmaOS aims to reduce dependency on bash/sh shell scripts by migrating build, test, ISO generation, packaging, and release automation into native, memory-safe, zero-dependency Rust modules (`src/tools/sigma_cli.rs`, `src/userland/shell.rs`, `src/sigpkg/`, and `src/boot/`).

## 2. Shell Dependency Assessment
An inventory of shell scripts in `scripts/` and root directory includes:
- `./run_sigma_tests.sh` (Master test runner)
- `./scripts/build_all.sh`, `./scripts/sigma_build.sh` (Build orchestration)
- `./scripts/gen_iso.sh`, `./scripts/build-iso.sh` (ISO generation)
- `./scripts/package_app.sh` (Application packaging)
- `./scripts/sign_release.sh` (Cryptographic artifact signing)
- `./scripts/sync_wiki.sh` (GitHub Wiki synchronization)

While bash scripts provided early bootstrap convenience, relying on external shell interpreters introduces portability risks across non-POSIX platforms (e.g. Windows, UEFI bare-metal environments) and un-typed shell script syntax errors.

## 3. Native Rust Substitution Roadmap

### Phase 1: Native Master CLI (`sigma_cli.rs`)
The master CLI (`src/tools/sigma_cli.rs`) replaces bash build scripts with native Rust subcommands:
- `sigma build`: Replaces `scripts/build_all.sh` and `scripts/sigma_build.sh`.
- `sigma run`: Replaces QEMU launch helper shell scripts.
- `sigma attest`: Replaces `scripts/sign_release.sh` by generating Dilithium-5 / Ed25519 signatures natively.
- `sigma publish`: Replaces `scripts/package_app.sh` and `.sigma-app` shard packagers.

### Phase 2: In-Kernel Shell REPL (`src/userland/shell.rs`)
- `RedirectionEngine` and builtin command parsers in `src/userland/shell.rs` implement Zsh/Bash/Fish shell builtins natively in Rust (`alias`, `unalias`, `export`, `unset`, `type`, `history`, `pushd`, `popd`, `dirs`, `${VAR:-default}`, `${VAR#prefix}`).
- Pipelines and stream redirections execute via zero-copy `SovereignPipe` IPC channels without invoking external `/bin/sh` or `/bin/bash` binaries.

### Phase 3: Native Rust Test Harnesses
- Integration tests migrate from shell wrappers to Rust test harnesses (`cargo test --lib`, `cargo test --tests`, and standalone `rustc --test` test runners).

---
*Maintained by the SigmaOS Zero-Dependency & Architecture Steering Committee.*
