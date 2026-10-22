## Summary

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
- [ ] Documentation
- [ ] CI / tooling

## Kernel component

- [ ] Kernel core / scheduler
- [ ] Memory management / paging
- [ ] Drivers / I/O subsystem
- [ ] Architecture / boot

## Shell component

- [ ] Lexer / parser / AST
- [ ] Execution engine / built-ins
- [ ] Redirection / pipelines
- [ ] Expansion / variables / quoting

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly I/O audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Quarterly architecture/security/release review

## Upstream study

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- Upstream study details:

## Reference behavior

<!-- Required for lifecycle, ABI, or I/O changes -->

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## What SigmaOS adopts

- Adopted abstractions and architecture patterns:

## Changes

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

## Kernel safety review

- [ ] Memory safety invariants held
- [ ] Lock ordering verified
- [ ] No race conditions

## Shell safety review

- [ ] Deterministic evaluation
- [ ] Quoting and redirection safe
- [ ] Resource limits respected

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

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] I/O documentation
- [ ] Boot matrix, if applicable

## Safety and rollback

- Risk:
- Safety precautions:
- Rollback:

## Risks and rollback

- Risk:
- Rollback:

## Reviewer checklist

- [ ] Code review completed
- [ ] Safety invariants verified
