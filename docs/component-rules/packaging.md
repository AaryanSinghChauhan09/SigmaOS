# Component Rules: Packaging

## Component owner
packaging-agent

## Upstream references
- Arch Linux pacman & PKGBUILD
- Debian dpkg / APT ecosystem
- Fedora RPM / DNF dependency management

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS Package Management components.

## Mandatory rules

1. Package metadata must be parsed before any installation action.
2. Dependency resolution must support dry-run mode.
3. Package scripts must run only in a sandbox with declared capabilities.
4. Updates must be atomic or recoverable; failed updates must restore the prior known-good state.

## Prohibited

- Executing package scripts outside an isolated sandbox
- Mutating system state before metadata verification completes
- Unresolved dependency loops or broken graph execution
- Unsupported package format claims without dry-run validation

## Required evidence

- Package metadata parser test suite report
- Dependency graph dry-run execution report
- Sandboxed script execution security log
- Atomic rollback verification test

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
