# Contributing to SigmaOS

Thank you for contributing to SigmaOS! SigmaOS is an experimental Rust operating system with kernel, userspace, desktop, packaging, and tooling areas.

To maintain code quality, security, and predictability across the repository, all contributions must follow our cadence-based engineering program, component rules, testing requirements, and governance processes.

---

## Engineering Cadence

SigmaOS operates on a cadence-based engineering program designed to transform isolated components into verified, integrated, reviewable pull requests.

| Period | Thread Manager / Scheduler Work | OS-Wide Maintenance | Required PR Output |
|---|---|---|---|
| **Daily** | Triage scheduler bugs, flaky tests, lock-order warnings, and regression reports | Triage CI failures, build breakage, security advisories, and documentation drift | Bug reports or small fix PRs |
| **Weekly** | Implement one scheduler behavior: run queue, preemption, priority, affinity, or wait queue | Dependency/toolchain updates, lint cleanup, test-suite repair | One scoped PR per change |
| **Biweekly** | Integration test: threads with signals, FDs, IPC, timers, and process exit | Component status audit: update `FEATURE_STATUS.toml` and docs with evidence | Status + test-evidence PR |
| **Monthly** | Scheduler benchmark, contention test, starvation audit, deadlock/fuzz review | Release-candidate build, ISO smoke test, installer/update recovery test | Release-readiness PR |
| **Quarterly** | Scheduler architecture review against Linux/BSD patterns; remove dead code and duplicated plans | Roadmap consolidation, security review, API stability review, contributor survey | Architecture RFC + cleanup PRs |

---

## Enforceable Repository Rules

### Code Rules

1. **Scope**: One logical subsystem change per PR.
2. **API Documentation**: No new public API without documentation, tests, and a capability-status update.
3. **Unsafe Rust Safety Comments**: No `unsafe` Rust block without an explicit safety comment explaining memory invariants, aliasing, lifetime, and panic behavior.
4. **Lock Ordering**: Every lock acquisition must have a documented lock order to prevent deadlocks.
5. **Kernel Execution Bounds**: No unbounded allocation, recursion, or loops in kernel context.
6. **Timeout Bounds**: Every blocking operation must have a timeout or explicit wakeup path.
7. **Error Handling**: Every error must be handled or explicitly documented as fatal.
8. **Runtime Verification**: No feature is considered "complete" until it is connected to the runtime path and verified with tests.

### Testing Rules

- **Unit Tests**: Required for pure logic and data structures.
- **Integration Tests**: Required for cross-subsystem interaction (e.g. threads with signals, FDs, IPC).
- **Stress Tests**: Required for concurrent primitives, run queues, and wait channels.
- **Fuzz Tests**: Required for parsers, syscall inputs, filesystem paths, and network packets.
- **Boot Smoke Tests**: Required for kernel architecture and boot path changes.
- **Regression Tests**: A regression test must be attached to every bug fix.

Every pull request must execute and pass the standard verification suite:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

### Documentation Rules

1. **Status Inventory**: Update `FEATURE_STATUS.toml` whenever implementation status changes.
2. **Status Distinctions**: Explicitly distinguish implementation maturity: `Modeled`, `Unit-tested`, `Integrated`, `Hardware-tested`, and `Release-ready`.
3. **Subsystem Documentation Pages**: Every subsystem page (in `docs/`) must list: purpose, public API, dependencies, limitations, test evidence, and known gaps.
4. **No Unverified Roadmap Claims**: No roadmap claim may say "complete" without a linked PR and verification evidence.

### Security Rules

1. **Threat Modeling**: Threat-model every new syscall, IPC channel, driver interface, and parser.
2. **Boundary Validation**: Validate all user-supplied lengths, pointers, paths, and flags at system boundary entry points.
3. **Least Privilege**: Apply least privilege by default. Fail closed on ambiguous permission or capability checks.
4. **Second Reviewer Requirement**: Require a second reviewer for security-sensitive changes: scheduler, memory, drivers, crypto, IPC, and syscall dispatch.

---

## Governance & Pull Requests

### Labels

- `area:thread-manager`, `area:scheduler`, `area:kernel`, `area:security`, `area:vfs`, `area:ipc`, `area:drivers`, `area:installer`
- `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`
- `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`
- `needs-second-reviewer`

### Merge Policy

- **Standard Changes**: One maintainer approval required.
- **Core Subsystems**: Two maintainer approvals required for scheduler, memory management, syscall dispatch, driver interfaces, security policy, or boot path changes.
- **CI Requirements**: All CI checks must be green prior to merge.
- **No Direct Commits**: All changes must arrive via reviewable Pull Requests.
- **Traceability**: Every merged PR must update tests, documentation, or `FEATURE_STATUS.toml`.
