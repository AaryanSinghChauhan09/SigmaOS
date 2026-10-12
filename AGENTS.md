# SigmaOS Agent Guidelines & Repository Governance Rules

Welcome to **SigmaOS**, a high-performance capability-based microkernel operating system written in Rust with strict `#![no_std]` core enforcement.

---

## Architectural Directives & Guidelines

1. **`no_std` Core Enforcement:**
   - All kernel modules, drivers, and core subsystems must compile under `#![no_std]`.
   - Direct imports of `std` are prohibited outside test targets (`#[cfg(test)]`).

2. **Capability-Based Security Model:**
   - Every syscall entrypoint must verify the caller's `CapabilityToken` before executing privileged kernel or resource operations.

3. **WDM-Style Driver Architecture:**
   - Drivers must follow the pattern: `IoManager`, `DriverObject`, `DeviceObject`, and `DeviceExtension`.

4. **Explicit Type Annotations:**
   - Public API functions, constants, and key collection types must feature explicit type annotations.

---

## Cadence-Based Engineering Program

SigmaOS utilizes a cadence-based engineering workflow to transform isolated components into verified, integrated, reviewable pull requests.

| Period | Thread Manager / Scheduler Work | OS-Wide Maintenance | Required PR Output |
|---|---|---|---|
| **Daily** | Triage scheduler bugs, flaky tests, lock-order warnings, and regression reports | Triage CI failures, build breakage, security advisories, and documentation drift | Bug reports or small fix PRs |
| **Weekly** | Implement one scheduler behavior: run queue, preemption, priority, affinity, or wait queue | Dependency/toolchain updates, lint cleanup, test-suite repair | One scoped PR per change |
| **Biweekly** | Integration test: threads with signals, FDs, IPC, timers, and process exit | Component status audit: update `FEATURE_STATUS.toml` and docs with evidence | Status + test-evidence PR |
| **Monthly** | Scheduler benchmark, contention test, starvation audit, deadlock/fuzz review | Release-candidate build, ISO smoke test, installer/update recovery test | Release-readiness PR |
| **Quarterly** | Scheduler architecture review against Linux/BSD patterns; remove dead code | Roadmap consolidation, security review, API stability review | Architecture RFC + cleanup PRs |

---

## Enforceable Component Rules

### Code Rules

- One logical subsystem change per PR.
- No new public API without documentation, tests, and a capability-status update in `FEATURE_STATUS.toml`.
- No `unsafe` Rust without a safety comment explaining invariants, aliasing, lifetime, and panic behavior.
- Every lock acquisition must have a documented lock order.
- No unbounded allocation, recursion, or loops in kernel context.
- Every blocking operation must have a timeout or explicit wakeup path.
- Every error must be handled or explicitly documented as fatal.
- No feature is "complete" until it is connected to the runtime path and verified with evidence.

### Testing Rules

- Unit tests for pure logic (`rustc --test`).
- Integration tests for cross-subsystem behavior.
- Stress tests for concurrency and lock contention.
- Fuzz tests for parsers, syscall inputs, filesystem paths, and network packets.
- Boot smoke tests for kernel-facing changes.
- Regression test attached to every fixed bug.

### Mandatory Verification Block for Every PR

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

### Documentation Rules

- Update `FEATURE_STATUS.toml` whenever implementation status changes.
- Explicitly distinguish status states: `Modeled`, `Unit-tested`, `Integrated`, `Hardware-tested`, and `Release-ready`.
- Every subsystem page must list: purpose, public API, dependencies, limitations, tests, and known gaps.
- No roadmap claim may state "complete" without a linked PR and verification evidence.

### Security Rules

- Threat-model every new syscall, IPC channel, driver interface, and parser.
- Validate all user-supplied lengths, pointers, paths, and flags at boundary entrypoints.
- Apply least privilege by default.
- Fail closed on ambiguous permission or capability checks.
- Require a second reviewer for security-sensitive changes: scheduler, memory, drivers, crypto, IPC, and syscall dispatch.

---

## Governance & Merge Policy

- **Labels:** `area:thread-manager`, `area:scheduler`, `area:kernel`, `area:security`, `kind:feature`, `kind:bug`, `kind:test`, `kind:docs`, `status:modeled`, `status:unit-tested`, `status:integrated`, `status:hardware-tested`, `needs-second-reviewer`.
- **Approvals:** One maintainer approval for normal changes; two approvals for scheduler, memory, syscall, driver, security, or boot-path changes.
- **CI Gate:** All test pipelines must pass green.
- **No Direct Commits:** All changes arrive through reviewable pull requests.
