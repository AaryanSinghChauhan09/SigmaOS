# SigmaOS Recovery & Disaster Recovery Runbook

## Emergency Boot Options

When system startup fails or installer update fails, pass one of the following kernel parameters at the bootloader menu:

- `sigmaos.recovery=1`: Boots directly into emergency maintenance shell with minimal read-only root mount.
- `sigmaos.rollback=latest`: Automatically restores the previous boot snapshot created by Timeshift / Btrfs / ZFS.
- `sigmaos.init=/bin/sh`: Direct single-user shell entry.

---

## Recovery Procedures

### 1. Failed System Update Rollback
```bash
# Check available snapshot generations
sigma-timeshift --list

# Restore prior stable generation
sigma-timeshift --restore --snapshot 2026-10-14_02-00
```

### 2. Filesystem & Inode Repair
```bash
# Perform non-destructive filesystem check on target root
sigma-fsck /dev/sda2 --dry-run
```

### 3. Emergency Diagnostics Collection
```bash
# Run diagnostics collector script
/scripts/diagnostics/collect_system_report.sh /var/log/recovery_report.json
```
