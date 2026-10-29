# Contributing to SigmaOS

Thank you for contributing to SigmaOS. To ensure code quality, deterministic safety, and auditable progress from component prototypes to a fully verified operating system, all contributions must adhere to the engineering cadence, component rules, and verification standards below.

---

## Engineering Cadence

SigmaOS follows a structured engineering cadence to convert broad roadmap goals into narrow, reviewable pull requests with honest capability status updates.

| Cadence Period | Subsystem Task Focus | OS-Wide Maintenance Task | Required PR Output |
|---|---|---|---|
| **Daily** | Triage subsystem bugs, race conditions, lock-order warnings, and test regressions. | Triage CI failures, build breakages, toolchain updates, and documentation drift. | Bug reports or small bug-fix PRs. |
| **Weekly** | Implement one focused subsystem behavior (e.g., run queue ordering, preemption, signal mask, wait channel wake). | Toolchain updates, lint/clippy cleanup, test-suite maintenance and repair. | One tightly-scoped PR per change. |
| **Biweekly** | Cross-subsystem integration testing (threads with signals, FDs, IPC, timers, exit coordination). | Component status audit: update `FEATURE_STATUS.toml` and documentation with verified test evidence. | Status update + integration test evidence PR. |
| **Monthly** | Subsystem benchmarks, lock contention analysis, starvation audit, deadlock and fuzz testing. | Release-candidate build, ISO smoke testing, installer and recovery path verification. | Release-readiness & bench audit PR. |
| **Quarterly** | Architecture review against Linux/BSD OS paradigms; remove dead code and duplicate plans. | Roadmap consolidation, security review, API stability audit, contributor survey. | Architecture RFC + cleanup PRs. |

---

## Repository Component Rules

### Code Rules

1. **Scope**: One logical subsystem change per pull request.
2. **Documentation**: No new public API without complete rustdoc comments, usage examples, and a capability status entry.
3. **Unsafe Rust**: No `unsafe` block or function without a formal safety comment explaining memory layout, invariants, aliasing guarantees, lifetime bounds, and panic safety.
4. **Lock Ordering**: Every lock acquisition must have a documented lock hierarchy order to prevent deadlocks.
5. **Bounded Execution**: No unbounded dynamic heap allocations, recursion, or infinite loops in kernel context.
6. **Timeouts**: Every blocking operation (wait queues, channels, syscalls) must support an explicit timeout or cancel/wakeup path.
7. **Error Handling**: Every error must be explicitly handled or documented as a fatal panic condition.
8. **Runtime Evidence**: No feature may be claimed as "Complete" or "Integrated" until connected to the runtime boot/execution path and verified by test evidence.

### Testing Rules

- **Unit Tests**: Pure logic, data structure transitions, and algorithm state machines must be covered by unit tests.
- **Integration Tests**: Cross-subsystem interactions (e.g., process exit reaping threads and closing FDs) require integration tests.
- **Stress & Concurrency Tests**: Multi-threaded, lock-contention, and wait-channel race conditions require stress tests.
- **Fuzzing**: Parsers, syscall dispatch, filesystem paths, and network packet handlers must have fuzz tests.
- **Boot Smoke Tests**: Kernel-facing changes must pass QEMU or bare-metal boot smoke verification.
- **Regression Tests**: Every bug fix PR must attach a regression test proving the bug cannot recur.

---

## Verification Standards

Before submitting any Pull Request, every contributor must execute and include the output of the standard verification suite:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

---

## Security & Governance Rules

- **Threat Modeling**: Perform threat modeling on all syscall boundaries, IPC channels, driver interfaces, and user input parsers.
- **Input Validation**: Enforce strict length, boundary, pointer, path, and flag checks at all user-kernel boundaries.
- **Least Privilege**: Default to least privilege across all processes, capabilities, and file permissions.
- **Fail Closed**: Security and capability checks must fail closed upon error or ambiguous input.
- **Review Requirement**: High-impact subsystems (scheduler, memory management, syscalls, security, IPC, drivers) require review from two maintainers.
