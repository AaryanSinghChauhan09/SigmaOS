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

## Reference study & Upstream study

<!-- Required for lifecycle, ABI, or storage changes -->
<!-- Upstream study: Open-source OS references and Linux/BSD/Redox/xv6 study notes -->
<!-- What SigmaOS adopts: Subsystem architecture and component improvements -->

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- What SigmaOS adopts:
- Decision: Adopt / Adapt / Reject
- Reason:

## Changes

-

## Kernel safety review and Reviewer checklist

- [ ] Reviewer checklist completed
- [ ] No unnecessary `unsafe`
- [ ] Every `unsafe` block documents invariants
- [ ] Lock ordering documented
- [ ] Interrupt-context safety considered
- [ ] Signal races considered
- [ ] Double-completion prevented
- [ ] Resource limits enforced
- [ ] No unbounded allocation or recursion

## Evidence & Reviewer checklist

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

## Risks, Safety and rollback

- Risk:
- Rollback:

## Reviewer checklist

- [ ] Verification complete
