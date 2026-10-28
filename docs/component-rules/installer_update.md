# Component Rules: Installer & Update

## Component owner
installer-agent

## Upstream references
- Fedora / RHEL Anaconda & OSTree updates
- Debian Subiquity / debian-installer
- Linux Mint Timeshift & Update Manager

## Purpose
Define installation verification, atomic updates, and recovery rollback safety
for SigmaOS Installer & Update components.

## Mandatory rules

1. Installation must be testable in QEMU from a clean virtual disk.
2. Updates must define pre-checks, staged activation, rollback, and recovery.
3. A failed update must either complete atomically or restore the prior state.

## Prohibited

- Unrecoverable installation or update failures
- Modifying target disk state without user confirmation or dry-run validation
- Incomplete updates leaving system in an unbootable state
- Status promotion without full QEMU clean install/update test logs

## Required evidence

- QEMU clean disk install report
- Staged update and atomic rollback log
- Recovery runbook validation report
- Regression test for failed update recovery

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
