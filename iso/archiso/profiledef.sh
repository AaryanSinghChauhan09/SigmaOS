#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# profiledef.sh: archiso profile definition for SigmaOS / Omarchy live media
# Inspires by omacom/omarchy-iso

iso_name="sigmaos-omarchy"
iso_label="SIGMAOS_$(date +%Y%m)"
iso_publisher="SigmaOS Architecture <https://github.com/AaryanSinghChauhan09/SigmaOS>"
iso_application="SigmaOS Sovereign Live Desktop"
iso_version="$(date +%Y.%m.%d)"
install_dir="sigmaos"
buildmodes=('iso')
bootmodes=('bios.syslinux.mbr' 'bios.syslinux.eltorito' 'uefi-ia32.systemd-boot.eltorito' 'uefi-x64.systemd-boot.eltorito')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'zstd' '-Xcompression-level' '19')
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/etc/gshadow"]="0:0:400"
  ["/root"]="0:0:700"
  ["/root/.automated_script.sh"]="0:0:755"
  ["/usr/local/bin/choose-mirror"]="0:0:755"
  ["/usr/local/bin/Installation_guide"]="0:0:755"
  ["/usr/local/bin/livecd-sound"]="0:0:755"
)
