## Summary

<<<<<<< HEAD
<!-- What shell behavior changed and why? -->

## Shell component

- [ ] Lexer
- [ ] Parser
- [ ] Expansion
- [ ] Executor
- [ ] Pipelines
- [ ] Redirection
- [ ] Job control
- [ ] Signals
- [ ] Built-ins
- [ ] Interactive line editor
- [ ] Script runner
- [ ] Diagnostics
- [ ] Security / sandboxing
=======
<!-- What I/O-management behavior changed and why? -->

## I/O component

- [ ] I/O policy
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Interrupt manager
- [ ] DMA manager
- [ ] Buffer manager
- [ ] I/O scheduler
- [ ] Submit / complete
- [ ] Callback
- [ ] Interrupt
- [ ] DMA
- [ ] Buffer
- [ ] Recovery
- [ ] procfs / diagnostics
- [ ] Security / capabilities
>>>>>>> origin/main
- [ ] Documentation
- [ ] CI / tooling

## Recurring task

<<<<<<< HEAD
- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly feature/bug triage
- [ ] Weekly POSIX/sh study
- [ ] Biweekly parser/expansion audit
- [ ] Biweekly built-in review
- [ ] Monthly boot-to-shell validation
- [ ] Monthly compatibility suite
- [ ] Monthly fuzz triage
- [ ] Monthly security review
- [ ] Quarterly release/UX/performance review

## Reference behavior

<!-- Required for compatibility or grammar changes -->

- POSIX section:
- Bash/dash/Zsh/BusyBox reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Changes

-
-

## Shell safety review

- [ ] Expansion order is correct
- [ ] Quoting behavior is preserved
- [ ] Redirection failures are handled
- [ ] Exit codes are explicit
- [ ] Signal behavior is defined
- [ ] No unbounded recursion or allocation
- [ ] Interactive terminal state is restored
- [ ] Script mode does not execute invalid partial input

## Evidence

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated shell tests
- [ ] POSIX/conformance test evidence
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-shell evidence, if applicable
- [ ] Security review requested, if applicable

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Shell documentation
- [ ] Boot matrix, if applicable

## Risks and rollback

- Risk:
- Rollback:
=======
<<<<<<< HEAD
- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly I/O audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Quarterly architecture/security/release review

## Reference behavior

<!-- Required for lifecycle, ABI, or I/O changes -->

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Changes

-
-

## I/O safety review

- [ ] Lifecycle transitions are valid
- [ ] Zombies are eventually reaped
- [ ] Orphans are adopted
- [ ] PID reuse is safe
- [ ] Lock ordering is documented
- [ ] Interrupt-context safety considered
- [ ] Signal races considered
- [ ] Double-completion prevented
- [ ] Resource limits enforced
- [ ] No unbounded allocation or recursion

## Evidence

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated unit tests
- [ ] Integration tests
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-boot evidence, if boot-affecting
- [ ] I/O benchmark, if performance-affecting
- [ ] Security review requested, if applicable
=======
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

## Changes

- [ ] Source implementation
- [ ] Unit tests added / updated
- [ ] Integration tests added / updated
- [ ] Concurrency / stress tests added / updated
- [ ] Fuzz tests added / updated
- [ ] Boot smoke test performed, if kernel-facing
- [ ] Documentation updated
- [ ] `FEATURE_STATUS.toml` capability updated
>>>>>>> origin/main

## Status updates

<<<<<<< HEAD
- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] I/O documentation
- [ ] Boot matrix, if applicable

## Risks and rollback

- Risk:
- Rollback:
=======
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
>>>>>>> origin/main
>>>>>>> origin/main
