# Component Rules: Packaging & Distribution

## Component owner
package-management-agent

## Upstream references
- Arch Linux pacman & PKGBUILD
- Debian apt/dpkg & debconf
- Nix / Guix package store mechanics

## Purpose
Ensure deterministic, safe, and verifiable package management and dependency resolution.

## Mandatory rules

1. Package metadata must be parsed and validated before any installation action.
2. Dependency resolution must support a dry-run mode that calculates actions without mutating state.
3. Package scripts must run only in a sandbox with declared capabilities.
4. Updates must be atomic or recoverable; failed updates must restore the prior known-good state.
5. All packages must support cryptographic signature verification (e.g. Dilithium-5 / PQC or GPG).

## Prohibited

- Executing arbitrary package scripts with root host privileges
- Non-atomic package file overwrites that leave broken state on failure
- Unsigned package installations in production modes
- Status promotion without package dry-run and sandbox test evidence

## Required evidence

- Metadata parser and dependency solver unit tests (`cargo test --lib`)
- Package dry-run resolution report
- Sandboxed execution test log

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and package specification format |
| Prototype | Metadata parser and dependency solver unit tests |
| Integrated | Dry-run package installation in sandboxed runtime |
| Hardened | PQC signature verification and rollback recovery tests |
| Supported | Reproducible package build and repository distribution evidence |
