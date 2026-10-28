# Component Rules: Security

## Component owner
security-agent

## Upstream references
- OpenBSD pledge and unveil
- FreeBSD Capsicum sandbox model
- Linux capabilities & seccomp

## Purpose
Define privilege isolation, capability enforcement, and sandboxing rules
for SigmaOS Security components.

## Mandatory rules

1. Every privileged operation requires an explicit capability.
2. Denial tests are mandatory: protected operations must fail without the required capability.
3. Package scripts, downloaded binaries, and external repositories are untrusted by default.
4. Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.

## Prohibited

- Default root or ambient privilege exposure
- Unsandboxed execution of untrusted package scripts
- Security claims without denial test verification
- Status promotion without security review signoff

## Required evidence

- Capability denial test report
- Fuzzing & negative security check log
- Security audit signoff report
- Regression test for every security advisory

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
