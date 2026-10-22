# Contributing to SigmaOS

Thank you for contributing to SigmaOS! SigmaOS is an experimental, sovereign Rust operating system designed for safety, performance, and deterministic behavior.

This document outlines the **cadence-based engineering model**, **component rules**, **testing standards**, and **governance policies** required for all contributions.

---

## Engineering Cadence

To ensure high quality and prevent unverified feature claims, development follows a cadence-based engineering program:

| Cadence | Thread Manager / Scheduler Focus | OS-Wide Maintenance Focus | Required PR Output |
|---|---|---|---|
| **Daily** | Triage scheduler bugs, flaky tests, lock-order warnings, regression reports | Triage CI failures, build breakage, security advisories, documentation drift | Bug reports or small bug fix PRs |
| **Weekly** | Implement one scoped behavior (run queue, preemption, priority, affinity, wait channel) | Dependency/toolchain updates, lint cleanup, test-suite repair | One scoped PR per feature/fix |
| **Biweekly** | Cross-subsystem integration tests (threads + signals, FDs, IPC, timers, process exit) | Component status audit: update `FEATURE_STATUS.toml` and docs with test evidence | Status + test-evidence PR |
| **Monthly** | Scheduler benchmarks, contention tests, starvation audits, deadlock/fuzz reviews | Release-candidate build, ISO smoke tests, installer/recovery tests | Release-readiness PR |
| **Quarterly** | Scheduler architecture review against Linux/BSD patterns; dead code removal | Roadmap consolidation, security reviews, API stability reviews, contributor survey | Architecture RFC + cleanup PRs |

---

## Component Rules

### Code Rules
- **Scoped PRs**: One logical subsystem change per PR.
- **Documentation & Status**: No new public API without inline documentation, tests, and a capability status update in `FEATURE_STATUS.toml`.
- **Unsafe Rust Invariants**: No `unsafe` Rust without a safety comment explicitly detailing invariants, aliasing, lifetime guarantees, and panic behavior.
- **Lock Hierarchy**: Every lock acquisition must have a documented lock order to prevent deadlocks.
- **Kernel Constraints**: No unbounded allocation, recursion, or loops in kernel context.
- **Timeout Protection**: Every blocking operation must have a timeout or explicit wakeup path.
- **Error Handling**: Every error must be handled or explicitly documented as fatal.
- **Runtime Verification**: No feature is marked "complete" until connected to the runtime path and verified with proof of behavior.

### Testing Rules
- **Unit Tests**: Mandatory for pure logic and data structure state machines.
- **Integration Tests**: Required for cross-subsystem interactions (e.g. threads + IPC + signals).
- **Stress & Concurrency Tests**: Required for synchronization primitives and concurrent queues.
- **Fuzzing**: Required for parsers, syscall boundary inputs, filesystem paths, and network packet handlers.
- **Boot Smoke Tests**: Mandatory for kernel-facing changes impacting boot, paging, IDT, or scheduling.
- **Regression Tests**: A regression test must accompany every bug fix.

### Required PR Verification Checklist

Before submitting a PR, every author must execute and attach output for:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

### Documentation Rules
- Update `FEATURE_STATUS.toml` whenever implementation status changes.
- Status Vocabulary: `Modeled`, `Unit-tested`, `Integrated`, `Hardware-tested`, `Release-ready`.
- Every subsystem document in `docs/` must list: Purpose, Public API, Dependencies, Limitations, Tests, and Known Gaps.
- No roadmap claim may state "complete" without a linked PR and verification evidence.

### Security Rules
- Threat-model every new syscall, IPC channel, driver interface, and input parser.
- Validate all user-supplied lengths, pointers, paths, and flags at system boundaries.
- Apply least privilege by default and fail closed on ambiguous capability or permission checks.
- Require a second reviewer for security-sensitive changes (scheduler, memory manager, drivers, crypto, IPC, syscall dispatch).

---

## Pull Request Guidelines & Governance

### Pull Request Labels
- **Areas**: `area:thread-manager`, `area:scheduler`, `area:kernel`, `area:security`, `area:vfs`, `area:ipc`, `area:drivers`, `area:installer`
- **Kinds**: `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`, `kind:refactor`
- **Statuses**: `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`
- **Reviews**: `needs-second-reviewer`

### Merge Policy
- **Standard Changes**: Requires 1 maintainer approval and green CI.
- **Core Kernel & Security**: Requires 2 maintainer approvals (scheduler, memory, syscall dispatch, drivers, crypto, IPC, security).
- **Direct Commits**: Prohibited on `main` branch; all changes arrive through verified PRs.
