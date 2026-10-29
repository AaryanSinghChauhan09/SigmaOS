## Summary

<!-- What component does this PR improve, add, or regulate? -->

## Related issue

Closes #<issue-id>

## Component

<!-- kernel, memory, VFS, IPC, security, drivers, packaging, desktop, etc. -->

## Upstream study

- Project:
- Version/commit:
- Research note:
- ADR:

## Component rules added or enforced

- [ ] New rule
- [ ] Existing rule enforced
- [ ] Rule test added
- [ ] CI gate added
- [ ] Documentation updated

## What SigmaOS adopts

<!-- State the upstream principle or mechanism adopted. -->

## What SigmaOS changes or rejects

<!-- State simplifications, deviations, and rejected ideas. -->

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
- [ ] Untrusted input validated
- [ ] Package scripts sandboxed, if applicable
- [ ] Rollback/recovery behavior defined
- [ ] Security review completed, if applicable

## Reviewer checklist

- [ ] Scope is narrow
- [ ] Component rules are enforceable
- [ ] Tests cover the rules
- [ ] Runtime evidence is linked
- [ ] Documentation matches implementation
- [ ] Capability status is evidence-based
- [ ] No unsupported Linux/BSD parity claims
