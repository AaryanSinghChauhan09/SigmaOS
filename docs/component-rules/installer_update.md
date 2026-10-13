# Component Rules: Installer & Update

## Component owner
installer-agent

## Upstream references
- Fedora Anaconda / Silverblue OSTree atomic updates
- Linux Mint Installer & Timeshift snapshot recovery
- OpenSUSE YaST & Snapper

## Purpose
Define safety contracts, failure recovery, atomic staging, and clean disk installation procedures for SigmaOS installer and updater.

## Mandatory rules

1. Installation must be testable in QEMU from a clean virtual disk in automated dry-run and full-install modes.
2. Updates must define pre-checks, staged activation, rollback, and recovery paths.
3. A failed update or installation step must either complete atomically or restore the prior known-good system state.
4. Disk partitioning and filesystem formatting operations must require explicit confirmation and safety checks before execution.
5. Recovery boot entries must be automatically generated and validated during update staging.

## Prohibited

- Unrecoverable system state on update interruption or power loss
- Modifying partition tables without target verification
- Promoting installer readiness without clean virtual disk install evidence

## Required evidence

- Installer dry-run unit test logs
- QEMU clean virtual disk installation report
- Update failure injection and rollback recovery report
- Recovery runbook validation output (`docs/operations/RECOVERY_RUNBOOK.md`)

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and recovery runbook architecture |
| Prototype | Installer dry-run and failure injection unit tests |
| Integrated | QEMU clean virtual disk boot and automated installation log |
| Hardened | Power loss / update interruption fault injection recovery verified |
| Supported | Production ISO deployment matrix and multi-disk hardware evidence |
