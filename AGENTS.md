# SigmaOS Agent Guidelines & Engineering Rules

This file provides rules and instructions for automated agents and engineers working in the SigmaOS repository.

---

## Core Directive: Proof over Claims

The SigmaOS README explicitly warns that unit-tested subsystems are not proof of a bootable, installable OS. Always prioritize **proof of behavior** over additional feature claims.

---

## Repository Rules

### Code & Safety
- **Single Subsystem Boundary**: Every PR must touch only one logical subsystem unless explicitly doing integration tests.
- **Safety Documentation**: Any `unsafe` block must include a `// SAFETY:` comment explaining memory safety invariants, non-null guarantees, pointer aliasing, and panic bounds.
- **Lock Ordering**: Document lock ordering before acquiring multiple locks to prevent deadlocks.
- **Error Propagation**: Do not swallow errors; pass them up via `Result` or log explicit panic reasons in early boot.

### Testing & Verification
- Always run the mandatory verification block for changes:
```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```
- For standalone module tests (e.g., `src/kernel/thread.rs`), execute:
```bash
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```

### Capability Tracking
- Update `FEATURE_STATUS.toml` whenever code status or test coverage changes. Use standard status tags:
  - `modeled`
  - `unit-tested`
  - `integrated`
  - `hardware-tested`
  - `release-ready`

### Documentation Policy
- Documentation files in `docs/` or repository root must be force-added to git staging using `git add -f <filepath>` because `.gitignore` excludes `*.md` by default.
