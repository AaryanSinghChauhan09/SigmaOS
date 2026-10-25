# System Recovery & Emergency Operating Runbook

## Emergency Boot Protocols

If a system update or driver installation causes boot failure, SigmaOS provides automated and manual recovery paths.

---

## Recovery Modes

### 1. Automatic Safe Mode / Snapshot Rollback
- On 3 consecutive failed boot attempts, the bootloader automatically selects the previous known-good CoW snapshot / Slot A.
- Recovery daemon restores `/etc` configuration backup.

### 2. Live Recovery ISO Shell
Boot using the official SigmaOS Live Installer ISO:
```bash
# Mount target root partition
mount /dev/sda2 /mnt

# Mount API filesystems
mount -t proc proc /mnt/proc
mount -t sysfs sys /mnt/sys
mount -o bind /dev /mnt/dev

# Chroot into target system
chroot /mnt /bin/sh

# Trigger package rollback or kernel restore
sigpkg rollback --generation -1
```

### 3. Emergency Kernel Recovery Console
Select "SigmaOS Safe Recovery" in GRUB menu (`init=/bin/sh panic=0`).
Execute diagnostic collector:
```bash
/scripts/diagnostics/collect_system_report.sh --output /var/log/recovery_report.json
```
