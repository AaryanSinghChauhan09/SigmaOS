# SigmaOS Operations: Installer & Update Recovery Runbook

## Core Recovery Principle
All destructive operations—such as disk partitioning, filesystem formatting, base system deployment, bootloader installation, and system package updates—must be transactional and non-destructive until user confirmation. In the event of an abort, crash, power outage, or verification failure, the system must cleanly restore or retain existing storage partitions.

---

## 1. Installer Dry-Run Validation
Before applying changes to physical block devices, the installer must simulate the execution plan:
```bash
sigma-installer --dry-run --target /dev/sda --layout /etc/installer/partition_schema.json
```
- Dry-run mode validates GPT table alignment, sector offsets, and filesystem compatibility without writing a single sector to disk.
- Verified in CI via `installer/tests/dry_run.rs`.

---

## 2. Injected Failure and Abort Clean-Up
If an installation step fails:
1. **Unmount active mountpoints** cleanly in reverse order (`/mnt/target/boot`, `/mnt/target/home`, `/mnt/target`).
2. **Purge temporary staging directories** (`/mnt/target/var/tmp`, `/mnt/target/tmp`).
3. **Release block device locks** (`flock` on `/dev/sdX`).
4. **Log failure diagnosis** to `/var/log/installer-failure.json`.
- Verified in CI via `installer/tests/failure_injection.rs`.

---

## 3. Atomic System Updates and Rollback
When updating kernel packages or userspace binaries:
- Root filesystems utilize A/B slotting or copy-on-write subvolumes (e.g., OverlayFS / ZFS / btrfs snapshots).
- The bootloader updates a fallback entry before replacing the active kernel.
- If a new kernel fails to reach the `shell_ready` or `systemd-compat` milestone within 30 seconds, the hardware watchdog triggers a reboot into the previous working slot.
