# Component Rules: Security

## Component owner
security-agent

## Upstream references
- OpenBSD pledge/unveil & pinsyscall
- FreeBSD Capsicum
- Linux Landlock & Capability model

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS security components.

## Mandatory rules

1. Every privileged operation requires an explicit capability.
2. Denial tests are mandatory: protected operations must fail without the required capability.
3. Package scripts, downloaded binaries, and external repositories are untrusted by default.
4. Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.

## Prohibited

- Default root or unconstrained execution for user processes
- Unchecked syscall capability escalation
- Merging security changes without a completed security review

## Required evidence

- Capability denial test report
- Fuzzing & security audit report
- QEMU sandbox enforcement log

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Integration denial tests |
| Hardened | Fuzzing & dedicated security review |
| Supported | Proven sandbox security in release builds |
