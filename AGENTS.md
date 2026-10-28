# AGENTS.md - Instructions for Autonomous Agents Working on SigmaOS

Welcome to SigmaOS. As an automated engineering agent, you are responsible for maintaining high code quality, deterministic safety, and auditable status tracking across the repository.

---

## Operating Directives for Agents

### 1. Scope & Plan Execution
- Keep PRs focused on a single subsystem boundary or task.
- Before making changes, formulate a plan and request plan review.
- Always verify changes after modifying files using read-only tools or test runs.

### 2. Mandatory Verification Suite
Every change must pass the standard verification suite. Always run:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

For subsystem-specific unit tests (e.g. Thread Manager):
```bash
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```

### 3. Documentation & Status Honesty
- Do not claim a subsystem or feature is "bootable" or "release-ready" if it is only unit-tested or modeled.
- Always update `FEATURE_STATUS.toml` when subsystem status or test evidence changes.
- Ensure documentation files in `docs/` or `wiki/` are force-added with `git add -f` if required by repository `.gitignore` patterns.

### 4. Safety & Coding Constraints
- Document all `unsafe` Rust code blocks with explicit safety invariant comments.
- Track lock ordering to prevent deadlocks.
- Ensure all allocation and loop bounds in kernel context are bounded.
