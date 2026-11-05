# Component Rule: ipc.md

## Component owner
ipc-agent

## Upstream references
- Linux kernel and userland
- FreeBSD, OpenBSD, NetBSD architectures
- Industry standards and POSIX specifications

## Purpose
Inter-process communication, pipes, and signals

## Mandatory rules
1. All changes must follow PR-based review.
2. Must compile cleanly in bare-metal and userland targets.
3. Must maintain backward compatibility where applicable.

## Prohibited
- Direct commits without testing.
- Bypassing security boundaries or assertions.
- Unchecked memory leaks or raw pointers without safety guarantees.

## Required evidence
- Unit and integration tests passing.
- Runtime logs and verifiable benchmarks.
- Security review and lint audits.

## Promotion criteria
- Proposed to Prototype: Initial functional implementation and basic unit tests.
- Prototype to Integrated: Full integration tests passing with test coverage.
- Integrated to Hardened: Fuzzing, negative tests, and security review.
- Hardened to Supported: Reproducible boot, install, and update evidence.
