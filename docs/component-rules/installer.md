# Component Rules: Installer & Updates

## Component owner
installer-agent

## Upstream references
- Linux Mint Timeshift snapshot & restore system
- Fedora Silverblue / OSTree atomic deployment
- Archinstall / Calamares installer framework

## Purpose
Mandate clean QEMU disk installation testing, pre-checks, staged activation, atomic updates, and automatic rollback.

## Mandatory rules
1. Installation must be testable in QEMU from a clean virtual disk.
2. Updates must define pre-checks, staged activation, rollback, and recovery.
3. A failed update must either complete atomically or restore the prior state.
4. Pre-flight checks must verify AC power, available disk space, and snapshot integrity before applying system updates.
5. Bootloader entries for previous system snapshots must be generated automatically.

## Prohibited
- Unrecoverable system update failures
- Overwriting active root partition without pre-snapshot creation
- Untested installation scripts
- Non-deterministic update manifest processing

## Required evidence
- Automated installer dry-run test report
- QEMU clean virtual disk installation log
- Forced update failure and atomic rollback verification report
- Bootloader snapshot entry generation test

## Promotion criteria
| Level | Requirement |
|---|---|
| Proposed | Atomic update & installer architecture ADR |
| Prototype | Installer dry-run unit tests |
| Integrated | QEMU clean virtual disk install & rollback test |
| Hardened | Power loss / disk full failure injection tests |
| Supported | Reproducible bare-metal / QEMU install evidence |
