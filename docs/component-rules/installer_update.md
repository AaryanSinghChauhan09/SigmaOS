# Component Rules: Installer & Update

## Component owner
installer-agent

## Upstream references
- Debian debootstrap & installer
- Fedora Anaconda & OSTree updates
- Linux Mint Ubiquity/Subiquity

## Purpose
Define required disk partitioning, OS installation, system update, and rollback safety standards.

## Mandatory rules

1. Installation must be testable in QEMU from a clean virtual disk.
2. Updates must define pre-checks, staged activation, rollback, and recovery.
3. A failed update must either complete atomically or restore the prior state.
4. Disk partitioning and formatting operations must require explicit confirmation.
5. Bootloader installation must verify target ESP/MBR sanity before writing.

## Prohibited

- Non-recoverable OS update states
- Destructive disk partition changes in dry-run or automated test modes
- Silent bootloader installation failure
- Unverified update payload execution

## Required evidence

- Installer dry-run unit test report
- QEMU clean virtual disk installation report
- Atomic update and rollback failure recovery log
- Bootloader verification test report

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and installation flow design |
| Prototype | Dry-run partition and installation unit tests |
| Integrated | QEMU automated disk install and boot test |
| Hardened | Power-loss failure recovery and rollback verification |
| Supported | Proven bare-metal / QEMU install and atomic update evidence |
