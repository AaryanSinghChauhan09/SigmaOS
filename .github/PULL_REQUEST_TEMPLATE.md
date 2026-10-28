## Summary

<!-- What changed and why? -->

## Component

- [ ] IPC handler
- [ ] Pipes / FIFOs
- [ ] UNIX-domain sockets
- [ ] Shared memory
- [ ] Message queues
- [ ] Event wakeups
- [ ] Signals
- [ ] Process/thread management
- [ ] VFS
- [ ] Security
- [ ] Documentation
- [ ] CI / tooling

## Changes

-

## Motivation

<!-- Link issue, roadmap item, or gap-analysis entry. -->

## IPC semantics

- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:
- Error behavior:
- Permission model:
- Resource limits:
- Backpressure policy:

## Security review

- [ ] User input validated
- [ ] Descriptor lifecycle checked
- [ ] Capability/namespace checks enforced
- [ ] No unbounded kernel allocation
- [ ] `unsafe` justified with SAFETY comments
- [ ] Threat model updated

## Tests

- [ ] Unit tests
- [ ] Integration tests
- [ ] Concurrency/stress tests
- [ ] Timeout tests
- [ ] Fuzz tests
- [ ] Resource-exhaustion tests
- [ ] Regression test for fixed bug

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

Paste output below.

## Capability status

- Previous status:
- New status:
- Evidence:

## Integration impact

- [ ] Unit-tested only
- [ ] Connected to runtime path
- [ ] Boot path validated
- [ ] Installer/update path validated
- [ ] Hardware tested
