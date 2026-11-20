#!/usr/bin/env bash
# Runs inside the airootfs chroot after package installation.
# The kernel lands in /usr/lib/modules/<kver>/vmlinuz but /boot/vmlinuz-linux
# is only created by kernel-install, which does not run in the pacstrap chroot.
# mkinitcpio presets require /boot/vmlinuz-linux, so create the symlink and
# (re)generate the initramfs images here.
set -uo pipefail

KVER="$(basename "$(find /usr/lib/modules -mindepth 1 -maxdepth 1 -type d | sort -V | tail -1)")"
[ -n "$KVER" ] || { echo "customize: no kernel found in /usr/lib/modules" >&2; exit 1; }
[ -f "/usr/lib/modules/${KVER}/vmlinuz" ] || { echo "customize: vmlinuz missing for ${KVER}" >&2; exit 1; }

mkdir -p /boot
# Copy the real kernel image — a symlink would dangle after mkarchiso copies
# /boot out of the airootfs onto the ISO staging tree.
cp -f "/usr/lib/modules/${KVER}/vmlinuz" /boot/vmlinuz-linux
# mkinitcpio preset also references the fallback image; regenerate both.
# mkinitcpio exits non-zero on non-fatal warnings (missing optional firmware
# modules, crc32c_intel) — tolerate that, but verify the images exist after.
mkinitcpio -P || echo "customize: mkinitcpio reported warnings" >&2
for img in /boot/initramfs-linux.img /boot/initramfs-linux-fallback.img; do
  [ -s "$img" ] || { echo "customize: missing $img" >&2; exit 1; }
done
ls -l /boot/
