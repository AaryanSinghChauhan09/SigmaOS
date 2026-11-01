## Summary

<!-- What changed and why? -->

## Task type

- [ ] Daily CI maintenance
- [ ] Weekly dependency/security review
- [ ] Weekly open-source study
- [ ] Biweekly component gap review
- [ ] Monthly boot validation
- [ ] Monthly installer/recovery rehearsal
- [ ] Monthly package/update rehearsal
- [ ] Quarterly release review
- [ ] Quarterly architecture review
- [ ] Quarterly hardware audit

## Component

- [ ] Boot / platform
- [ ] Kernel
- [ ] Memory
- [ ] Scheduler
- [ ] VFS / filesystems
- [ ] Syscalls
- [ ] Drivers
- [ ] Networking
- [ ] IPC
- [ ] Security
- [ ] Userspace
- [ ] Desktop
- [ ] Packaging / installer
- [ ] Documentation
- [ ] CI / tooling

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

<!-- Required for lifecycle, ABI, or storage changes -->

- Upstream study:
- Reference study:
- Upstream project, document, or behavior reviewed:
- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- What SigmaOS adopts:
- What remains intentionally different:
- Compatibility impact:

## Changes

-
-

## Evidence

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New or updated tests
- [ ] QEMU boot evidence, if applicable
- [ ] Installer/recovery evidence, if applicable
- [ ] Hardware evidence, if applicable
- [ ] Security review requested, if applicable

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Component documentation
- [ ] Roadmap or release checklist

## Risks and rollback

- Risk:
- Rollback:
- Recovery or downgrade procedure:

## IPC handler

- [ ] This change affects IPC handler behavior, or I confirm it does not.
- IPC semantics:
- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:

## Evidence

- Kernel safety review:
- Capability/namespace checks enforced:
- Shared-memory or descriptor implications:
- Input validation and failure behavior:
- Rollback or downgrade path:

## Shell safety review

- Shell safety review:

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
- [ ] New/updated shell tests
- [ ] POSIX/conformance test evidence
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-shell evidence, if boot-affecting
- [ ] Security review requested, if applicable
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
