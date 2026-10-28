# Contributing to SigmaOS

## Engineering Cadence & PR Guidelines

SigmaOS follows a cadence-based engineering model turning isolated components—especially process/thread management and scheduling—into verified, integrated, reviewable pull requests.

| Period | Thread Manager / Scheduler Work | OS-Wide Maintenance | Required PR Output |
|---|---|---|---|
| **Daily** | Triage scheduler bugs, flaky tests, lock-order warnings, and regression reports | Triage CI failures, build breakage, security advisories, and documentation drift | Bug reports or small fix PRs |
| **Weekly** | Implement one scheduler behavior: run queue, preemption, priority, affinity, or wait queue | Dependency/toolchain updates, lint cleanup, test-suite repair | One scoped PR per change |
| **Biweekly** | Integration test: threads with signals, FDs, IPC, timers, and process exit | Component status audit: update `FEATURE_STATUS.toml` and docs with evidence | Status + test-evidence PR |
| **Monthly** | Scheduler benchmark, contention test, starvation audit, deadlock/fuzz review | Release-candidate build, ISO smoke test, installer/update recovery test | Release-readiness PR |
| **Quarterly** | Scheduler architecture review against Linux/BSD patterns; remove dead code and duplicated plans | Roadmap consolidation, security review, API stability review, contributor survey | Architecture RFC + cleanup PRs |

---

## Repository Rules

### Code Rules
1. **One Logical Subsystem Change Per PR**: Keep PRs focused on single subsystem boundaries.
2. **Public API Requirements**: No new public API without documentation, tests, and a `FEATURE_STATUS.toml` update.
3. **`unsafe` Rust Standards**: No `unsafe` Rust without a safety comment explaining invariants, aliasing, lifetime, and panic behavior.
4. **Lock Ordering**: Every lock acquisition must have a documented lock order.
5. **Context Constraints**: No unbounded allocation, recursion, or infinite loops in kernel context.
6. **Blocking Operations**: Every blocking operation must have a timeout or explicit wakeup path.
7. **Error Handling**: Every error must be handled or explicitly documented as fatal.
8. **Definition of Done**: No feature is "complete" until it is connected to the runtime path and verified.

### Testing Rules
1. **Unit Tests**: Mandatory for pure logic.
2. **Integration Tests**: Required for cross-subsystem interactions.
3. **Stress & Concurrency Tests**: Required for multi-threaded/concurrent objects.
4. **Verification Block**: Every PR must pass:
   ```bash
   cargo fmt --check
   cargo clippy --all-targets -- -D warnings
   cargo check --lib
   cargo test --lib
   ./run_sigma_tests.sh
   ```

### Documentation & Capability Rules
1. **Status Updating**: Update `FEATURE_STATUS.toml` whenever implementation status changes.
2. **Explicit Status Levels**: Distinguish `Modeled`, `Unit-tested`, `Integrated`, `Hardware-tested`, and `Release-ready`.
3. **Subsystem Documentation**: Every subsystem page (`docs/`) must list: purpose, public API, dependencies, limitations, tests, and known gaps.
4. **No Unbacked Parity Claims**: Unit-tested subsystems are not proof of a bootable, installable OS. Do not claim "complete" without linked PR evidence.

---

## Governance & Labels

- `area:thread-manager`, `area:scheduler`, `area:kernel`, `area:security`, `area:process`
- `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`
- `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`
- `needs-second-reviewer`

### Review Policy
- One maintainer approval for normal changes.
- Two approvals for scheduler, memory, syscall, driver, security, or boot-path changes.
- CI must be green prior to merge.
