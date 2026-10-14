# SigmaOS Installer & Recovery Runbook

## Overview

SigmaOS requires destructive operations (installer, disk partitioning, system updates, recovery) to be atomic, safe, and fully recoverable.

## Principles of Safe Installation and Recovery

1. **Dry-Run Mode First**: Every installation or update transaction must support a dry-run execution mode (`--dry-run`) that verifies system constraints without mutating disk partitions or writing filesystems.
2. **Failure Injection Resiliency**: System installers must handle mid-transaction failures (power loss, corrupt payload, disk full) by reverting state or providing a clean rollback path.
3. **Emergency Chroot Environment**: The installer ISO and recovery environment provide a minimal rescue shell with chroot capabilities to repair damaged bootloaders, `/etc/fstab`, or corrupted kernel images.

## Recovery Procedures

### 1. Emergency Rescue Boot
Boot the SigmaOS Live Installer ISO and select **Emergency Recovery Shell** from the GRUB boot menu.

### 2. Mounting Target System
```bash
# Locate root partition
sigma-disks --list

# Mount target root and virtual filesystems
mount /dev/sda2 /mnt
mount -t proc proc /mnt/proc
mount -t sysfs sys /mnt/sys
mount --bind /dev /mnt/dev

# Chroot into target system
chroot /mnt /bin/sh
```

### 3. Repairing Bootloader and Init Configuration
```bash
# Reinstall bootloader
sigma-bootinstall --target=/dev/sda

# Re-generate initramfs / boot configuration
sigma-init-update --rebuild
```

### 4. Timeshift / Snapshot Rollback
```bash
# List available system snapshots
timeshift --list

# Restore to last known good snapshot
timeshift --restore --snapshot "2026-10-14_02-00-00"
```

## Testing Recovery Workflows

Installer safety and failure injection unit tests are defined in:
- `installer/tests/dry_run.rs`
- `installer/tests/failure_injection.rs`

Run tests via:
```bash
rustc --test --edition=2021 installer/tests/dry_run.rs -o build/test_dry_run && ./build/test_dry_run
rustc --test --edition=2021 installer/tests/failure_injection.rs -o build/test_failure_injection && ./build/test_failure_injection
```
