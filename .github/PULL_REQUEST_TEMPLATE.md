## Summary

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
- [ ] Documentation
- [ ] Capability/status update
- [ ] Regression protection
- [ ] Security review, if applicable

## Motivation

<!-- Detailed motivation behind the component rule or capability enhancement -->

## Implementation notes

<!-- Architecture decisions, safety invariants, and data structures -->

## Tests

<!-- Description of unit, integration, and fuzz testing added -->

## Recurring task

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
- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:

## Subsystem safety reviews

### Kernel safety review
- [ ] Preserves boot contract
- [ ] `unsafe` justified
- [ ] Classified panic diagnostics

## Shell safety review

- [ ] Expansion order is correct
- [ ] Quoting behavior is preserved
- [ ] Redirection failures are handled
- [ ] Exit codes are explicit
- [ ] Signal behavior is defined
- [ ] No unbounded recursion or allocation
- [ ] Interactive terminal state is restored
- [ ] Script mode does not execute invalid partial input

## Verification

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated shell tests
- [ ] POSIX/conformance test evidence
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-shell evidence, if boot-affecting
- [ ] Security review requested, if applicable

## Runtime evidence

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

## Capability status

## Integration impact

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Shell documentation
- [ ] Boot matrix, if applicable
- [ ] Integration impact documented

## Reviewer checklist

- Risk:
- Rollback:

## Component
- [ ] Thread manager
- [ ] Scheduler
- [ ] Process management

## Motivation

## Implementation notes

## Tests

## Verification
- cargo check --lib
- cargo test --lib
- ./run_sigma_tests.sh

## Capability status

## Integration impact

## I/O component
- [ ] I/O policy
- [ ] Interrupt manager
- [ ] DMA manager
- [ ] Buffer manager
- [ ] I/O scheduler
- [ ] I/O safety review

## Reviewer checklist
- [ ] Scope is narrow
- [ ] `unsafe` justified
