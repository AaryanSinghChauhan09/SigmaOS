# Component Rules: Packaging

## Component owner
packaging-agent

## Upstream references
- Debian dpkg & APT
- Arch Linux pacman & PKGBUILD
- Nix / Guix package model

## Purpose
Define required package metadata parsing, dependency resolution, and sandboxed package execution rules.

## Mandatory rules

1. Package metadata must be parsed before any installation action.
2. Dependency resolution must support dry-run mode.
3. Package scripts must run only in a sandbox with declared capabilities.
4. Updates must be atomic or recoverable; failed updates must restore the prior known-good state.
5. All package signatures must be cryptographically verified prior to execution.

## Prohibited

- Unchecked execution of arbitrary shell maintainer scripts
- Destructive package file overwrites outside the package database tracker
- Silent dependency resolution failures
- Promotion of package management features without dry-run test evidence

## Required evidence

- Metadata parser unit test report
- Dependency resolver dry-run integration report
- Sandboxed maintainer script execution test log
- Corrupted package rejection test evidence

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and package specification format |
| Prototype | Metadata parser unit tests |
| Integrated | Dry-run dependency resolution and installation test |
| Hardened | Script sandboxing security review and signature verification |
| Supported | Reproducible package installation and rollback verification |
