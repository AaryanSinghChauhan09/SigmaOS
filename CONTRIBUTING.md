# Contributing to SigmaOS

Thank you for contributing to SigmaOS! SigmaOS is an experimental Rust operating system. To ensure high quality, auditable, and reviewable contributions, we follow a strict cadence-based engineering program and component rules.

## Engineering Cadence

| Period | Thread manager / scheduler work | OS-wide maintenance | Required PR output |
|---|---|---|---|
| Daily | Triage scheduler bugs, flaky tests, lock-order warnings, and regression reports | Triage CI failures, build breakage, security advisories, and documentation drift | Bug reports or small fix PRs |
| Weekly | Implement one scheduler behavior: run queue, preemption, priority, affinity, or wait queue | Dependency/toolchain updates, lint cleanup, test-suite repair | One scoped PR per change |
| Biweekly | Integration test: threads with signals, FDs, IPC, timers, and process exit | Component status audit: update `FEATURE_STATUS.toml` and docs with evidence | Status + test-evidence PR |
| Monthly | Scheduler benchmark, contention test, starvation audit, deadlock/fuzz review | Release-candidate build, ISO smoke test, installer/update recovery test | Release-readiness PR |
| Quarterly | Scheduler architecture review against Linux/BSD patterns; remove dead code and duplicated plans | Roadmap consolidation, security review, API stability review, contributor survey | Architecture RFC + cleanup PRs |

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

Minimum verification block for every PR:

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

## Governance & Merge Policy

- Use labels: `area:thread-manager`, `area:scheduler`, `area:kernel`, `area:security`, `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`, `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`, `needs-second-reviewer`.
- Merge policy:
  - One maintainer approval for normal changes.
  - Two approvals for scheduler, memory, syscall, driver, security, or boot-path changes.
  - CI must be green.
  - No direct commits to `main`; all changes arrive through pull requests.
  - Every merged PR must update tests, docs, or capability status when behavior changes.
