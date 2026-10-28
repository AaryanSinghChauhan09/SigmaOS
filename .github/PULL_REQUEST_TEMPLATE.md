## Summary

<!-- One paragraph: what component does this PR improve or add, and why? -->

## Component

- [ ] Thread manager
- [ ] Scheduler
- [ ] Process management
- [ ] Memory management
- [ ] VFS / filesystems
- [ ] Drivers
- [ ] IPC
- [ ] Security
- [ ] Installer / packaging
- [ ] Documentation
- [ ] CI / tooling

## Upstream study

- Reference study:
- What SigmaOS adopts:
- What SigmaOS changes or rejects:

## Changes

-

## Motivation

<!-- Link the issue, roadmap item, or gap-analysis entry. -->

## Implementation notes

<!-- Design decisions, data structures, locking, error handling. -->

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

Paste relevant verification output below.

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

## Safety and rollback

- [ ] Risks and rollback behavior defined
- [ ] No broad credentials or host access introduced
- [ ] Untrusted input is validated
- [ ] Package scripts are sandboxed, if applicable
- [ ] Security review completed, if applicable

## Reviewer checklist

- [ ] Scope is narrow
- [ ] Public API documented
- [ ] `unsafe` justified with safety comments
- [ ] Lock ordering documented
- [ ] Error paths tested
- [ ] Feature status updated in FEATURE_STATUS.toml
- [ ] No unsupported compatibility claims
