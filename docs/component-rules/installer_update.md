# Component Rules: Installer & Atomic Updates

## Component owner
installer-agent

## Upstream references
- Fedora Ignition / OSTree
- Linux Mint Installer & Timeshift
- Archinstall & Calamares

## Purpose
Govern system installation, partition formatting, atomic updates, and snapshot recovery.

## Mandatory rules

1. Installation must be testable in QEMU from a clean virtual disk.
2. Updates must define pre-checks, staged activation, rollback, and recovery paths.
3. A failed update must either complete atomically or restore the prior known-good state.
4. Partitioning and formatting operations must require explicit user confirmation and pre-flight validation.
5. Bootloader entries must be auto-generated with rollback entries for prior system states.

## Prohibited

- Destructive partition changes without user confirmation or dry-run checks
- Unrecoverable system update states
- Hardcoding disk device paths (e.g. `/dev/sda`) without disk probe verification
- Promoting installer or update capabilities without clean QEMU virtual disk evidence

## Required evidence

- Partitioning and update engine unit tests (`cargo test --lib`)
- QEMU clean virtual disk installation log
- Atomic update failure and rollback verification log

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and atomic update state machine design |
| Prototype | Partitioning and update state unit tests |
| Integrated | QEMU automated clean-disk installation log |
| Hardened | Power-failure simulated mid-update rollback test |
| Supported | End-to-end install, update, and recovery evidence |
