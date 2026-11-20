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
# modules) — tolerate that, but verify the images exist after.
mkinitcpio -P || echo "customize: mkinitcpio reported warnings" >&2
for img in /boot/initramfs-linux.img /boot/initramfs-linux-fallback.img; do
  [ -s "$img" ] || { echo "customize: missing $img" >&2; exit 1; }
done

# Verify the initramfs can actually boot the live media. A previous build
# shipped an image whose HOOKS lacked archiso/archiso_loop_mnt and whose
# MODULES lacked storage drivers, producing an ISO that dropped to an
# emergency shell with no block devices. Fail the build instead.
verify_initramfs() {
  local img="$1" cfg listing
  cfg="$(lsinitcpio -c "$img" 2>/dev/null)" || { echo "customize: cannot read config of $img" >&2; return 1; }
  echo "$cfg" | grep -q 'archiso_loop_mnt' || {
    echo "customize: $img missing archiso_loop_mnt hook" >&2; return 1; }
  echo "$cfg" | grep -q 'archiso_pxe_nfs' || {
    echo "customize: $img missing archiso pxe hooks" >&2; return 1; }
  listing="$(lsinitcpio -l "$img" 2>/dev/null)" || { echo "customize: cannot list $img" >&2; return 1; }
  local mod
  for mod in sr_mod ata_piix isofs cdrom; do
    echo "$listing" | grep -q "/$mod\.ko" || {
      echo "customize: $img missing required module $mod" >&2; return 1; }
  done
}
verify_initramfs /boot/initramfs-linux.img || exit 1
verify_initramfs /boot/initramfs-linux-fallback.img || exit 1
echo "customize: initramfs verification passed"

# Live session user: the shipped /etc/passwd defines liveuser, but nothing
# creates its home — without it, tty1 autologin lands in a bare shell and
# .bash_profile (which launches Hyprland) never runs. Populate /home/liveuser
# from /etc/skel exactly like useradd -m would.
if id liveuser >/dev/null 2>&1; then
  mkdir -p /home/liveuser
  cp -a /etc/skel/. /home/liveuser/
  chown -R liveuser:liveuser /home/liveuser
  chmod 750 /home/liveuser
  echo "customize: liveuser home populated from /etc/skel"
else
  echo "customize: WARNING liveuser not in passwd" >&2
fi
ls -l /boot/
