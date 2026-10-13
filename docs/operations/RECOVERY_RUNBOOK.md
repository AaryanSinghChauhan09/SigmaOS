# SigmaOS Installer Safety and Recovery Runbook

## Overview

This runbook outlines the operational procedures for rehearsing the SigmaOS installation and recovery paths. Destructive disk operations must always enforce dry-run validation, failure injection safeguards, and documented recovery workflows.

## Safety Principles

1. **Dry-Run Default:** The installer defaults to `--dry-run` mode, logging proposed partition modifications without touching disk blocks.
2. **Atomic Mutations:** Partitioning and file staging use transactional staging directories.
3. **Failure Isolation:** Interruptions during system installation trigger automatic rollback to pre-installation disk state.

## Installer Rehearsal Procedure

### 1. Dry-Run Validation
Verify that the installer correctly simulates disk partitioning and package extraction without writing to physical devices:

```bash
cargo test --test dry_run -- --nocapture
```

Key invariant: Disk write calls must equal `0` when `--dry-run` flag is active.

### 2. Failure Injection & Rollback Rehearsal
Simulate power loss or disk failure during package staging and verify rollback cleanup:

```bash
cargo test --test failure_injection -- --nocapture
```

Key invariant: Temporary staging mounts and lock files are safely unmounted and removed.

## Disaster Recovery Runbook

### Scenario A: Interrupted System Update
- **Symptom:** Kernel fails to load new initramfs image after interrupted package update.
- **Recovery Steps:**
  1. Boot into SigmaOS Rescue ISO environment.
  2. Mount target root filesystem: `mount /dev/sda2 /mnt/target`.
  3. Execute package rollback helper: `sigpkg-recovery --rollback --root /mnt/target`.
  4. Verify `/boot/vmlinuz-sigma` symlink points to previous working snapshot.
  5. Reboot system.

### Scenario B: Storage Subsystem Read-Only Fallback
- **Symptom:** Storage driver detects IO error and remounts filesystem read-only (`EROFS`).
- **Recovery Steps:**
  1. Run hardware diagnostics: `./scripts/hardware/collect_diagnostics.sh`.
  2. Inspect storage block device ledger for SMART warnings or block errors.
  3. Run filesystem check: `fsck.sigma /dev/sda2`.
  4. Remount filesystem read-write: `mount -o remount,rw /mnt/target`.
