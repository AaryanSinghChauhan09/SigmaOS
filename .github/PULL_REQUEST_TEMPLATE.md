## Summary

<!-- What component does this PR improve or add? -->

## Related issue

Closes #<issue-id>

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

## IPC handler & IPC semantics

- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:
- Integration impact:

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

## Upstream study

- Project:
- Version/commit:
- Research note:
- ADR:

## What SigmaOS adopts

<!-- State the design principle or mechanism adopted. -->

## What SigmaOS changes or rejects

<!-- State simplifications, deviations, and rejected ideas. -->

## Reference behavior

- POSIX section:
- Bash/dash/Zsh/BusyBox reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Component impact

- Component:
- Maturity before:
- Maturity after:
- Runtime path affected:

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

## Safety and rollback

- [ ] No broad credentials or host access introduced
- [ ] Untrusted input is validated
- [ ] Package scripts are sandboxed, if applicable
- [ ] Rollback/recovery behavior defined
- [ ] Security review completed, if applicable
- [ ] Capability/namespace checks enforced

## Verification

```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```

## Runtime evidence

- [ ] QEMU boot log
- [ ] Installer/update test log
- [ ] Package dry-run report
- [ ] Hardware test log, if applicable
- [ ] Artifact hash

## Status updates

- [ ] Capability status
- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Shell documentation
- [ ] Boot matrix, if applicable

## Reviewer checklist

- [ ] Scope is narrow
- [ ] Tests cover the change
- [ ] Runtime evidence is linked
- [ ] Documentation matches implementation
- [ ] Capability status is evidence-based
- [ ] No unsupported parity claims
