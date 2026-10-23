# SigmaOS Autonomous Agent Instructions & Operating Rules

Welcome Agent! You are working on **SigmaOS**, an experimental Rust operating system encompassing kernel, userspace, desktop, packaging, and tooling subsystems.

---

## Key Directives & Principles

1. **Prioritize Proof of Behavior Over Feature Claims:**
   - Unit-tested data structures are NOT proof of a bootable, installable OS.
   - Never mark a feature "integrated" or "complete" without connected execution runtime path verification and test evidence.
2. **Obey Subsystem Boundaries & Scope:**
   - Keep PRs scoped to a single logical subsystem.
   - Do not make sweeping, unverified global refactors across multiple kernel subsystems in one step.
3. **Documentation Files Git Policy:**
   - `.gitignore` ignores `*.md` files by default in this workspace.
   - Whenever creating or editing documentation files (such as `docs/*.md`, `CONTRIBUTING.md`, `AGENTS.md`, or `.github/PULL_REQUEST_TEMPLATE.md`), you **must** force-stage them using `git add -f <filepath>`.

---

## Cadence & Task Execution Rules

Follow the engineering cadence program when selecting and implementing tasks:
- **Daily:** Bug fixes, lint cleanups, test repair, lock ordering audits.
- **Weekly:** Single subsystem feature addition (e.g. run queue, wait channel wakeup, priority inheritance).
- **Biweekly:** Cross-subsystem integration tests and capability status audits (`FEATURE_STATUS.toml`).
- **Monthly:** Concurrency/stress benchmarks, release-candidate builds, smoke tests.

---

## Mandatory Pre-Commit Verification Command Sequence

Before submitting any code or documentation changes, execute and verify:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

---

## Security & Code Rules

- **Safety Comments:** Every `unsafe` block must include a `// SAFETY:` rationale explaining pointer validity, bounds checks, aliasing invariants, and panic safety.
- **Lock Ordering:** Explicit lock ordering must be documented for every mutex/spinlock acquisition.
- **Error Handling:** All error cases must be handled gracefully without silent swallow or unhandled kernel panics.
- **Capability Updates:** Every change altering implementation status must update `FEATURE_STATUS.toml`.
