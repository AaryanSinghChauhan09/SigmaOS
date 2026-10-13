# Component Rules: Security

## Component owner
security-agent

## Upstream references
- FreeBSD Capsicum & MAC Framework
- OpenBSD Pledge & Unveil
- Linux Capabilities & Seccomp

## Purpose
Enforce security boundaries, least privilege, capability verification, sandboxing, and audit logging across all SigmaOS subsystems.

## Mandatory rules

1. Every privileged operation requires an explicit capability token check at the syscall entrypoint.
2. Denial tests are mandatory: protected operations must fail with appropriate error codes when invoked without required capabilities.
3. Package scripts, downloaded binaries, and external repositories are untrusted by default.
4. Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.
5. All security checks must fail closed (deny access by default on error or unknown capability token).

## Prohibited

- Hardcoded root or bypass flags in capability engines
- Unsandboxed execution of third-party package post-install scripts
- Silently ignoring failed security or signature checks
- Parity claims without capability denial test evidence

## Required evidence

- Capability denial unit test report
- Sandbox isolation integration test log
- Cryptographic verification test output
- Security audit review sign-off for privileged syscall changes

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR, threat model, and gap analysis |
| Prototype | Capability check unit tests |
| Integrated | End-to-end sandbox execution and denial test verification |
| Hardened | Static analysis, fuzzing, and security review pass |
| Supported | Formal attestation and continuous vulnerability scanning |
