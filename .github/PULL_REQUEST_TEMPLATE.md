## Summary

<!-- One paragraph: what changed and why. -->

## Component

- [ ] Thread manager
- [ ] Scheduler
- [ ] Process management
- [ ] Memory management
- [ ] VFS / filesystems
- [ ] Drivers
- [ ] IPC / IPC handler
- [ ] Security
- [ ] Installer / packaging
- [ ] Documentation
- [ ] CI / tooling

## Changes

-

## Motivation

<!-- Link the issue, roadmap item, or gap-analysis entry. -->

## Implementation notes

<!-- Design decisions, data structures, locking, error handling. -->

## Reference study / What SigmaOS adopts

<!-- Upstream patterns, BSD/Linux study references, adopted designs. -->

## Safety and rollback

<!-- Risks, safety invariants, rollback plan. -->

## Tests

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Concurrency/stress tests added/updated
- [ ] Fuzz tests added/updated
- [ ] Boot smoke test performed, if kernel-facing
- [ ] No regression in existing tests

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

Paste relevant output below.

## Capability status

- Previous status:
- New status:
- Evidence:

## Integration impact

- [ ] No runtime integration yet
- [ ] Connected to runtime path
- [ ] Boot path validated
- [ ] Installer/update path validated
- [ ] Hardware tested

## Reviewer checklist

- [ ] Scope is narrow
- [ ] Public API documented
- [ ] `unsafe` justified
- [ ] Lock ordering documented
- [ ] Error paths tested
- [ ] Feature status updated
- [ ] No unsupported compatibility claim

## Kernel safety review

- [ ] Lock ordering verified
- [ ] Interrupt and concurrency safety invariants verified
- [ ] Kernel stack guard checks verified
