# Component Rules: Packaging

## Component owner
packaging-agent

## Upstream references
- Arch Linux pacman / PKGBUILD
- Debian dpkg / APT
- Alpine apk

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS package management, dependency resolution, and installation.

## Mandatory rules

1. Package metadata must be parsed before any installation action.
2. Dependency resolution must support dry-run mode.
3. Package scripts must run only in a sandbox with declared capabilities.
4. Updates must be atomic or recoverable; failed updates must restore the prior known-good state.

## Prohibited

- Executing unsandboxed package maintainer scripts
- Installing packages without validating cryptographic signatures or hashes
- Mutating system state during dry-run dependency resolution
- Leaving broken packages or dangling dependencies without rollback

## Required evidence

- Metadata corpus parser unit tests
- Dry-run dependency resolution test reports
- Sandboxed script execution denial tests
- Atomic update and rollback verification reports
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests and metadata parsing suite |
| Integrated | Metadata corpus and dry-run tests with sandboxed runtime |
| Hardened | Fuzzing, negative dependency cycles, sandboxing review |
| Supported | Reproducible package install, update, and rollback evidence |
