# Component Rules: Packaging

## Component owner
packaging-agent

## Upstream references
- Arch Linux pacman & PKGBUILD
- Debian dpkg / apt
- Fedora RPM / dnf

## Purpose
Enforce package metadata validation, dependency resolution safety, sandboxed script execution, and atomic updates.

## Mandatory rules

1. Package metadata must be parsed and verified before any installation or system modification action.
2. Dependency resolution must support a dry-run mode that outputs planned changes without altering system files.
3. Package post-install or build scripts must run only in a sandbox with declared minimum capabilities.
4. Updates must be atomic or recoverable; failed updates must restore the prior known-good state.
5. All package signatures must be cryptographically validated prior to extraction.

## Prohibited

- Unsandboxed execution of package maintainer scripts
- Modifying system files on dependency solver failure
- Ignoring package signature verification failures
- Promoting packaging status without dry-run test evidence

## Required evidence

- Package parser unit test reports
- Dependency solver dry-run integration test reports
- Sandboxed script execution test output
- Package rollback / recovery test log

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and packaging specification |
| Prototype | Package metadata parser and dry-run solver unit tests |
| Integrated | Sandboxed installation and rollback test execution in QEMU |
| Hardened | Malformed package fuzzing and capability sandbox verification |
| Supported | Reproducible package builds and verified mirror integration |
