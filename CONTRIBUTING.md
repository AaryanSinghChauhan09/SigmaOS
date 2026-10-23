# SigmaOS Contributing Guidelines & Cadence Program

Thank you for contributing to SigmaOS! To turn isolated subsystems into verified, reviewable, and integrated pull requests, all contributions follow a strict **cadence-based engineering program** and repository governance rules.

---

## Engineering Cadence

| Period | Thread manager / scheduler work | OS-wide maintenance | Required PR output |
|---|---|---|---|
| **Daily** | Triage scheduler bugs, flaky tests, lock-order warnings, and regression reports | Triage CI failures, build breakage, security advisories, and documentation drift | Bug reports or small fix PRs |
| **Weekly** | Implement one scheduler behavior: run queue, preemption, priority, affinity, or wait queue | Dependency/toolchain updates, lint cleanup, test-suite repair | One scoped PR per change |
| **Biweekly** | Integration test: threads with signals, FDs, IPC, timers, and process exit | Component status audit: update `FEATURE_STATUS.toml` and docs with evidence | Status + test-evidence PR |
| **Monthly** | Scheduler benchmark, contention test, starvation audit, deadlock/fuzz review | Release-candidate build, ISO smoke test, installer/update recovery test | Release-readiness PR |
| **Quarterly** | Scheduler architecture review against Linux/BSD patterns; remove dead code and duplicated plans | Roadmap consolidation, security review, API stability review, contributor survey | Architecture RFC + cleanup PRs |

---

## Enforceable Repository Rules

### Code Rules
- **One Logical Subsystem Change Per PR:** Do not mix unrelated architectural changes in a single PR.
- **No Un-documented Public API:** Every new public function, struct, or module must have documentation comments, unit tests, and a `FEATURE_STATUS.toml` status entry update.
- **Explicit `unsafe` Rationale:** Every `unsafe` block must include a `// SAFETY:` comment explaining memory safety invariants, aliasing constraints, stack guarantees, and panic behavior.
- **Documented Lock Ordering:** Every lock acquisition must document the lock order hierarchy to prevent deadlocks.
- **Resource Constraints in Kernel Context:** No unbounded allocations, dynamic recursion, or infinite loops in kernel space.
- **Explicit Wakeup & Timeout Paths:** Every blocking operation must specify a timeout or explicit wakeup path.
- **Failure Recovery:** Every error path must be handled or documented as fatal.
- **Proof of Behavior Required:** No feature is marked "complete" or "integrated" without connected execution runtime proof and test evidence.

### Testing Rules
- **Unit Tests:** Pure logic unit tests for state machines and algorithms.
- **Integration Tests:** Cross-subsystem interactions (e.g. threads + signals, IPC + process exit).
- **Stress & Concurrency Tests:** Concurrent lock contention, wait channel wakeups, and race conditions.
- **Fuzzing:** Input fuzzing for syscall boundaries, parsers, VFS paths, and IPC messages.
- **Boot Smoke Tests:** Boot path smoke tests for kernel-facing changes.
- **Regression Tests:** Every bug fix must include a regression test.

### Minimum Verification Block

Every PR must pass the following verification steps prior to submission:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

### Documentation Rules
- Update `FEATURE_STATUS.toml` whenever implementation status changes.
- Distinguish explicit capability statuses: `Modeled`, `Unit-tested`, `Integrated`, `Hardware-tested`, and `Release-ready`.
- Documentation pages must list: purpose, public API, dependencies, limitations, test evidence, and known gaps.
- No roadmap claim may state "complete" without a linked PR and verification evidence.

### Security Rules
- Threat-model every new syscall, IPC channel, driver interface, and boundary parser.
- Validate all user-supplied lengths, pointers, paths, and flags at subsystem boundaries.
- Apply least privilege by default; fail closed on ambiguous capability checks.
- Require a second reviewer for security-sensitive subsystems: scheduler, memory, drivers, crypto, IPC, and syscall dispatch.

---

## Governance & Merge Policy

### Labels
- Areas: `area:thread-manager`, `area:scheduler`, `area:kernel`, `area:memory`, `area:ipc`, `area:security`, `area:vfs`, `area:drivers`, `area:installer`, `area:ci`
- Kinds: `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`, `kind:security`
- Status: `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`
- Special: `needs-second-reviewer`

### Merge Requirements
- **Standard Changes:** Requires approval from at least 1 maintainer.
- **Security & Kernel Sensitive Changes:** Scheduler, memory manager, syscall dispatch, driver, crypto, IPC, and boot path changes require approval from at least 2 maintainers (`needs-second-reviewer`).
- **CI Gate:** All automated CI workflow checks must pass cleanly.
- **No Direct Commits:** All changes to `main` must arrive via verified pull requests.
