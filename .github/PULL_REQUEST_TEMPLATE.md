# SigmaOS pull request

## Summary

<!-- What component does this PR improve, add, or regulate? -->

## Related issue

Closes #<issue-id>

## Component

<!-- kernel, memory, VFS, IPC, security, drivers, packaging, desktop, installer/update, etc. -->

## Component impact

- Component/repository:
- User-visible behavior:
- Related issue or plan:

## Upstream study

- Project:
- Version/commit:
- Research note:
- ADR:

## Upstream study and reference behavior

- Reference study:
- Upstream project, document, or behavior reviewed:
- What SigmaOS adopts:
- What remains intentionally different:

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

- [ ] Focused unit tests pass.
- [ ] ./run_sigma_tests.sh
- [ ] Documentation and wiki mirrors updated where applicable.
- Test/evidence commands:
- Results:

## Runtime evidence

- [ ] QEMU boot log
- [ ] Installer/update test log
- [ ] Package dry-run report
- [ ] Hardware test log, if applicable
- [ ] Artifact hash

## Capability status

- Current status:
- Evidence supporting the status:
- Claims intentionally left partial:

## Integration impact

- APIs, ABI, protocols, or file formats affected:
- Cross-repository synchronization required:
- Release or migration notes:

## Safety and rollback

- Risks and rollback:
- Recovery or downgrade procedure:
- [ ] No broad credentials or host access introduced
- [ ] Untrusted input validated
- [ ] Package scripts sandboxed, if applicable
- [ ] Rollback/recovery behavior defined
- [ ] Security review completed, if applicable

## IPC handler

- [ ] This change affects IPC handler behavior, or I confirm it does not.
- IPC semantics:
- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:

## Security review

- Kernel safety review:
- Security review completed:

## Shell safety review

- Shell safety review:
- Capability/namespace checks enforced:
- Shared-memory or descriptor implications:
- Input validation and failure behavior:
- Rollback or downgrade path:

## Reviewer checklist

- [ ] Scope is narrow
- [ ] Component rules are enforceable
- [ ] Tests cover the rules
- [ ] Runtime evidence is linked
- [ ] Documentation matches implementation
- [ ] Capability status is evidence-based
- [ ] No unsupported Linux/BSD parity claims
