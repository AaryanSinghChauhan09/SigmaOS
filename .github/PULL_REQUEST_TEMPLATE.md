## Summary

<!-- What component does this PR improve or add? -->

## Related issue

Closes #<issue-id>

## Upstream study

- Project:
- Version/commit:
- Research note:
- ADR:

## What SigmaOS adopts

<!-- State the design principle or mechanism adopted. -->

## What SigmaOS changes or rejects

<!-- State simplifications, deviations, and rejected ideas. -->

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

## Safety and rollback

- [ ] No broad credentials or host access introduced
- [ ] Untrusted input is validated
- [ ] Package scripts are sandboxed, if applicable
- [ ] Rollback/recovery behavior defined
- [ ] Security review completed, if applicable

## Reviewer checklist

- [ ] Scope is narrow
- [ ] Tests cover the change
- [ ] Runtime evidence is linked
- [ ] Documentation matches implementation
- [ ] Capability status is evidence-based
- [ ] No unsupported parity claims
