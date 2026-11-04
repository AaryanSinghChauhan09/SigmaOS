## Summary

## Summary

## I/O component

- [ ] I/O policy
- [ ] Interrupt manager
- [ ] DMA manager / DMA queue
- [ ] Buffer manager
- [ ] I/O scheduler
- [ ] I/O safety review

## Storage component

- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## Storage safety review

- [ ] Double-completion prevented
- [ ] Resource limits enforced

## File component

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

## I/O component

- [ ] Interrupt manager
- [ ] DMA manager / DMA queue
- [ ] Buffer manager
- [ ] I/O scheduler
- [ ] I/O policy
- [ ] I/O safety review

## Storage component
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## Storage safety review
- [ ] Double-completion prevented
- [ ] Resource limits enforced

## IPC handler

- [ ] This change affects IPC handler behavior, or I confirm it does not.
- IPC semantics:
- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:

## Security review

- Kernel safety review:
- Capability/namespace checks enforced:
- Shared-memory or descriptor implications:
- Input validation and failure behavior:
- Rollback or downgrade path:

## Verification

- [ ] Focused unit tests pass.
- [ ] ./run_sigma_tests.sh
- [ ] Documentation and wiki mirrors updated where applicable.
- Test/evidence commands:
- Results:

## Capability status

- Current status:
- Evidence supporting the status:
- Claims intentionally left partial:

## Integration impact

- APIs, ABI, protocols, or file formats affected:
- Cross-repository synchronization required:
- Release or migration notes:

## Shell component

- [ ] Interactive shell
- [ ] Builtins
- [ ] Parser
- [ ] Diagnostics

## Component impact

- Component/repository:
- User-visible behavior:
- Related issue or plan:

## Shell safety review

- [ ] Shell safety review complete
- [ ] Safety and rollback
