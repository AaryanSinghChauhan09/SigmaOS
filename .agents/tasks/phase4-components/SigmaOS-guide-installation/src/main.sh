#!/usr/bin/env bash
# SigmaOS Guide — Installation Guide CLI scaffold
# Upstream: Arch Linux Installation Guide (https://wiki.archlinux.org/title/Installation_guide)
set -euo pipefail

GUIDE_VERSION="0.5"
GUIDE_NAME="SigmaOS Installation Guide"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

$GUIDE_NAME v$GUIDE_VERSION

Subcommands:
  overview           Print installation overview
  step  <n>          Show step N (1-8)
  checklist          Print pre-installation checklist
  hardware           Show minimum hardware requirements
  partitioning       Guide for disk partitioning layouts
  troubleshoot       Common installation problems and fixes
  list               List all available steps
  version            Print guide version

EOF
}

print_overview() {
    cat <<EOF
SigmaOS Installation Overview
==============================
SigmaOS installs in 8 sequential steps:
  1. Boot from live ISO
  2. Partition disk
  3. Format filesystems
  4. Bootstrap base system
  5. Configure locale, timezone, hostname
  6. Create user account
  7. Install bootloader
  8. Reboot and verify

Run: $(basename "$0") step <1-8> for detail on each step.
EOF
}

print_step() {
    local n="$1"
    case "$n" in
        1) cat <<'EOF'
Step 1 — Boot from Live ISO
============================
• Download the latest SigmaOS ISO from releases.sigmaos.org
• Verify the SHA-256 checksum: sha256sum sigmaos-*.iso
• Write to USB:  dd if=sigmaos-*.iso of=/dev/sdX bs=4M status=progress
• Boot target machine from USB (select in BIOS/UEFI boot menu)
• At the live shell, run: sigmaos-installer install
EOF
        ;;
        2) cat <<'EOF'
Step 2 — Partition Disk
========================
• Recommended GPT layout (UEFI):
    /dev/sdX1   512 MiB   EFI System Partition  (type: EF00)
    /dev/sdX2   4 GiB     Linux swap             (type: 8200)
    /dev/sdX3   remaining Linux root             (type: 8300)
• Tools: gdisk /dev/sdX  or  cfdisk /dev/sdX
• For BIOS/MBR use fdisk; skip the EFI partition.
EOF
        ;;
        3) cat <<'EOF'
Step 3 — Format Filesystems
=============================
• mkfs.fat -F32 /dev/sdX1   (EFI)
• mkswap /dev/sdX2 && swapon /dev/sdX2
• mkfs.ext4 /dev/sdX3       (or btrfs: mkfs.btrfs /dev/sdX3)
EOF
        ;;
        4) cat <<'EOF'
Step 4 — Bootstrap Base System
================================
• mount /dev/sdX3 /mnt
• mount --mkdir /dev/sdX1 /mnt/boot/efi
• sigmaos-pkg bootstrap --root /mnt --profile desktop
EOF
        ;;
        5) cat <<'EOF'
Step 5 — Configure Locale, Timezone, Hostname
===============================================
• arch-chroot /mnt
• echo "LANG=en_US.UTF-8" > /etc/locale.conf
• ln -sf /usr/share/zoneinfo/Region/City /etc/localtime
• echo "myhostname" > /etc/hostname
• locale-gen
EOF
        ;;
        6) cat <<'EOF'
Step 6 — Create User Account
==============================
• useradd -m -G wheel -s /bin/bash alice
• passwd alice
• visudo  — uncomment  %wheel ALL=(ALL:ALL) ALL
EOF
        ;;
        7) cat <<'EOF'
Step 7 — Install Bootloader
=============================
• UEFI:  bootctl install --esp-path=/boot/efi
         sigmaos-boot-entry --generate
• BIOS:  grub-install /dev/sdX
         grub-mkconfig -o /boot/grub/grub.cfg
EOF
        ;;
        8) cat <<'EOF'
Step 8 — Reboot and Verify
============================
• exit   (leave chroot)
• umount -R /mnt
• reboot
• After login: sigmaos-core --self-check
EOF
        ;;
        *) echo "Unknown step: $n  (valid: 1-8)" >&2; exit 1 ;;
    esac
}

print_checklist() {
    cat <<'EOF'
Pre-Installation Checklist
===========================
[ ] Backed up all data on target disk
[ ] Verified ISO SHA-256 checksum
[ ] Target machine has >= 4 GiB RAM
[ ] Target disk has >= 20 GiB free space
[ ] Network available (for package bootstrap)
[ ] UEFI Secure Boot setting noted (disable if using BIOS mode)
[ ] USB drive >= 2 GiB capacity
EOF
}

print_hardware() {
    cat <<'EOF'
Minimum Hardware Requirements
===============================
CPU   : x86_64 (64-bit), 2+ cores
RAM   : 2 GiB minimum, 4 GiB recommended
Disk  : 20 GiB (root) + 512 MiB (EFI)
GPU   : Any with DRM/KMS (VESA fallback available)
Network: Required for package bootstrap
EOF
}

print_partitioning() {
    cat <<'EOF'
Partitioning Guide
===================
UEFI systems (recommended):
  Partition 1: 512 MiB  vfat   /boot/efi   EFI System
  Partition 2: 4 GiB    swap               Linux swap
  Partition 3: rest     ext4   /            Linux filesystem

BIOS/legacy systems:
  Partition 1: 1 MiB    -      BIOS boot   (no fs, required for GRUB)
  Partition 2: 4 GiB    swap               Linux swap
  Partition 3: rest     ext4   /            Linux filesystem

Encrypted root (LUKS):
  Run: cryptsetup luksFormat /dev/sdX3
       cryptsetup open /dev/sdX3 cryptroot
  Then format /dev/mapper/cryptroot as ext4/btrfs.
EOF
}

print_troubleshoot() {
    cat <<'EOF'
Common Installation Problems
==============================
Boot loop after install:
  → Check bootloader: bootctl status  (UEFI)  or  grub-install (BIOS)
  → Verify /boot/efi is mounted during install
  → Check sigmaos-boot-entry --verify

Package bootstrap fails:
  → Verify network: ip link; ping 8.8.8.8
  → Check /etc/sigmaos/repos.d/ for correct repo URLs
  → Run: sigmaos-repo --sync --force

Display not working:
  → Boot with nomodeset kernel parameter
  → Check SigmaOS-drivers GPU detection: sigmaos-drivers --diagnostics

Cannot create user:
  → Ensure you are in chroot: arch-chroot /mnt
EOF
}

list_steps() {
    cat <<'EOF'
Available installation steps:
  1  Boot from Live ISO
  2  Partition Disk
  3  Format Filesystems
  4  Bootstrap Base System
  5  Configure Locale/Timezone/Hostname
  6  Create User Account
  7  Install Bootloader
  8  Reboot and Verify
EOF
}

# ── main ────────────────────────────────────────────────────────────────────
case "${1:-}" in
    overview)    print_overview ;;
    step)        print_step "${2:?'step number required (1-8)'}" ;;
    checklist)   print_checklist ;;
    hardware)    print_hardware ;;
    partitioning) print_partitioning ;;
    troubleshoot) print_troubleshoot ;;
    list)        list_steps ;;
    version)     echo "$GUIDE_NAME v$GUIDE_VERSION" ;;
    -h|--help|help|"") usage ;;
    *) echo "Unknown subcommand: $1" >&2; usage; exit 1 ;;
esac
