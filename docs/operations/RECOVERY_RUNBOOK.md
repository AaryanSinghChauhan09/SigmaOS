# SigmaOS System Recovery Runbook

## Overview
This runbook provides emergency procedures for recovering a SigmaOS instance from boot failures, broken updates, or storage corruption.

## Emergency Boot Recovery

### 1. Emergency Diagnostic Shell
If system initialization halts prior to userland launch, the kernel invokes the built-in diagnostic prompt:
- **Marker:** `[SIGMA_DIAGNOSTIC_SHELL_ACTIVE]`
- **Commands:**
  - `dmesg`: Display early kernel printk buffer and panic traces.
  - `fsck /dev/sda2`: Execute filesystem integrity check.
  - `rollback`: Revert to the last known good snapshot.

### 2. Atomic Snapshot Rollback
SigmaOS maintains Btrfs / ZFS / hardlink system snapshots managed by `src/resilience/backup.rs` (`LinuxMintTimeshiftProEngine`).
To revert to a prior state from the emergency shell or live USB media:

```bash
# List available system snapshots
sigrabackup --list-snapshots

# Restore snapshot YYYY-MM-DD-HHMMSS
sigmabackup --restore YYYY-MM-DD-HHMMSS --target /dev/sda2
```

### 3. Installer Recovery Mode
Boot from the SigmaOS Live ISO installer and select **"Repair Existing System"**:
1. Mounts target root partition at `/mnt/target`.
2. Runs `installer/src/recovery/mod.rs` self-test diagnostics.
3. Re-installs EFISTUB / GRUB bootloader entries to `/mnt/target/boot/efi`.
