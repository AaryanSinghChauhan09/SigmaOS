# Component Rules: Security

## Component owner
security-agent

## Upstream references
- OpenBSD pledge/unveil & Pinsyscall
- FreeBSD Capsicum sandbox framework
- Linux Seccomp & Landlock

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS Security components.

## Mandatory rules

1. Every privileged operation requires an explicit capability.
2. Denial tests are mandatory: protected operations must fail without the required capability.
3. Package scripts, downloaded binaries, and external repositories are untrusted by default.
4. Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.

## Prohibited

- Implicit root or elevated privilege escalation
- Unsandboxed execution of untrusted package scripts
- Merging security changes without negative/denial tests
- Disabling security checks for convenience

## Required evidence

- Security capability denial test suite report
- Sandbox containment verification log
- Dedicated security review sign-off
- Fuzzing and static analysis reports

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
