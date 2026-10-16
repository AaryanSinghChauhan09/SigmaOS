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
- [ ] CI / tooling

## Component

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

## Upstream study

<!-- Required for compatibility or grammar changes -->

- POSIX section:
- Bash/dash/Zsh/BusyBox reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Changes

- [ ] Source implementation
- [ ] Unit tests
- [ ] Integration tests
- [ ] QEMU/runtime test
- [ ] Documentation
- [ ] Capability/status update
- [ ] Regression protection
- [ ] Security review, if applicable

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
- [ ] Boot-to-shell evidence, if boot-affecting
- [ ] Security review requested, if applicable

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Shell documentation
- [ ] Boot matrix, if applicable

## Risks and rollback

- Risk:
- Rollback:
