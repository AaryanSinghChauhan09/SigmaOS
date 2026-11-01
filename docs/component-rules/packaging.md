# Component Rules: Packaging

## Component owner
packaging-agent

## Upstream references
- Arch Linux pacman / PKGBUILD
- Debian dpkg / APT
- Alpine apk

## Purpose
Define required metadata structures, sandboxed execution, dependency resolution,
and validation for the SigmaOS package management subsystem.

## Mandatory rules

1. Package metadata must be parsed before any installation action.
2. Dependency resolution must support dry-run mode.
3. Package scripts must run only in a sandbox with declared capabilities.
4. Updates must be atomic or recoverable; failed updates must restore the prior known-good state.

## Prohibited

- Modifying filesystem state prior to successful dependency resolution
- Running maintainer scripts as unrestricted root
- Non-reproducible package archive creation
- Failing during installation without reverting back to consistent state

## Required evidence

- Metadata parser fuzzing and unit test suite
- Dependency solver graph resolution and dry-run execution reports
- Package script sandbox denial verification logs
- Rollback tests confirming filesystem integrity after simulated failure

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Package format and repository metadata ADR |
| Prototype | Metadata parser and dependency graph unit tests |
| Integrated | Dry-run package installation and extraction test |
| Hardened | Sandboxed install execution and atomic rollback verification |
| Supported | Multi-package ecosystem repository with automated signature verification |
