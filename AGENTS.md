# AGENTS.md — Agent Guidelines for SigmaOS

This document outlines instructions and rules for AI coding agents and human contributors working on the SigmaOS codebase.

## Scope & Precedence

- This file applies to the entire repository root directory tree.
- The initial problem description and explicit instructions take precedence over this document.

## Engineering Cadence & Process

Follow the project's engineering cadence for component improvements:

| Period | Thread Manager / Scheduler Work | OS-Wide Maintenance | Required PR Output |
|---|---|---|---|
| Daily | Triage scheduler bugs, lock-order warnings, flaky tests | Triage CI failures, build breakage, documentation drift | Bug reports or small fix PRs |
| Weekly | Implement one scheduler behavior (run queue, preemption, affinity) | Dependency/toolchain updates, test repairs | One scoped PR per change |
| Biweekly | Integration test across subsystem boundaries (threads + signals/IPC) | Component status audit in `FEATURE_STATUS.toml` | Status + test-evidence PR |
| Monthly | Scheduler benchmark, contention test, starvation audit | Release-candidate build, ISO smoke test | Release-readiness PR |
| Quarterly | Architecture review against Linux/BSD patterns | Roadmap consolidation, dead code removal | Architecture RFC + cleanup PRs |

## Enforceable Repository Rules

### Code Rules
1. One logical subsystem change per PR.
2. No new public API without documentation, tests, and a capability-status update.
3. No `unsafe` Rust without an explicit safety comment explaining invariants, aliasing, lifetime, and panic behavior.
4. Every lock acquisition must have a documented lock order.
5. No unbounded allocation, recursion, or loops in kernel context.
6. Every blocking operation must have a timeout or explicit wakeup path.
7. Every error must be handled or explicitly documented as fatal.
8. No feature is "complete" until connected to the runtime path and verified.

### Testing & Verification Rules

Every PR must run and pass the standard verification suite:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

For standalone module tests (e.g., `src/kernel/thread.rs`):

```bash
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```

### Documentation & Capability Rules
- Always update `FEATURE_STATUS.toml` when capability status changes.
- Use explicit status terms: `Modeled`, `Unit-tested`, `Integrated`, `Hardware-tested`, `Release-ready`.
- Maintain exact synchronization between code claims and verified test evidence.
