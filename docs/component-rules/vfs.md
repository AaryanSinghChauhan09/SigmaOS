# Component Rules: VFS

## Component owner
filesystem-agent

## Upstream references
- Linux VFS
- xv6 filesystem
- FreeBSD VFS

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS VFS components.

## Mandatory rules

1. All path resolution must use the central resolver.
2. Every filesystem operation must return a standardized error code.
3. A failed operation must leave filesystem state coherent.
4. New filesystem backends must implement the VFS trait contract.
5. Path traversal, invalid components, and mount boundaries require tests.
6. No VFS change may bypass capability checks.
7. Every VFS change must include unit, integration, and QEMU runtime evidence.

## Prohibited

- Direct filesystem access outside the VFS abstraction
- Silent error swallowing
- Unsupported parity claims
- Status promotion without runtime evidence

## Required evidence

- Unit test report
- Integration test report
- QEMU RAM-disk boot log
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
