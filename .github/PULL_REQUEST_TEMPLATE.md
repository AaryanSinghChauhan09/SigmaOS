## Summary

<!-- What secondary-memory, shell, or subsystem behavior changed and why? -->

## Storage component

- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager
- [ ] Mount / unmount
- [ ] Cache
- [ ] Filesystem
- [ ] Recovery
- [ ] procfs / diagnostics
- [ ] Security / capabilities
- [ ] Documentation
- [ ] CI / tooling

## Shell component

- [ ] Lexer / Parser
- [ ] Expansion engine
- [ ] Builtin commands
- [ ] Redirection / Piping
- [ ] Job control / Signals
- [ ] Interactive REPL
- [ ] Diagnostics / Error formatting
- [ ] Conformance / POSIX

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly storage audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Quarterly architecture/security/release review

## Reference behavior

<!-- Required for lifecycle, ABI, or storage changes -->
<!-- Upstream study: Open-source OS references and Linux/BSD/Redox/xv6 study notes -->
<!-- What SigmaOS adopts: Subsystem architecture and component improvements -->

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Reviewer checklist

- [ ] Reviewer checklist completed

## Changes

-

## Storage safety review

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

## Shell safety review

<!-- Safety and rollback review checklist -->
<!-- Reviewer checklist: Verification, lock ordering, memory limits, and runtime evidence -->

- [ ] Deterministic script execution
- [ ] Explicit exit status codes
- [ ] No partial execution on parse error
- [ ] Quoting and expansion integrity verified
- [ ] Redirection and file descriptor limits enforced

## Shell safety review

- [ ] Deterministic script execution
- [ ] Explicit exit status on error
- [ ] Quoting and globbing expansion verified
- [ ] No partial execution on parse error
- [ ] Signal handling and redirection safe

## Reviewer checklist

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated unit tests
- [ ] Integration tests
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-shell evidence, if boot-affecting
- [ ] Storage benchmark, if performance-affecting
- [ ] Security review requested, if applicable

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Storage documentation
- [ ] Boot matrix, if applicable

## Safety and rollback

- Risk:
- Rollback:

## Reviewer checklist

- [ ] Verification complete
