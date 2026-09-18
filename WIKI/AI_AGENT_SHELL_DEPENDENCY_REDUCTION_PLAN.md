# SigmaOS AI Agent Shell Dependency Reduction & Pure-Rust Automation Plan

## 1. Executive Summary & Strategic Objectives

SigmaOS aims for a self-sufficient, pure-Rust system architecture. Shell scripts (`.sh`, `.bash`) in `./scripts/`, `./tools/`, and helper runners were historically used for build automation, testing, ISO generation, and release packaging. To eliminate shell interpreter dependencies, reduce vulnerability surface areas, and ensure cross-platform reproducibility, the SigmaOS roadmap mandates the systematic reduction of shell scripts in favor of native Rust binaries, `cargo` subcommands, and `src/shell/repl.rs` embedded REPL builtins.

This document establishes the official AI agent guidelines, migration phases, and architectural standards for reducing Shell programming language dependencies in SigmaOS.

---

## 2. Shell Script Inventory & Replacement Architecture

An audit of the repository identifies four main categories of shell scripts slated for pure-Rust migration:

| Shell Script Category | Path Patterns | Primary Responsibilities | Target Pure-Rust Module / Binary |
| :--- | :--- | :--- | :--- |
| **Test Runners & CI Pipeline** | `run_sigma_tests.sh`, `tools/sigma_ci_pipeline.sh`, `FIX_TESTS.sh` | Orchestrates 13 test phases, pytest, rustc | `src/bin/sigpkg.rs`, Rust test harnesses |
| **Build & ISO Automation** | `scripts/build_all.sh`, `scripts/gen_iso.sh`, `scripts/build-iso.sh` | Bootloader ISO staging, initramfs packing | `src/boot/bootloader.rs`, `src/bin/sigpkg.rs` |
| **Release & Git Hooks** | `scripts/release.sh`, `scripts/sigma_git_sync.sh`, `scripts/setup_hooks.sh` | Commit verification, wiki sync, PR merges | `src/userland/shell.rs`, Rust automation tools |
| **Diagnostics & Benchmarks** | `scripts/benchmark-boot.sh`, `scripts/benchmark-memory.sh` | Boot time, PMC memory audit telemetry | `src/hal/multi_arch.rs`, `src/system/cron.rs` |

---

## 3. Phased Migration Strategy for AI Agents

AI agents refactoring or replacing shell scripts must follow a 4-phase transition strategy:

### Phase 1: Native Rust Embedded Builtins (`src/shell/`)
- Ensure all command builtins provided by external shell utilities (`alias`, `export`, `pushd`, `popd`, `dirs`, `${VAR:-default}`, glob matching) are implemented directly in `src/shell/zsh_bash_parity.rs` and `src/shell/repl.rs`.
- Guarantee zero dependency on host `/bin/sh` or `/bin/bash` binaries for userland REPL execution.

### Phase 2: Rust Cargo Subcommands & Tooling Binaries
- Replace build/packaging shell scripts with native Rust binaries under `src/bin/` or Cargo build scripts (`build.rs`).
- For ISO creation and bootloader staging, use `src/boot/bootloader.rs` automated installer logic directly.

### Phase 3: Pure-Rust Test Harnesses
- Transition multi-step test orchestration from `run_sigma_tests.sh` to native Rust test harnesses (`tests/stress_and_fuzz_tests.rs`, `tests/test_universal_adapter.rs`).
- Use `std::process::Command` in Rust test drivers to execute sub-processes securely without invoking shell string interpreters (`sh -c`).

### Phase 4: Full Shell Purge & Deprecation
- Safely archive or remove legacy `.sh` scripts under `scripts/` once Cargo and native Rust tools provide 100% functional equivalence.

---

## 4. Coding Conventions for Pure-Rust Automation

AI agents porting shell scripts to Rust must adhere to these principles:

1. **Structured Execution over String Concatenation**:
   - Never construct shell commands via string formatting (`format!("sh -c ...")`) to prevent shell injection vulnerabilities.
   - Pass explicit argument vectors (`Command::new("cargo").args(["test", "--lib"])`).
2. **Cross-Platform Path Safety**:
   - Use `std::path::PathBuf` and `Path` joining instead of slash-separated shell strings.
3. **Environment Isolation**:
   - Explicitly define process environment variables using `.env()` or `.envs()` on child process builders.

---

## 5. Verification & Testing Protocol

AI agents completing Shell script to Rust migrations must pass verification:

1. **Native Test Suite**: Execute `./run_sigma_tests.sh` (or its Rust runner replacement) to confirm all test phases pass cleanly.
2. **Build Verification**: Confirm that all Cargo targets compile cleanly without relying on host shell binaries.

---

*Approved by the SigmaOS System Architecture & Shell Reduction Steering Committee.*
