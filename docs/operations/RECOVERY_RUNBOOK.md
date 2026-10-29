# SigmaOS Operational Recovery Runbook

## Overview
This runbook describes recovery actions for boot failures, installer crashes, broken updates, and kernel panics.

## Scenario 1: Boot Hang / Panic During Kernel Initialization
1. Boot into Live ISO or recovery kernel using GRUB boot menu option `SigmaOS (Recovery Mode)`.
2. Append `sigma.debug=1 serial=ttyS0` to boot parameters.
3. Collect serial debug output via `scripts/diagnostics/collect_system_report.sh`.
4. Inspect `kernel_report.json` for register dumps and subsystem panic origin.

## Scenario 2: Failed System Update / Root Filesystem Corruption
1. Boot into fallback partition snapshot:
   ```bash
   sigma-bootselect --select fallback
   ```
2. Verify integrity of secondary root partition:
   ```bash
   sigma-fsck /dev/sda2
   ```
3. Roll back atomic deployment:
   ```bash
   sigpkg-admin rollback --target /mnt/root
   ```

## Scenario 3: Broken Installer Partitioning
1. Boot Live environment.
2. Restore pre-installation MBR/GPT backup:
   ```bash
   dd if=/tmp/sigma_preinstall_mbr.bak of=/dev/sda bs=512 count=34
   ```
3. Re-scan partition table with `partprobe /dev/sda`.
