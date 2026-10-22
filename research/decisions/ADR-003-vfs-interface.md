# ADR-003: VFS Mount and File Descriptor Architecture

## Component
VFS/filesystems

## Current gap
FD table allocation lacks cloexec handling and atomic duplication.

## Upstream study
Linux VFS file descriptor table management and POSIX `dup2`/`CLOEXEC` flags.

## Improvement
Implement bounded process file descriptor table with `FD_CLOEXEC` flags.

## Scope
- FD table allocation
- Atomic `dup2` and `fcntl` flags
