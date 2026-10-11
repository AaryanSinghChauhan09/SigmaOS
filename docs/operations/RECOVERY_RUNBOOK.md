# SigmaOS Disaster Recovery & Live VM Runbook

This runbook provides step-by-step procedures for recovering a corrupted SigmaOS system, rescuing unbootable VM or bare-metal instances, repairing interrupted updates, and restoring bootloader/partition state.

---

## 1. Booting into Live Rescue / Recovery Mode

When SigmaOS fails to complete the boot state machine (`SIGMAOS_SHELL_INIT` not reached), boot into the Live ISO or Recovery Environment.

1. Insert Live ISO media or attach `sigmaos.iso` to VM drive.
2. Intercept GRUB / UEFI boot menu and select:
   ```text
   SigmaOS Maintenance & Emergency Recovery Environment
   ```
3. Boot parameters appended automatically: `init=/sbin/recovery_shell rescue=1 debug=1`
4. Access emergency root diagnostic shell (`#`).

---

## 2. Broken Bootloader & GRUB Repair

If the system stops at `grub>` or `SIGMAOS_BOOT_INIT` failure:

1. Identify root and boot partitions in rescue shell:
   ```bash
   sigpkg-disk list
   ```
2. Mount system hierarchy:
   ```bash
   mount /dev/sda2 /mnt
   mount /dev/sda1 /mnt/boot
   mount --bind /dev /mnt/dev
   mount --bind /proc /mnt/proc
   mount --bind /sys /mnt/sys
   ```
3. Re-install Multiboot2 GRUB image:
   ```bash
   chroot /mnt grub-install /dev/sda
   chroot /mnt update-grub
   ```
4. Verify `/boot/grub/grub.cfg` contains valid kernel paths (`/boot/kernel.bin`).

---

## 3. Interrupted Package Upgrade & Btrfs/ZFS Snapshot Rollback

If a system update was interrupted during package extraction:

1. List available system boot snapshots:
   ```bash
   sigpkg-snapshot list
   ```
2. Inspect latest pre-update snapshot (e.g. `snapshot-20261011-01`):
   ```bash
   sigpkg-snapshot diff snapshot-20261011-01
   ```
3. Roll back root filesystem to pre-update checkpoint:
   ```bash
   sigpkg-snapshot rollback snapshot-20261011-01 --force
   ```
4. Sync state and reboot:
   ```bash
   sync && reboot
   ```

---

## 4. Emergency Partition & Filesystem Inspection

If filesystem metadata corruption occurs:

1. Run non-destructive filesystem check on target drive:
   ```bash
   fsck.ext4 -n /dev/sda2
   ```
2. If journal replay or metadata repair is required:
   ```bash
   fsck.ext4 -y /dev/sda2
   ```
3. Verify filesystem integrity and mount to `/mnt` to inspect `/var/log/sigmaos.log`.

---

## 5. Recovery Verification Checklist

Before releasing a system back to production:

- [ ] System mounts `/` and `/boot` without kernel panics.
- [ ] `scripts/ci/qemu_boot_smoke.sh` passes or manual reboot reaches `ZenithShell`.
- [ ] System journal shows zero unhandled kernel panic registers.
- [ ] `sigpkg check` confirms zero corrupted package hashes.
