## Summary

<!-- One paragraph: what changed and why. -->

## Component

<!-- Choose one -->
- [ ] Boot / platform
- [ ] Kernel core
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

## Reference study

<!-- Required for design-changing PRs -->

- Study document:
- Reference projects:
- Decision: Adopt / Adapt / Reject
- Reason:

## Changes

-
-
-

## Evidence

<!-- Do not mark complete without evidence. -->

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated tests included
- [ ] QEMU boot evidence attached, if boot-related
- [ ] Installer/recovery evidence attached, if installer-related
- [ ] Hardware evidence attached, if driver-related
- [ ] Security review requested, if applicable

## Status impact

- [ ] `FEATURE_STATUS.toml` updated
- [ ] `CAPABILITY_MATRIX.toml` updated
- [ ] Component documentation updated
- [ ] No unsupported “complete” claim introduced

## Risks and limitations

-
-

## Rollback plan

<!-- How to revert or recover if this change regresses boot, install, update, or security. -->

## Reviewer checklist

- [ ] Scope is small and reviewable
- [ ] Tests cover success and failure paths
- [ ] Error handling is explicit
- [ ] Unsafe code is justified
- [ ] Documentation matches actual behavior
- [ ] Status claims are evidence-based
