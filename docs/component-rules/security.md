# Component Rules: Security

## Component owner
security-agent

## Upstream references
- OpenBSD Pledge & Unveil Architecture
- FreeBSD Capsicum Capability & Sandboxing Framework
- Linux Security Modules (LSM) & seccomp-BPF

## Purpose
Define privilege management, capability enforcement, threat modeling, and audit requirements for SigmaOS security components.

## Mandatory rules

1. Every privileged operation requires an explicit capability (`CapabilityToken`).
2. Denial tests are mandatory: protected operations must fail explicitly without the required capability.
3. Package scripts, downloaded binaries, and external repositories are untrusted by default.
4. Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.
5. All user-space input parameters (pointers, lengths, buffers) must be validated before kernel consumption.

## Prohibited

- Implicit privilege escalation or default-allow capability checks
- Executing package hooks or external scripts without capability sandboxing
- Bypassing W^X memory protection or kernel protection rings
- Merging security-critical PRs without dedicated security review approval

## Required evidence

- Capability denial test suite logs
- Input validation and boundary check test report (`src/security/input_validation.rs`)
- Static analysis and security review sign-off
- Fuzz target execution evidence

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Threat model and security policy ADR |
| Prototype | Capability check unit tests |
| Integrated | End-to-end sandbox and denial integration tests |
| Hardened | Automated fuzzing, penetration analysis, security review |
| Supported | Production security posture verification and audit logs |
