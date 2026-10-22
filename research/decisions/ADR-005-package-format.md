# ADR-005: Packaging Dependency Resolution and Signed Manifests

## Component
Packaging

## Current gap
Package manager lacks dependency conflict resolution and post-quantum manifest verification.

## Upstream study
Arch Linux ALPM/pacman database formats and Debian dpkg dependency solvers.

## Improvement
Implement dependency graph solver and Dilithium5 signed manifest parser.

## Scope
- Manifest parsing
- Dependency graph conflict resolution
- Rollback snapshots
