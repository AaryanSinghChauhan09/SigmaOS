# Component Rules: Installer/update

## Component owner
installer-agent

## Upstream references
- Fedora Silverblue / rpm-ostree atomic updates
- Linux Mint installer (Ubiquity / live-installer)
- Archinstall unattended deployment framework

## Purpose
Define required reliability, disk partitioning, rollback guarantees, and validation
for the SigmaOS system installer and image updater.

## Mandatory rules

1. Installation must be testable in QEMU from a clean virtual disk.
2. Updates must define pre-checks, staged activation, rollback, and recovery.
3. A failed update must either complete atomically or restore the prior state.
4. Partitioning tools must validate disk geometry and partition alignment before writing partition tables.
5. Bootloader installation must verify ESP (EFI System Partition) structure and NVRAM entries.
6. System recovery images must provide an independent minimal boot environment.
7. Post-update verification must confirm kernel and root filesystem checksums before finalizing boot entries.

## Prohibited

- Unchecked disk writes without partition table dry-run validation
- Irreversible partial system state transitions
- Claiming install readiness without reproducible clean-disk QEMU test runs
- Status promotion without automated install/rollback evidence

## Required evidence

- Unit test report
- Integration test report
- Clean install and rollback test log (`reports/installer/clean-install.log`)
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
