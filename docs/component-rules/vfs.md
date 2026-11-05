# Vfs Component Rules

## Component owner
filesystem-agent

## Upstream references
POSIX VFS, OpenZFS, ext4

## Purpose
Virtual filesystem abstractions and disk operations

## Mandatory rules
- Must include unit tests verifying boundary conditions.
- Must preserve backwards compatibility or provide automated migration paths.
- Must pass linting and static analysis without warnings.

## Prohibited
- Direct commits to main branch without PR validation.
- Undocumented unsafe operations.
- Unbounded blocking operations on critical fast-paths.

## Required evidence
- Unit test suite execution logs.
- Integration tests or runtime environment logs.

## Promotion criteria
- Proposed: ADR and design specification.
- Prototype: Unit tests pass in CI.
- Integrated: Runtime integration evidence verified.
- Hardened: Fuzzing, negative tests, and security reviews passed.
- Supported: Reproducible boot/install/update evidence validated.
