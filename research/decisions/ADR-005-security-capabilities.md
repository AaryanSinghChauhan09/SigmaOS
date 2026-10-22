# ADR-005: Dual pledge/unveil and Landlock Capability Model

## Status
Accepted

## Context
SigmaOS requires least-privilege sandboxing for userspace processes, IPC, and package scripts.

## Upstream Study
OpenBSD `pledge()` restricts system calls; OpenBSD `unveil()` restricts filesystem paths; Linux Landlock provides unprivileged sandboxing.

## Decision
Implement unified `ProcessCapabilityGuard` providing pledge bitmask syscall filters and unveil path restriction trees.

## Consequences
- Unauthorized system call invocation triggers immediate process termination.
- Path restrictions block access to sensitive system directories (`/etc/shadow`).
- Sandboxes are monotonic and cannot re-grant revoked privileges.

## Validation
- Security capability denial unit test suite.
