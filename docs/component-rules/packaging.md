# Component Rules: Packaging

## Component owner
packaging-agent

## Upstream references
- Arch Linux pacman & PKGBUILD
- Debian dpkg & apt
- Nix package store

## Purpose
Define package metadata verification, dependency resolution, and sandboxed installation
for SigmaOS Packaging components.

## Mandatory rules

1. Package metadata must be parsed before any installation action.
2. Dependency resolution must support dry-run mode.
3. Package scripts must run only in a sandbox with declared capabilities.
4. Updates must be atomic or recoverable; failed updates must restore the prior known-good state.

## Prohibited

- Executing untrusted package scripts outside a sandbox
- Non-deterministic or unvalidated package dependency trees
- Overwriting existing package files without atomic transaction safety
- Status promotion without dry-run and install evidence

## Required evidence

- Unit test report
- Dependency resolution dry-run report
- Package script sandbox denial log
- Regression test for package corruption

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
