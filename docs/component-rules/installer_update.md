# Component Rules: Installer & Update

## Component owner
installer-agent

## Upstream references
- Debian Subiquity / Calamares installer engine
- Fedora Silverblue / OSTree A/B update mechanism
- Linux Mint Update Manager (mintupdate)

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS Installer and Update components.

## Mandatory rules

1. Installation must be testable in QEMU from a clean virtual disk.
2. Updates must define pre-checks, staged activation, rollback, and recovery.
3. A failed update must either complete atomically or restore the prior state.
4. Storage formatting or partition modification requires explicit confirmation safety checks.

## Prohibited

- Destructive disk operations without dry-run validation options
- Unrecoverable update failures that leave system unbootable
- Direct drive modification without verifying target disk identifiers
- Parity claims without clean virtual disk installation logs

## Required evidence

- Clean virtual disk QEMU installation log
- Failure injection & atomic rollback verification test output
- System update dry-run report
- Recovery runbook execution trace

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
