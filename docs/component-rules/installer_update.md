# Component Rules: Installer & Update

## Component owner
installer-agent

## Upstream references
- Debian / Ubuntu Subiquity & Calamares Installer
- Fedora Silverblue / OSTree A/B Updates
- Mint Timeshift Snapshot & Recovery Manager

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Installer, System Updates, Rollback, and Recovery components.

## Mandatory rules

1. Installation must be testable in QEMU from a clean virtual disk (`installer/tests/dry_run.rs`, `install_dry_run.rs`).
2. Updates must define pre-checks, staged activation (A/B slots), rollback, and failure recovery.
3. A failed update or installation must either complete atomically or restore the prior known-good state cleanly without data loss.
4. Installer disk partitioning and filesystem formatting operations must require explicit user confirmation and safety guards.
5. Recovery runbook (`docs/operations/RECOVERY_RUNBOOK.md`) must be kept up to date for all installer and update error codes.

## Prohibited

- Unrecoverable drive formatting without safety confirmation prompts
- Non-atomic updates leaving half-written system states on power failure
- Omitting rollback paths for system updates
- Failing silently during installer partitioning or bootloader installation
- Status promotion without QEMU installation dry-run or update rollback log

## Required evidence

- Installer dry-run test suite report (`installer/tests/install_dry_run.rs`)
- Failure injection and recovery test output (`installer/tests/failure_injection.rs`)
- QEMU clean disk installation log
- A/B slot update and rollback verification report

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Installer design ADR and recovery runbook specification |
| Prototype | Installation dry-run unit tests |
| Integrated | QEMU virtual disk clean install & boot test |
| Hardened | Power-failure injection tests, rollback verification |
| Supported | Reproducible bare-metal / QEMU install-and-update evidence |
