# Component Rules: Security

## Component owner
security-agent

## Upstream references
- OpenBSD pledge and unveil
- FreeBSD Capsicum
- Linux seccomp and namespaces

## Purpose
Define required privilege separation, capability models, sandbox invariants, and validation
for SigmaOS security boundaries.

## Mandatory rules

1. Every privileged operation requires an explicit capability.
2. Denial tests are mandatory: protected operations must fail without the required capability.
3. Package scripts, downloaded binaries, and external repositories are untrusted by default.
4. Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.

## Prohibited

- Implicit root/superuser bypasses
- Granting ambient authority across IPC or syscall boundaries
- Unsandboxed execution of post-install package scripts
- Merging security-critical changes without dedicated review

## Required evidence

- Capability grant and capability denial test suites
- Sandbox syscall-filter verification report
- Security review sign-off document
- Regression tests for known exploit vectors

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Security threat model and capability architecture ADR |
| Prototype | Capability table and policy unit tests |
| Integrated | Sandboxed process execution in QEMU |
| Hardened | Automated fuzzing, penetration denial tests, and third-party review |
| Supported | Production capability enforcement and audited sandbox profiles |
