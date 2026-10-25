# Component Rules: Security

## Component owner
security-agent

## Upstream references
- OpenBSD pledge/unveil
- FreeBSD Capsicum sandbox model
- Linux Landlock & Seccomp-BPF

## Purpose
Define required security behavior, access control boundaries, and validation for SigmaOS.

## Mandatory rules

1. Every privileged operation requires an explicit capability.
2. Denial tests are mandatory: protected operations must fail without the required capability.
3. Package scripts, downloaded binaries, and external repositories are untrusted by default.
4. Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.
5. All security boundaries must log access violations with diagnostic context.

## Prohibited

- Unchecked root/superuser overrides
- Execution of untrusted package scripts outside a capability sandbox
- Silent capability bypasses
- Hardcoded security bypass tokens or mock credentials

## Required evidence

- Capability denial unit & integration test report
- Sandbox path isolation test log
- Cryptographic verification test suite log
- Security review sign-off document

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and threat model specification |
| Prototype | Capability denial unit tests |
| Integrated | Sandboxed process execution in QEMU |
| Hardened | Fuzzing, negative permission tests, security review |
| Supported | Proven zero-privilege violation evidence |
