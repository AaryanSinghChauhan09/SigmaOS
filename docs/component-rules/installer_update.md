# Component Rules: Installer and Update

## Component owner
installer-agent

## Upstream references
- Fedora Silverblue / rpm-ostree
- Linux Mint installer (Ubiquity / Calamares)
- Debian Installer

## Purpose
Define required reliability, disk integrity, staged activation, rollback mechanisms,
and validation for the SigmaOS installer and system update pipeline.

## Mandatory rules

1. Installation must be testable in QEMU from a clean virtual disk.
2. Updates must define pre-checks, staged activation, rollback, and recovery.
3. A failed update must either complete atomically or restore the prior state.

## Prohibited

- Non-reproducible installation procedures
- In-place destructive system updates without snapshot or recovery path
- Leaving the system unbootable upon an interrupted update
- Silent failures during disk partitioning or bootloader installation

## Required evidence

- QEMU clean virtual disk installation test run log
- Simulated update interruption with verified automatic rollback
- Pre-update health check test reports
- Partition integrity and checksum audit logs

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Installation architecture and atomic update ADR |
| Prototype | Partitioning and image extraction unit tests |
| Integrated | QEMU end-to-end installation test from ISO/disk |
| Hardened | Power-loss simulation during update with verified recovery |
| Supported | Multi-hardware installation matrix with reproducible update verification |
