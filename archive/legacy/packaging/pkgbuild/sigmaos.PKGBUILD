# SigmaOS package meta-definition
# Installs the complete SigmaOS desktop system.
# Modeled on Omarchy's packaging approach.

pkgname=sigmaos
pkgver=0.1.0
pkgrel=1
pkgdesc="SigmaOS - Sovereign, AI-Native Desktop Operating System"
arch=('x86_64')
url="https://github.com/AaryanSinghChauhan09/SigmaOS"
license=('MIT OR GPL-2.0')
depends=(
  'base' 'base-devel' 'linux-firmware' 'systemd'
  'grub' 'efibootmgr' 'dosfstools' 'btrfs-progs' 'zram-generator'
  'limine' 'limine-mkinitcpio-hook'
  'wayland' 'wayland-utils' 'xorg-server-xwayland' 'uwsm' 'seatd'
  'hyprland' 'hyprlock' 'hypridle' 'swaylock'
  'sddm'
  'plymouth'
  'foot' 'ghostty'
  'wofi'
  'waybar'
  'mako'
  'pipewire' 'pipewire-pulse' 'pipewire-alsa' 'wireplumber'
  'alsa-utils' 'pamixer' 'pavucontrol'
  'libinput' 'xdotool' 'ydotool'
  'grim' 'slurp' 'wf-recorder' 'wl-clipboard'
  'nautilus' 'gvfs' 'gvfs-mtp' 'gvfs-nfs'
  'chromium' 'firefox'
  'micro' 'helix'
  'fastfetch' 'btop' 'brightnessctl' 'power-profiles-daemon'
  'gum' 'fzf' 'git' 'curl' 'wget' 'unzip' 'zip' 'tar' 'rsync'
  'bash' 'bash-completion' 'zsh'
  'noto-fonts' 'noto-fonts-emoji' 'ttf-jetbrains-mono-nerd'
  'gtk4' 'gtk3' 'qt5-wayland' 'qt6-wayland'
  'adwaita-icon-theme' 'papirus-icon-theme' 'cursor-themes'
  'networkmanager' 'network-manager-applet'
  'bluez' 'bluez-utils' 'blueman'
  'cups' 'cups-pk-helper'
  'polkit-kde-agent' 'gnome-keyring'
  'tpm2-tools' 'tpm2-tss'
  'speakup' 'espeakup' 'espeak-ng'
  'cloud-init'
  'clonezilla' 'drbl' 'testdisk'
  'sdparm' 'openbsd-netcat' 'xorg-xprop'
  'archinstall' 'arch-install-scripts'
  'sigmaos-keyring' 'sigmaos-settings'
)

package() {
  # Install etc configuration
  cp -a etc/* "${pkgdir}/etc/"
  
  # Install default desktop configuration
  mkdir -p "${pkgdir}/usr/share/sigmaos/default"
  cp -a default/* "${pkgdir}/usr/share/sigmaos/default/"
  
  # Install shell (QML)
  mkdir -p "${pkgdir}/usr/share/sigmaos/shell"
  cp -a shell/* "${pkgdir}/usr/share/sigmaos/shell/"
  
  # Install runtime binaries
  mkdir -p "${pkgdir}/usr/bin"
  cp -a bin/* "${pkgdir}/usr/bin/"
  
  # Install Plymouth theme
  mkdir -p "${pkgdir}/usr/share/plymouth/themes/sigmaos"
  cp -a usr/share/plymouth/themes/sigmaos/* "${pkgdir}/usr/share/plymouth/themes/sigmaos/"
  
  # Install bootloader config
  mkdir -p "${pkgdir}/boot/limine"
  cp -a boot/limine/* "${pkgdir}/boot/limine/"
  
  # Install /etc/skel dotfiles
  cp -a etc/skel/. "${pkgdir}/etc/skel/"
  
  # Install systemd user services
  mkdir -p "${pkgdir}/etc/systemd/user"
  cp -a etc/systemd/user/* "${pkgdir}/etc/systemd/user/"
  
  # Install packages list
  mkdir -p "${pkgdir}/usr/share/sigmaos/install/packages"
  cp -a install/packages/* "${pkgdir}/usr/share/sigmaos/install/packages/"
  
  # Install install scripts
  mkdir -p "${pkgdir}/usr/share/sigmaos/install"
  cp -a install/* "${pkgdir}/usr/share/sigmaos/install/"
  
  # Install PKGBUILD definitions
  mkdir -p "${pkgdir}/usr/share/sigmaos/pkgbuild"
  cp -a pkgbuild/* "${pkgdir}/usr/share/sigmaos/pkgbuild/"
}
