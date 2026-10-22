# Agent Guidelines for SigmaOS Repository

Welcome to the SigmaOS repository. This document provides instructions and guidelines for AI and human engineering agents working on the codebase.

---

## 1. Subsystem Architecture & Philosophy

SigmaOS is an experimental, sovereign Rust operating system workspace containing kernel, userspace, desktop environment, package management, and system tooling.

**Primary Principle**: Unit-tested code is **not** proof of a bootable, installable OS. Always prioritize **proof of behavior** over unverified capability claims.

---

## 2. Mandatory Verification Rules

Before marking any task or PR as complete, agents MUST run and confirm clean output for:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

For standalone unit-tested kernel subsystems (e.g. `src/kernel/thread.rs`), execute:

```bash
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```

---

## 3. Engineering Cadence & PR Discipline

- **Daily**: Triage bugs, lock-order warnings, and flaky test reports.
- **Weekly**: Implement single scoped behaviors (e.g. preemption, affinity, run queue).
- **Biweekly**: Cross-subsystem integration testing and `FEATURE_STATUS.toml` audits.
- **Monthly**: Benchmarks, contention audits, release candidates, ISO smoke tests.
- **Quarterly**: Architecture reviews against Linux/BSD patterns and dead code removal.

---

## 4. Component Rules

- **Unsafe Invariants**: Every `unsafe` block must include a `// SAFETY:` comment explaining memory safety, lifetime, aliasing, and non-panic invariants.
- **Lock Hierarchy**: Document lock acquisition order for all locks held simultaneously.
- **Kernel Bounds**: No unbounded recursion, allocations, or loops in kernel code.
- **Feature Status**: Always update `FEATURE_STATUS.toml` when adding or modifying component capabilities.
- **Documentation Policy**: Documentation files in `docs/*.md` or root `.md` files must be force-added to git staging using `git add -f <path>` due to `.gitignore` rules.

---

## 5. Security & Review Requirements

- Threat model new syscalls, IPC interfaces, and parsers.
- Apply least privilege and fail closed on invalid inputs.
- Mandatory two-reviewer approval for: scheduler, memory, drivers, security, crypto, IPC, and syscall dispatch paths.
