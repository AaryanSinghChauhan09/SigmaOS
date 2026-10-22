# ADR-004: Universal Package Format and Staged Atomic Updates

## Status
Accepted

## Context
SigmaOS needs transparent package archives with dependency resolution and failure recovery.

## Upstream Study
Arch Pacman provides transparent PKGINFO and tar.zst; Linux Mint and Fedora ostree provide atomic update rollbacks.

## Decision
Adopt dual-layer `.sigpkg` format, SAT dependency solver, and Timeshift-style staged update rollback triggers.

## Consequences
- Zero partial package installation state corruption.
- Automated system snapshot restore on pre/post-install script failure.
- Package build and hook execution runs in sandboxed environments.

## Validation
- Simulated failed package update triggers pre-update snapshot restore.
