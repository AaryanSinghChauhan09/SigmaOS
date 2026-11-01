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

## Summary

<!-- What behavior changed and why? -->

## Component impact

- Component/repository:
- User-visible behavior:
- Related issue or plan:

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

## Security review

- Kernel safety review:
- Capability/namespace checks enforced:
- Shared-memory or descriptor implications:
- Input validation and failure behavior:
- Rollback or downgrade path:

## Shell safety review

- Shell safety review:

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

- Current status:
- Evidence supporting the status:
- Claims intentionally left partial:

## Integration impact

- APIs, ABI, protocols, or file formats affected:
- Cross-repository synchronization required:
- Release or migration notes:
