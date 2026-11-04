## Summary

## Summary

<!-- What behavior changed and why? -->

## Component impact

## Shell component

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
- [ ] CI / tooling

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

## Upstream study and reference behavior

- Reference study:
- Upstream project, document, or behavior reviewed:
- POSIX section:
- Bash/dash/Zsh/BusyBox reference:
- SigmaOS current behavior:
- What SigmaOS adopts:
- What remains intentionally different:
- Compatibility impact:

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
- Capability/namespace checks enforced:
- Shared-memory or descriptor implications:
- Input validation and failure behavior:
- Rollback or downgrade path:

## Verification

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] Focused unit tests pass.
- [ ] New/updated shell tests
- [ ] POSIX/conformance test evidence
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-shell evidence, if boot-affecting
- [ ] Security review requested, if applicable
- [ ] Documentation and wiki mirrors updated where applicable.
- Test/evidence commands:
- Results:

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

<!--
## I/O Subsystem Verification Checklist
- I/O component: Verification of registered I/O drivers and peripheral devices
- I/O policy: Strict conformance to I/O safety guidelines
- Interrupt manager: Top-half and bottom-half interrupt handling isolation
- DMA manager / DMA queue: Memory safety in scatter-gather DMA mappings
- Buffer manager: LRU cache eviction and lock safety
- I/O scheduler: Deadline and priority queue validation
- I/O safety review: Reentrancy and race condition audits
-->
