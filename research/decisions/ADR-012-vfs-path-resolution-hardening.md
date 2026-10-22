# ADR-012: VFS Path-Resolution Hardening

## Component
VFS/filesystems

## Current gap
Path resolution lacks consistent handling of traversal, invalid components, and error propagation.

## Upstream study
Linux VFS separates generic path walking from filesystem-specific operations; xv6 uses a simpler inode/path model.

## Improvement
Introduce a single path-resolution function with explicit error codes, component validation, and mount-point awareness.

## Scope
- Absolute and relative paths
- `/`, `.`, and `..`
- Invalid path components
- Standardized error codes
- Regression tests

## Non-goals
- Symlinks
- Permissions
- Mount namespaces
- Persistent filesystems

## Validation
- Unit tests
- VFS integration tests
- QEMU boot test using RAM-disk root
