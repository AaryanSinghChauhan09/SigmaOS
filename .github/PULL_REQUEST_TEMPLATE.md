## Summary

<!-- One paragraph: what changed and why. -->

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

## Changes (What SigmaOS adopts)

-

## Motivation

<!-- Link the issue, roadmap item, or gap-analysis entry. -->

## Reference study (Upstream study)

<!-- Required for lifecycle, ABI, storage, or scheduling changes -->

## What SigmaOS adopts

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

- [ ] Lock ordering documented
- [ ] Interrupt-context safety considered
- [ ] Signal races considered
- [ ] Double-completion prevented
- [ ] Resource limits enforced
- [ ] No unbounded allocation or recursion

## Safety and rollback

- [ ] QEMU boot log
- [ ] Installer/update test log
- [ ] Package dry-run report
- [ ] Hardware test log, if applicable
- [ ] Artifact hash

## Safety and rollback

- [ ] No broad credentials or host access introduced
- [ ] Untrusted input validated
- [ ] Package scripts sandboxed, if applicable
- [ ] Rollback/recovery behavior defined
- [ ] Security review completed, if applicable

## Reviewer checklist

- [ ] Scope is narrow
- [ ] Component rules are enforceable
- [ ] Tests cover the rules
- [ ] Runtime evidence is linked
- [ ] Documentation matches implementation
- [ ] Capability status is evidence-based
- [ ] No unsupported Linux/BSD parity claims
