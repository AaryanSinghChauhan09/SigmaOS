# Component Rules: Security

## Component owner
security-agent

## Upstream references
- OpenBSD Pledge & Unveil
- FreeBSD Capsicum & Casper
- Linux Capability & Landlock Model

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Security, Capability Tokens, Sandboxing, Cryptography, and Audit components.

## Mandatory rules

1. Every privileged operation requires an explicit capability token check on syscall entry.
2. Denial tests are mandatory: protected operations must fail with appropriate error code without the required capability.
3. Package scripts, downloaded binaries, and external repositories are untrusted by default and must run sandboxed.
4. Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.
5. PQC (Post-Quantum Cryptography) signature verification must be enforced on system updates and package signatures.
6. System diagnostics and panic reports must sanitize credentials or secret tokens.

## Prohibited

- Granting implicit ambient privileges without explicit capability token
- Executing untrusted package scripts unsandboxed
- Hardcoding secrets or unversioned mock tokens in production builds
- Merging security-critical PRs without dedicated security review
- Claiming security parity without denial test evidence

## Required evidence

- Capability denial unit & integration test output (`tests/security/capability_test.rs`)
- Sandbox pledge/unveil isolation test reports
- PQC cryptographic signature verification unit test report
- Security audit log and threat model review

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Threat model, capability policy ADR, and security review plan |
| Prototype | Capability check unit tests |
| Integrated | Sandbox enforcement and capability denial integration tests |
| Hardened | SAST/fuzzing, negative security tests, dedicated security review |
| Supported | Production attestation and security audit compliance evidence |
