# SigmaOS Installer Safety & Recovery Runbook

## Overview

SigmaOS requires non-destructive, transactional installation and update procedures. Destructive disk mutation operations must be gated behind dry-run confirmation and support atomic rollback.

---

## Pre-Flight Requirements & Dry-Run Mode

Before modifying any partition table or writing filesystems, the installer must execute a dry-run:

```bash
# Execute dry-run installation test harness
rustc --test installer/tests/dry_run.rs --edition=2021 -o build/test_installer_dry_run
./build/test_installer_dry_run
```

Dry-run mode verifies:
1. Target block device existence and storage capacity.
2. Partition boundaries and alignment checks.
3. Target filesystem support (`ext4`, `CoW`, etc.).
4. Non-destructive plan generation without writing to disk.

---

## Failure Injection and Rehearsal

Simulate installation failure points (partitioning, formatting, extraction, bootloader installation) to verify clean cleanup:

```bash
# Execute failure injection integration test
rustc --test installer/tests/failure_injection.rs --edition=2021 -o build/test_installer_failure
./build/test_installer_failure
```

---

## Emergency Recovery Steps

1. **Interrupted Package Extraction**: Reboot into Live ISO environment, run `/usr/bin/sigpkg recover-transaction`.
2. **Bootloader Corruption**: Boot Live ISO, run `chroot /sysroot` and reinstall GRUB/SigmaBootloader.
3. **Partition Table Mismatch**: Restore partition table snapshot from backup location `/etc/sigmaos/partition_backup.bin`.
