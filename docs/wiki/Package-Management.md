# Package Management

SigmaOS uses Arch Linux's package manager (pacman) with a curated package set.

## Package Sources

### Official Arch Repositories
- core
- extra
- community
- multilib

### SigmaOS Mirror
Additional mirror configured in: `iso/archiso/airootfs/etc/pacman.d/sigmaos-mirrorlist`

## Package Set

### Total: 116 Packages

### Base System (11)
base, base-devel, linux, linux-firmware, linux-headers, systemd, systemd-sysvcompat, sudo, which, man-db, man-pages, bash-completion

### Storage (6)
btrfs-progs, e2fsprogs, dosfstools, xfsprogs, ntfs-3g, parted, smartmontools

### Networking (8)
networkmanager, network-manager-applet, nm-connection-editor, wireless_tools, wpa_supplicant, iwd, bluez, bluez-utils, curl, wget, git, openssh, rsync

### Desktop (20)
hyprland, hypridle, hyprlock, hyprpaper, swaybg, waybar, wallust, foot, alacritty, kitty, rofi-wayland, mako, dunst, wlogout, grim, slurp, swappy, wl-clipboard, cliphist, xdg-desktop-portal-hyprland, xdg-desktop-portal-gtk, polkit-gnome, brightnessctl, playerctl

### Audio (6)
pipewire, pipewire-pulse, pipewire-alsa, pipewire-jack, wireplumber, pavucontrol, mpv, imv

### Fonts (5)
ttf-jetbrains-mono, ttf-jetbrains-mono-nerd, noto-fonts, noto-fonts-cjk, noto-fonts-emoji, ttf-font-awesome

### File Manager (9)
thunar, thunar-volman, thunar-archive-plugin, file-roller, p7zip, unzip, zip, tar, gzip, bzip2, xz, zstd

### Tools (9)
btop, htop, fastfetch, tree, fzf, ripgrep, fd, bat, jq

### Development (12)
bash, zsh, fish, starship, neovim, helix, tmux, lazygit, mise, docker, docker-compose, rust, cargo, python, python-pip

### Applications (6)
chromium, obsidian, libreoffice-fresh, kdenlive, obs-studio, steam, retroarch, hyprshot, tpm2-tools, sbctl

## Custom SigmaOS Packages

### Essential Meta-Packages (9)

1. **sigmaos-base** - Minimal Arch base meta-package
2. **sigmaos-base-devel** - Development tools meta-package
3. **sigmaos-desktop-meta** - Desktop environment meta-package
4. **sigmaos-keyring** - PGP keyring
5. **sigmaos-mirrorlist** - Mirror configuration
6. **sigmaos-shell** - Shell configuration with Starship prompt
7. **sigmaos-pacman** - Pacman wrapper
8. **sigmaos-wallust-themes** - Color presets and templates
9. **sigmaos-linux** - Linux kernel meta-package

## Package Management Commands

### Update System
```bash
sudo pacman -Syu
```

### Install Package
```bash
sudo pacman -S <package>
```

### Remove Package
```bash
sudo pacman -R <package>
```

### Search Package
```bash
pacman -Ss <keyword>
```

### Orphan Packages
```bash
sudo pacman -Qtdq
sudo pacman -Rns $(pacman -Qtdq)
```

## AUR Support

The ISO includes yay for AUR package installation (optional):

```bash
yay -S <aur-package>
```

## Package Updates

SigmaOS follows Arch's rolling release model:
- Updates are frequent
- System is always current
- No version numbers in ISO name

## Customization

To add packages to the ISO:
1. Edit `iso/archiso/packages.x86_64`
2. Add package name (one per line)
3. Rebuild ISO

To remove packages:
1. Edit `iso/archiso/packages.x86_64`
2. Remove or comment out package name
3. Rebuild ISO
