## Summary

<!-- One paragraph: what changed and why. -->

## Component

- [ ] IPC handler
- [ ] Pipes / FIFOs
- [ ] UNIX-domain sockets
- [ ] Shared memory
- [ ] Message queues
- [ ] Event wakeups
- [ ] Signals
- [ ] Process/thread management
- [ ] Memory management
- [ ] VFS / filesystems
- [ ] Drivers / I/O
- [ ] Shell
- [ ] Security
- [ ] Documentation
- [ ] CI / tooling

## Changes

-

## Motivation

<!-- Link issue, roadmap item, or gap-analysis entry. -->

## Subsystem / IPC Semantics

- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:
- Error behavior:
- Permission model:
- Resource limits:
- Backpressure policy:

## Security Review

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

Paste relevant output below:

```text

```

## Capability Status

- Previous status:
- New status:
- Test evidence:

## Integration Impact

- [ ] No runtime integration yet (unit-tested only)
- [ ] Connected to runtime path
- [ ] Boot path validated
- [ ] Installer / update path validated
- [ ] Hardware tested

## Reviewer Checklist

- [ ] Scope is narrow (one logical subsystem per PR)
- [ ] Public API fully documented
- [ ] `unsafe` code justified with safety invariant comments
- [ ] Lock ordering documented and deadlock-safe
- [ ] Error paths and timeout bounds tested
- [ ] `FEATURE_STATUS.toml` feature status updated
- [ ] No unsupported Linux/BSD compatibility claims
