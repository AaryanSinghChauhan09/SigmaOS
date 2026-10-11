# ADR-006: Capability-Based Process Sandboxing and Pledge Promises

## Status
Accepted

## Upstream study
OpenBSD `pledge`/`unveil` system call confinement and FreeBSD Capsicum rights.

## SigmaOS decision
Implement capability promise filters (`stdio`, `rpath`, `wpath`, `cpath`, `inet`, `exec`) and VFS unveil path restrictions for sandboxed processes.

## Scope
- Pledge promise bitmask evaluation during syscall dispatch
- Path unveil restriction table per process
- Immutable security audit logging for denial events

## Non-goals
- Dynamic pledge capability expansion at runtime
- Unrestricted root bypass of unveil rules
- Full LSM stacked hooks beyond pledge/unveil

## Validation
- Unit tests verifying unauthorized syscall denial
- Integration tests confirming unveil path isolation
