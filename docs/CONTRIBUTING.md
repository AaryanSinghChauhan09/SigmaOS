# SigmaOS Contributing Guidelines & Cadence-Based Engineering Program

SigmaOS is an experimental Rust operating system with kernel, userspace, desktop,
packaging, and tooling components. Our primary engineering philosophy requires
**proof of behavior**, not aspirational feature claims. Code presence and unit
tests alone do not establish a bootable, installable OS.

All contributions follow a structured periodic engineering cadence and strict
repository rules designed to turn isolated components—especially the thread
manager and scheduler—into verified, reviewable pull requests.

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

- **Single subsystem scope**: Exactly one logical subsystem change per PR.
- **Documentation & tests for public API**: No new public API without full
  documentation, tests, and a capability-status update.
- **`unsafe` Rust justification**: No `unsafe` Rust without an explicit safety
  comment detailing invariants, aliasing, lifetime guarantees, and panic behavior.
- **Documented lock ordering**: Every lock acquisition must document the global
  lock hierarchy to prevent deadlocks.
- **Bounded execution**: No unbounded allocation, recursion, or infinite loops
  in kernel context.
- **Explicit wakeups**: Every blocking operation must have an explicit timeout or
  lost-wakeup-safe wake path.
- **Explicit error handling**: Every error must be explicitly handled or documented
  as fatal with justification.
- **Proof of behavior**: No feature is considered "complete" until it is connected
  to the runtime execution path and validated.

### Testing Rules

- **Unit tests**: Pure logic, data structures, state transitions, and arithmetic.
- **Integration tests**: Cross-subsystem interactions (e.g., threads with IPC,
  signals, timers, and file descriptors).
- **Stress tests**: Multi-threaded concurrency, contention, and race conditions.
- **Fuzz tests**: Parsers, system call arguments, filesystem paths, and network packets.
- **Boot smoke tests**: Mandatory for kernel, memory, dispatcher, interrupt, and
  scheduler changes.
- **Regression tests**: Mandatory test attached to every bug fix.

### Verification Gate

Every pull request must pass the standard verification block:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

### Documentation Rules

- Update `FEATURE_STATUS.toml` whenever implementation status changes.
- Distinguish explicit statuses: `modeled`, `unit-tested`, `integrated`,
  `hardware-tested`, and `release-ready`.
- Subsystem documentation must list: purpose, public API, dependencies, limitations,
  tests, and known gaps.
- No roadmap claim may state "complete" without a linked PR and verification evidence.

### Security Rules

- Threat-model every new system call, IPC channel, driver interface, and parser.
- Validate all user-supplied lengths, pointers, paths, and flags at the subsystem boundary.
- Apply least privilege by default across all capability checks.
- Fail closed on ambiguous permission or capability evaluations.
- Require a second reviewer (`needs-second-reviewer`) for security-sensitive subsystems:
  scheduler, memory, drivers, cryptography, IPC, and syscall dispatch.

## Repository Governance & Labels

### PR Labels
- Area: `area:thread-manager`, `area:scheduler`, `area:kernel`, `area:security`
- Kind: `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`
- Status: `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`
- Review: `needs-second-reviewer`

### Merge Policy
- One maintainer approval required for general changes.
- Two approvals mandatory for scheduler, memory, syscall, driver, security, or
  boot-path modifications.
- Continuous Integration must be completely green.
- No direct commits to `main`; all changes arrive through verified pull requests.
- Every merged PR must update tests, documentation, or `FEATURE_STATUS.toml`.
