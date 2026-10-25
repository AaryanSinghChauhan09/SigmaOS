# SigmaOS Autonomous Engineering Agent Guidelines

Welcome, Agent! This file contains explicit instructions and operational rules for AI engineering agents working on the SigmaOS repository.

---

## Scope & Precedence

1. **Scope**: This file applies to all directories and sub-repositories in the SigmaOS codebase.
2. **Precedence**: Specific nested `AGENTS.md` instructions override this root file. The user's explicit turn prompt takes precedence over default rules.

---

## Core Directive: Proof of Behavior Over Feature Claims

SigmaOS is an experimental operating system. **Unit-tested code is NOT proof of a bootable, installable OS.**
When implementing changes:
- Focus on verified proof of behavior.
- Do not claim POSIX or hardware compatibility without test evidence.
- Connect isolated modules to the execution runtime rather than introducing detached experiments.

---

## Engineering Cadence & PR Policy

1. **Scoped Changes**: Keep PRs narrow and single-subsystem focused.
2. **Lock Order**: Always document lock acquisition order when introducing or modifying mutexes / spinlocks.
3. **Safety Comments**: Every `unsafe` block must include a `// SAFETY:` comment detailing invariants, lifetime constraints, and pointer non-nullness.
4. **Capability Tracking**: Whenever code status changes, update `FEATURE_STATUS.toml`. Use precise statuses: `modeled`, `unit-tested`, `integrated`, `hardware-tested`, or `release-ready`.

---

## Mandatory Verification Protocol

Before submitting or marking any task complete, you **must** execute and verify the following commands in the sandbox environment:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

For subsystem-specific standalone tests (e.g. `src/kernel/thread.rs`), execute:

```bash
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```

Always verify using read-only tools (`read_file`, `list_files`, `run_in_bash_session`) that edited files are correctly formatted and modified before proceeding.
