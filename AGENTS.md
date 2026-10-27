# Agent Guidelines for SigmaOS

This document provides instructions and guidelines for AI and autonomous agents operating in the SigmaOS repository.

## Subsystem Priorities & Scope
- **Cadence-based Engineering**: Prioritize proof-of-behavior through incremental PRs rather than writing unvalidated planning documents.
- **Thread Manager & Scheduler**: Core kernel focus. Ensure `Thread` state transitions, wait channels, stack guard checks, lock ordering, and scheduler hooks follow `docs/thread-manager.md`.
- **Honest Status Tracking**: `FEATURE_STATUS.toml` is the source of truth for capability statuses (`modeled`, `unit-tested`, `integrated`, `hardware-tested`, `release-ready`).

## Mandatory Verification Protocol
Before submitting any code change, execute the required verification commands:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

For standalone module tests (when `cargo test --lib` times out on full workspace), run the specific subsystem test:
```bash
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```

## Repository Rules
- **No Unsafe Without Justification**: Explain safety invariants, aliasing, and pointer lifetimes in `// SAFETY:` comments.
- **Lock Ordering**: Document lock hierarchy before acquiring nested locks.
- **Fail Closed**: Security and capability checks must fail closed on ambiguous input or permissions.
- **Zero-Allocation Hot Paths**: Prefer zero-allocation byte scans / ASCII checks on search and hot paths.
