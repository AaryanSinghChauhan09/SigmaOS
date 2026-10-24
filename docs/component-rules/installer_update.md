# Component Rules: Installer & Update

## Component owner
installer-agent

## Upstream references
- Fedora rpm-ostree / Silverblue atomic updates
- Linux Mint Timeshift / MintUpgrade
- Arch install / ISO build pipelines

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS installer and update components.

## Mandatory rules

1. Installation must be testable in QEMU from a clean virtual disk.
2. Updates must define pre-checks, staged activation, rollback, and recovery.
3. A failed update must either complete atomically or restore the prior state.

## Prohibited

- Destructive partition actions without explicit confirmation & dry-run validation
- Non-atomic updates that leave the rootfs unbootable on failure
- Installer claims without clean disk QEMU test logs

## Required evidence

- Clean disk QEMU installation log
- Atomic update failure & rollback test log
- Disk layout & partition verification report

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | QEMU clean disk install & atomic update test |
| Hardened | Automated power-loss interruption recovery |
| Supported | Reproducible ISO boot, install, and update evidence |
