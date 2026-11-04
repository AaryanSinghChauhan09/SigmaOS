## Summary

<!-- What component does this PR improve, add, or regulate? -->

## Related issue

Closes #<issue-id>

## Component

<!-- kernel, memory, VFS, IPC, security, drivers, packaging, desktop, installer_update, etc. -->

- [ ] Kernel
- [ ] Memory component
- [ ] VFS / filesystem
- [ ] IPC handler
- [ ] Security
- [ ] Drivers
- [ ] Packaging
- [ ] Desktop
- [ ] Installer / update
- [ ] Thread manager
- [ ] Scheduler
- [ ] Process management
- [ ] I/O component
- [ ] Storage component

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

## Motivation

<!-- Detailed motivation behind the component rule or capability enhancement -->

## Implementation notes

<!-- Architecture decisions, safety invariants, and data structures -->

## Tests

<!-- Description of unit, integration, and fuzz testing added -->

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly memory audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Quarterly architecture/security/release review

## Reference behavior

<!-- Required for lifecycle, ABI, or file changes -->

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Subsystem safety reviews

### Kernel safety review
- [ ] Preserves boot contract
- [ ] `unsafe` justified
- [ ] Classified panic diagnostics

### IPC semantics and safety
- [ ] IPC handler
- [ ] Blocking behavior:
- [ ] Non-blocking behavior:
- [ ] EOF / closure behavior:
- [ ] Capability/namespace checks enforced

### I/O policy and safety
- [ ] I/O component
- [ ] I/O policy
- [ ] Interrupt manager
- [ ] DMA manager
- [ ] Buffer manager
- [ ] I/O scheduler
- [ ] I/O safety review

### Storage safety review
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager
- [ ] Double-completion prevented
- [ ] Resource limits enforced

### File safety review
- [ ] Lifecycle transitions are valid
- [ ] Zombies are eventually reaped
- [ ] Orphans are adopted
- [ ] PID reuse is safe
- [ ] Lock ordering is documented
- [ ] Interrupt-context safety considered
- [ ] Signal races considered
- [ ] Double-unlink prevented
- [ ] Resource limits enforced
- [ ] No unbounded allocation or recursion

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

## Capability status and integration impact

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Documentation updated
- [ ] Boot matrix, if applicable
- [ ] Integration impact documented

## Reviewer checklist

- [ ] Scope is narrow
- [ ] `unsafe` justified
- [ ] Component rules are enforceable
- [ ] Tests cover the rules
- [ ] Runtime evidence is linked
- [ ] Documentation matches implementation
- [ ] Capability status is evidence-based
- [ ] No unsupported Linux/BSD parity claims
