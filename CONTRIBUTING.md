# SigmaOS Contributing Guidelines & Engineering Program

Welcome to SigmaOS! SigmaOS is an experimental Rust operating system with kernel, userspace, desktop, packaging, and tooling areas.

To ensure high reliability, stability, and maintainability across all subsystems, SigmaOS employs a **cadence-based engineering program** and strict component rules.

---

## Engineering Cadence Program

All engineering contributions follow a predictable cadence to turn isolated experiments into verified, integrated, and reviewable Pull Requests.

| Period | Thread Manager / Scheduler Work | OS-Wide Maintenance | Required PR Output |
|---|---|---|---|
| **Daily** | Triage scheduler bugs, flaky tests, lock-order warnings, and regression reports | Triage CI failures, build breakage, security advisories, and documentation drift | Bug reports or small fix PRs |
| **Weekly** | Implement one scheduler behavior: run queue, preemption, priority, affinity, or wait queue | Dependency/toolchain updates, lint cleanup, test-suite repair | One scoped PR per change |
| **Biweekly** | Integration test: threads with signals, FDs, IPC, timers, and process exit | Component status audit: update `FEATURE_STATUS.toml` and docs with evidence | Status + test-evidence PR |
| **Monthly** | Scheduler benchmark, contention test, starvation audit, deadlock/fuzz review | Release-candidate build, ISO smoke test, installer/update recovery test | Release-readiness PR |
| **Quarterly** | Scheduler architecture review against Linux/BSD patterns; remove dead code and duplicated plans | Roadmap consolidation, security review, API stability review, contributor survey | Architecture RFC + cleanup PRs |

---

## Component Rules

These rules are strictly enforced for all code contributions:

### Code Rules
- **Single-Scope PRs**: One logical subsystem change per PR.
- **Documented APIs**: No new public API without documentation, tests, and a capability-status update.
- **Safety Comments**: No `unsafe` Rust without a safety comment explaining invariants, aliasing, lifetime, and panic behavior.
- **Lock Ordering**: Every lock acquisition must have a documented lock order to prevent deadlocks.
- **Bounded Resources**: No unbounded allocation, recursion, or loops in kernel context.
- **Explicit Wakeup/Timeout**: Every blocking operation must have a timeout or explicit wakeup path.
- **Error Handling**: Every error must be handled or explicitly documented as fatal.
- **Connected Runtime**: No feature is considered "complete" until it is connected to the runtime path and verified with tests.

### Testing Rules
- **Unit Tests**: Mandatory for pure logic.
- **Integration Tests**: Required for cross-subsystem interactions (e.g. threads with IPC or signals).
- **Stress & Concurrency Tests**: Required for lock-free or multi-threaded paths.
- **Fuzz Tests**: Mandatory for parsers, syscall inputs, filesystem paths, and network protocols.
- **Boot Smoke Tests**: Required for kernel boot path and memory layout changes.
- **Regression Tests**: Every bug fix must include a regression test.

### Documentation Rules
- **Feature Status Tracking**: Update `FEATURE_STATUS.toml` whenever implementation status changes.
- **Status Classification**: Distinguish `Modeled`, `Unit-tested`, `Integrated`, `Hardware-tested`, and `Release-ready`.
- **Subsystem Pages**: Every subsystem page must detail: purpose, public API, dependencies, limitations, tests, and known gaps.
- **Evidence-Based Claims**: No roadmap entry may say "complete" without a linked PR and verification test evidence.

### Security Rules
- **Threat Modeling**: Threat-model every new syscall, IPC channel, driver interface, and parser.
- **Input Validation**: Validate all user-supplied lengths, pointers, paths, and flags at the boundary.
- **Least Privilege**: Apply least privilege by default.
- **Fail Closed**: Fail closed on ambiguous permission or capability checks.
- **Dual Review**: Require a second reviewer for security-sensitive subsystems: scheduler, memory, drivers, crypto, IPC, and syscall dispatch.

---

## Required PR Verification Block

Before opening or merging a PR, you **must** execute and paste the results of the following verification commands:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

For kernel thread manager changes, also run:

```bash
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```

---

## Governance & PR Policy

### Labels Taxonomy
- Subsystems: `area:thread-manager`, `area:scheduler`, `area:kernel`, `area:security`, `area:vfs`, `area:ipc`, `area:drivers`, `area:packaging`
- Types: `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`
- Status: `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`
- Special: `needs-second-reviewer`

### Review and Merge Policy
- **Normal Changes**: Requires 1 maintainer approval and green CI.
- **Security-Sensitive Subsystems**: Requires 2 maintainer approvals (scheduler, memory, drivers, crypto, IPC, syscall dispatch).
- **Direct Commits**: Direct commits to `main` are strictly prohibited; all code must arrive via Pull Request.
