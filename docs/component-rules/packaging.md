# Component Rules: Packaging

## Component owner
packaging-agent

## Upstream references
- Arch Linux pacman / libalpm architecture
- Debian dpkg and APT clean-room builds
- Alpine APK package index format

## Purpose
Define required packaging integrity, dependency resolution, sandboxing, and validation
for SigmaOS package management and software distribution.

## Mandatory rules

1. Package metadata must be parsed before any installation action.
2. Dependency resolution must support dry-run mode.
3. Package scripts must run only in a sandbox with declared capabilities.
4. Updates must be atomic or recoverable; failed updates must restore the prior known-good state.
5. All package archives must enforce cryptographic signature and digest verification.
6. Package conflicts and file collisions must be detected prior to disk mutation.
7. Uninstallation must cleanly prune orphan dependencies without breaking system stability.

## Prohibited

- Unsandboxed execution of post-install maintenance scripts
- Overwriting existing system files without package replacement metadata
- Modifying system root state during dry-run commands
- Status promotion without metadata corpus validation and dry-run tests

## Required evidence

- Unit test report
- Integration test report
- Metadata corpus and dry-run tests
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
