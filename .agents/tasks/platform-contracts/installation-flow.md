# Installation Flow Contract

> **Shared SigmaOS platform contract.** Define the generic, kernel-agnostic install flow shared by Linux and BSD backends. The installer orchestrates these steps; backend adapters implement the kernel-specific portions.

---

## Steps

- 1. Hardware detection (delegate to SigmaOS-hardware) — gather CPU, disk, network, GPU inventory.
- 2. Disk partitioning — create root/boot/swap (and optional /home) partitions; validate layout.
- 3. Filesystem selection — offer ext4/btrfs/xfs (Linux) or UFS/ZFS (BSD); format partitions.
- 4. Base package install — install the SigmaOS-core base set via SigmaOS-package-manager.
- 5. Bootloader installation — install GRUB/systemd-boot (Linux) or boot(8) (BSD); validate boot config.
- 6. Locale/timezone — set locale, timezone, and keyboard layout.
- 7. User account creation — create the primary user with home directory and default shell.
- 8. Default package/desktop profile selection — install the curated desktop defaults (SigmaOS-desktop).
- 9. First-boot preparation — write first-boot marker and hand off to SigmaOS-first-boot.
- 10. Validation — run install-time smoke checks (boot config valid, packages installed, user created).

## Linux Adapter

Uses parted/cryptsetup for partitioning, systemd-boot or GRUB for bootloader, systemd-tmpfiles for initial file creation.

## BSD Adapter

Uses gpart for partitioning, boot(8)/loader.conf for bootloader, /etc/rc.conf for initial config. ZFS-on-root supported.

## Failure Modes and Recovery

Disk full, partition table invalid, bootloader install fails, package download interrupted, locale data missing. Each step logs to sigma-install.log and supports resume from the last completed step.

## Open Questions

Should the installer support dual-boot with existing OS? (planned: yes, with a guided partition editor). Encrypted root on BSD — needs ZFS native encryption or gel(8).
