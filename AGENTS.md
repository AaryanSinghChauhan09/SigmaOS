# Agent Guidelines for SigmaOS

Welcome, Autonomous Agents! This document provides operational guidelines, coding standards, and verification rules when working on the SigmaOS codebase.

---

## Directives & Operating Principles

1. **Prioritize Proof of Behavior**: Pass unit tests, integration tests, and explicit verification blocks. Never claim a feature is complete based on module structure alone.
2. **Follow Component Rules**:
   - One logical subsystem change per PR.
   - Every `unsafe` block MUST have an explicit `// SAFETY:` rationale explaining memory safety invariants.
   - Lock acquisitions must follow documented lock ordering.
   - All blocking calls must support explicit wakeups or timeouts.
3. **Follow Documentation Policy**:
   - When creating documentation in `docs/*.md` or root `.md` files, force-add them to git staging using `git add -f <path>` because `**/*.md` is listed in `.gitignore`.
   - Always update `FEATURE_STATUS.toml` when changing subsystem implementation maturity or test evidence.
4. **Mandatory Verification**:
   - Run the standard verification suite prior to finalizing changes:
     ```bash
     cargo fmt --check
     cargo clippy --all-targets -- -D warnings
     cargo check --lib
     cargo test --lib
     ./run_sigma_tests.sh
     ```
   - For standalone subsystem tests (e.g., `src/kernel/thread.rs`), compile and execute standalone tests using:
     ```bash
     rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
     ```

---

## Subsystem Contacts & Contracts

- **Thread Manager**: `src/kernel/thread.rs` (Contract: `docs/thread-manager.md`, Status: `unit-tested`).
- **Scheduler**: `src/kernel/scheduler.rs`, `src/scheduler/` (Contract: `docs/SCHEDULER_STATUS_AND_INVARIANTS.md`).
- **Process Manager**: `src/kernel/process.rs` (Contract: `docs/PROCESS_MANAGER_PLAN.md`).
- **Interrupts & IRQ**: `src/kernel/interrupt.rs`, `src/kernel/irq/` (Contract: `docs/INTERRUPT_HANDLER_PLAN.md`).
- **IPC Handler**: `src/ipc/ipc.rs` (Contract: `docs/ipc-handler.md`).
- **Signals**: `src/signal/mod.rs` (Contract: `docs/SIGNAL_HANDLER_PLAN.md`).
