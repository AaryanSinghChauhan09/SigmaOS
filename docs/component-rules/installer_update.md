# Component Rules: Installer & Update

## Component owner
installer-agent

## Upstream references
- Debian debootstrap & installer pipeline
- Linux Mint Timeshift Pro & system update policies
- Fedora Silverblue / OSTree atomic update model

## Purpose
Define installation repeatability, update atomicity, pre-checks, and recovery runbook enforcement for SigmaOS installer and system update subsystems.

## Mandatory rules

1. Installation must be fully testable in QEMU from a clean virtual disk image.
2. System updates must define explicit pre-checks (disk space, power, ISO checksum), staged activation, rollback, and recovery paths.
3. A failed update must either complete atomically or restore the prior known-good state.
4. Recovery runbooks (`docs/operations/RECOVERY_RUNBOOK.md`) must be tested with failure injection.
5. Storage partition operations must perform dry-run verification before modifying partition tables or filesystems.

## Prohibited

- Non-atomic updates that leave the system in an unbootable intermediate state
- Partition table or bootloader modification without prior state backup
- Unrecoverable installation failures on clean virtual disks
- Marking updates "Supported" without failure injection test evidence

## Required evidence

- Clean QEMU virtual disk installation log (`installer/tests/dry_run.rs`)
- Failure injection and rollback recovery test report (`installer/tests/failure_injection.rs`)
- Automated release/update gate benchmark output (`scripts/release_gate_mint_omarchy_migration.sh`)

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Install & atomic update architecture ADR |
| Prototype | Partitioning and update manifest unit tests |
| Integrated | QEMU clean install and staged update integration tests |
| Hardened | Failure injection, power loss simulation, rollback tests |
| Supported | Reproducible ISO boot, clean install, and update recovery evidence |
