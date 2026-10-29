# Component Rules: Packaging

## Component owner
package-agent

## Upstream references
- Arch Linux pacman & PKGBUILD / devtools
- Debian APT / dpkg & Lintian
- Sovereign Universal Package Format (sigpkg)

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS packaging components.

## Mandatory rules

1. Package metadata must be parsed before any installation action.
2. Dependency resolution must support dry-run mode.
3. Package scripts must run only in a sandbox with declared capabilities.
4. Updates must be atomic or recoverable; failed updates must restore the prior known-good state.

## Prohibited

- Unsandboxed execution of package scriptlets
- Silent dependency resolution failures
- Non-atomic package file overwrites

## Required evidence

- Packaging metadata corpus parser test report
- Dependency resolver dry-run report
- Sandboxed installation test log

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests & metadata parser |
| Integrated | Dry-run dependency resolution & sandboxing |
| Hardened | Malformed package fuzzing |
| Supported | Production package repository synchronization evidence |
