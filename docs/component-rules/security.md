# Component Rules: Security

## Component owner
security-agent

## Upstream references
- OpenBSD pledge/unveil
- FreeBSD Capsicum
- Linux seccomp / capabilities

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Security, isolation, and access control.

## Mandatory rules

1. Every privileged operation requires an explicit capability.
2. Denial tests are mandatory: protected operations must fail without the required capability.
3. Package scripts, downloaded binaries, and external repositories are untrusted by default.
4. Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.

## Prohibited

- Implicit or ambient privileges
- Disabling capability checks in test or release builds
- Executing unverified or unsigned third-party scripts outside the sandbox
- Bypassing separate security review for sensitive subsystems

## Required evidence

- Positive capability access tests
- Negative denial tests (verifying EPERM / permission rejection)
- Dedicated security review sign-off
- Fuzzing reports on untrusted input boundaries
- Regression test for every vulnerability or security fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests and initial denial tests |
| Integrated | Runtime capability enforcement and denial test suite |
| Hardened | Fuzzing, negative tests, separate security review sign-off |
| Supported | Reproducible boot/install/update evidence |
