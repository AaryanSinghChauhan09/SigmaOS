# Contributing to SigmaOS

Thank you for contributing to SigmaOS! SigmaOS is built on a **cadence-based engineering program** designed to turn isolated components into verified, reviewable, and integrated pull requests.

---

## Cadence-Based Workflow

| Period | Subsystem Work | OS-Wide Maintenance | Required PR Output |
|---|---|---|---|
| Daily | Triage subsystem bugs, flaky tests, lock-order warnings, and regressions | Triage CI failures, build breakage, security advisories, and doc drift | Bug reports or small fix PRs |
| Weekly | Implement one scoped behavior (e.g., run queue, preemption, priority, wait queue) | Dependency/toolchain updates, lint cleanup, test-suite repair | One scoped PR per change |
| Biweekly | Cross-subsystem integration test (threads, signals, FDs, IPC, timers, exit) | Component status audit: update `FEATURE_STATUS.toml` and docs with evidence | Status + test-evidence PR |
| Monthly | Benchmark, contention test, starvation audit, deadlock/fuzz review | Release-candidate build, ISO smoke test, installer/update recovery test | Release-readiness PR |
| Quarterly | Architecture review against Linux/BSD patterns; remove dead code/plans | Roadmap consolidation, security review, API stability review, contributor survey | Architecture RFC + cleanup PRs |

---

## Component Rules

### Code Rules
- One logical subsystem change per PR.
- No new public API without documentation, tests, and a capability-status update.
- No `unsafe` Rust without a safety comment explaining invariants, aliasing, lifetime, and panic behavior.
- Every lock acquisition must have a documented lock order.
- No unbounded allocation, recursion, or loops in kernel context.
- Every blocking operation must have a timeout or explicit wakeup path.
- Every error must be handled or explicitly documented as fatal.
- No feature is "complete" until it is connected to the runtime path and verified.

### Testing Rules
- Unit tests for pure logic.
- Integration tests for cross-subsystem behavior.
- Stress tests for concurrency.
- Fuzz tests for parsers, syscall inputs, filesystem paths, and network packets.
- Boot smoke tests for kernel-facing changes.
- Regression test attached to every fixed bug.

### Mandatory Verification Command Sequence
Before submitting a PR, verify your changes with:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

### Documentation Rules
- Update `FEATURE_STATUS.toml` whenever implementation status changes.
- Distinguish `Modeled`, `Unit-tested`, `Integrated`, `Hardware-tested`, and `Release-ready`.
- Every subsystem page must list: purpose, public API, dependencies, limitations, tests, and known gaps.
- No roadmap claim may say "complete" without a linked PR and verification evidence.

### Security Rules
- Threat-model every new syscall, IPC channel, driver interface, and parser.
- Validate all user-supplied lengths, pointers, paths, and flags at the boundary.
- Apply least privilege by default.
- Fail closed on ambiguous permission or capability checks.
- Require a second reviewer for security-sensitive changes: scheduler, memory, drivers, crypto, IPC, and syscall dispatch.

---

## Pull Request Submission Guidelines

1. All pull requests must use `.github/PULL_REQUEST_TEMPLATE.md`.
2. Do not commit directly to `main`; all changes arrive via PRs.
3. Every PR must include unit tests or integration test evidence.
