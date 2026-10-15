# Contributing to SigmaOS

Thank you for your interest in contributing to **SigmaOS**! SigmaOS is a high-performance, capability-based microkernel operating system written in Rust with strict `#![no_std]` core enforcement.

---

## Core Architectural Directives

All contributions to SigmaOS must strictly adhere to the following architectural rules:

1. **`no_std` Core Enforcement**:
   - All kernel modules, drivers, and core subsystems must compile under `#![no_std]`.
   - Direct imports of `std` are prohibited outside test targets (`#[cfg(test)]`).

2. **Capability-Based Security Model**:
   - Every syscall entrypoint must verify the caller's `CapabilityToken` before executing privileged kernel or resource operations.
   - Capability tokens must be explicit, unforgeable, and checked at API boundaries.

3. **WDM-Style Driver Architecture**:
   - Drivers must follow the Windows Driver Model (WDM) pattern: `IoManager`, `DriverObject`, `DeviceObject`, and `DeviceExtension`.
   - Every driver submission must include a standalone lifecycle unit test verifying driver creation, device attachment, dispatch handling, and teardown.

4. **Paged vs NonPaged Memory Pools**:
   - Interrupt service routines (ISRs) and core kernel structures must use `NonPaged` memory pools.
   - User-pageable buffers and background data must utilize `Paged` memory pools with bounds-checked operations (`copy_nonoverlapping`).

5. **Explicit Type Annotations**:
   - Public API functions, constants, and key collection types must feature explicit type annotations.

6. **Standalone Unit Testing**:
   - Standalone module tests must be runnable via `rustc --test`.
   - Workspace build scripts like `./scripts/changed_files_rustc_tests.sh` execute standalone tests per modified file to ensure high performance and isolated verification.

---

## Engineering Cadence

SigmaOS applies a structured cadence to ensure isolated components transition smoothly into reviewable, integrated pull requests:

| Period | Thread manager / scheduler work | OS-wide maintenance | Required PR output |
|---|---|---|---|
| **Daily** | Triage scheduler bugs, flaky tests, lock-order warnings, and regression reports | Triage CI failures, build breakage, security advisories, and documentation drift | Bug reports or small fix PRs |
| **Weekly** | Implement one scheduler behavior: run queue, preemption, priority, affinity, or wait queue | Dependency/toolchain updates, lint cleanup, test-suite repair | One scoped PR per change |
| **Biweekly** | Integration test: threads with signals, FDs, IPC, timers, and process exit | Component status audit: update `FEATURE_STATUS.toml` and docs with evidence | Status + test-evidence PR |
| **Monthly** | Scheduler benchmark, contention test, starvation audit, deadlock/fuzz review | Release-candidate build, ISO smoke test, installer/update recovery test | Release-readiness PR |
| **Quarterly** | Scheduler architecture review against Linux/BSD patterns; remove dead code and duplicated plans | Roadmap consolidation, security review, API stability review, contributor survey | Architecture RFC + cleanup PRs |

---

## Component Rules

Enforceable repository rules for code, testing, documentation, and security:

### Code Rules

- One logical subsystem change per PR.
- No new public API without documentation, tests, and a capability-status update.
- No `unsafe` Rust without a safety comment explaining invariants, aliasing, lifetime, and panic behavior.
- Every lock acquisition must have a documented lock order.
- No unbounded allocation, recursion, or loops in kernel context.
- Every blocking operation must have a timeout or explicit wakeup path.
- Every error must be handled or explicitly documented as fatal.
- No feature is “complete” until it is connected to the runtime path and verified.

### Testing Rules

- Unit tests for pure logic.
- Integration tests for cross-subsystem behavior.
- Stress tests for concurrency.
- Fuzz tests for parsers, syscall inputs, filesystem paths, and network packets.
- Boot smoke tests for kernel-facing changes.
- Regression test attached to every fixed bug.

### Verification Block

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
- No roadmap claim may say “complete” without a linked PR and verification evidence.

### Security Rules

- Threat-model every new syscall, IPC channel, driver interface, and parser.
- Validate all user-supplied lengths, pointers, paths, and flags at the boundary.
- Apply least privilege by default.
- Fail closed on ambiguous permission or capability checks.
- Require a second reviewer for security-sensitive changes: scheduler, memory, drivers, crypto, IPC, and syscall dispatch.

---

## Governance & Merge Policy

Use labels:

- `area:thread-manager`
- `area:scheduler`
- `area:kernel`
- `area:security`
- `kind:feature`
- `kind:bug`
- `kind:test`
- `kind:docs`
- `status:modeled`
- `status:unit-tested`
- `status:integrated`
- `status:hardware-tested`
- `needs-second-reviewer`

Merge policy:

- One maintainer approval for normal changes.
- Two approvals for scheduler, memory, syscall, driver, security, or boot-path changes.
- CI must be green.
- No direct commits to `main`; all changes arrive through PRs.
- Every merged PR must update tests, docs, or capability status when behavior changes.

---

## Submitting Pull Requests

1. **Branch Naming**: Use descriptive branch names like `feat/thread-manager-lifecycle`, `fix/capability-check-syscall`, or `docs/architecture-update`.
2. **PR Checklist**: Complete all items in `.github/PULL_REQUEST_TEMPLATE.md`.
3. **Commit Messages**: Write clear, concise commit messages following standard conventions:
   ```
   subsystem: brief summary of changes (50 chars max)

   Detailed explanation of the problem solved, design choices made,
   and test verification steps completed.
   ```
