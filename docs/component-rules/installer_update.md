# Component Rules: Installer and Update

## Component owner
installer-agent

## Upstream references
- Fedora Silverblue / rpm-ostree
- Linux Mint Installer (Ubiquity / Calamares)
- openSUSE transactional-update

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS system installation, disk provisioning, and system updates.

## Mandatory rules

1. Installation must be testable in QEMU from a clean virtual disk.
2. Updates must define pre-checks, staged activation, rollback, and recovery.
3. A failed update must either complete atomically or restore the prior state.

## Prohibited

- Non-atomic updates leaving intermediate or unbootable state
- Inability to revert an aborted or failed system update
- Unpartitioned or unverified disk formatting without user confirmation
- Promotion without QEMU clean disk installation logs

## Required evidence

- Clean install QEMU virtual disk automated test log
- Atomic update verification report
- Update failure injection and automatic rollback test report
- Recovery runbook test evidence

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests and disk layout simulator |
| Integrated | QEMU clean virtual disk install and rollback test |
| Hardened | Power-cut failure injection during update, automated recovery |
| Supported | Reproducible bare-metal and virtual machine installation evidence |
