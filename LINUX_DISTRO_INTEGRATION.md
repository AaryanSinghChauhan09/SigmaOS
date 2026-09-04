# Linux Distribution Integration Guide

## Overview

SigmaOS incorporates the best features and innovations from major Linux distributions, BSD systems, and other operating systems. This guide documents how these features are integrated and how users can leverage them.

## Integrated Distribution Features

### NixOS-Inspired Reproducible Builds

**Module**: `src/compatibility/nixos_reproducible.rs`

**Features**:
- Content-addressable package storage
- Deterministic build environments
- Atomic package operations
- Rollback capabilities
- Profile management

**Usage**:
```rust
use sigmaos::compatibility::nixos_reproducible::*;

let mut store = NixLikeStore::new("/sigma/store");
let derivation = store.create_derivation(
    "my-package",
    "1.0.0", 
    inputs,
    "make && make install"
)?;
let package_path = store.build_package(&derivation)?;
```

**Benefits**:
- Guaranteed reproducible builds
- No dependency hell
- Easy system rollbacks
- Multi-user package management

### Gentoo-Inspired USE Flags

**Module**: `src/compatibility/gentoo_useflags.rs`

**Features**:
- Fine-grained feature control
- Compile-time optimizations
- Hardware-specific builds
- Profile-based configurations

**Usage**:
```bash
# Enable USE flags globally
echo 'ssl ipv6 hardened' > /etc/sigma/use.flags

# Package-specific flags
echo 'firefox_USE="gtk3 wayland -pulseaudio"' >> /etc/sigma/package.use

# Apply profile
sigma-profile set desktop-gaming
```

**Available Profiles**:
- `desktop`: Full desktop environment
- `server`: Minimal server configuration
- `embedded`: Resource-constrained systems
- `gaming`: Optimized for gaming performance

### Arch Linux-Inspired AUR

**Module**: `src/compatibility/arch_aur.rs`

**Features**:
- Community package repository
- User-submitted build scripts
- Voting system for packages
- Automated building from source

**Usage**:
```bash
# Search community packages
sigma-aur search "media player"

# Build and install from AUR
sigma-aur build vlc-git

# Submit package to community repo
sigma-aur submit my-package-build
```

### FreeBSD-Inspired Jails

**Module**: `src/compatibility/freebsd_jails.rs`

**Features**:
- OS-level virtualization
- Process isolation
- Network virtualization
- Resource limiting
- Security boundaries

**Usage**:
```bash
# Create a jail
sigma-jail create web-server \
  --hostname webserver \
  --ip 192.168.1.100 \
  --root /sigma/jails/web

# Start the jail
sigma-jail start web-server

# Execute commands in jail
sigma-jail exec web-server "systemctl start nginx"

# List all jails
sigma-jail list
```

### Ubuntu/Debian APT Integration

**Module**: `src/compatibility/ubuntu_apt.rs`

**Features**:
- APT-compatible package management
- PPA (Personal Package Archive) support
- Dependency resolution
- Automatic updates

**Usage**:
```bash
# Add PPA repository
sigma-apt add-apt-repository ppa:deadsnakes/ppa

# Update package lists
sigma-apt update

# Install packages
sigma-apt install python3.11

# Search packages
sigma-apt search "video editor"

# Upgrade system
sigma-apt upgrade
```

## Advanced Integration Features

### Multi-Distribution Package Support

SigmaOS can handle packages from multiple distributions simultaneously:

```bash
# Install from different package formats
sigpkg install firefox.deb          # Debian package
sigpkg install vlc.rpm              # RPM package  
sigpkg install gimp.flatpak         # Flatpak
sigpkg install code.appimage        # AppImage
sigpkg install discord-aur          # AUR package
sigpkg install --use-flags="gtk3 wayland" firefox-gentoo  # Gentoo-style
```

### Distribution-Specific Compatibility Layers

#### APT Compatibility Layer
```bash
# Standard APT commands work
apt update
apt install vim
apt search editor
dpkg -l
```

#### DNF/YUM Compatibility Layer
```bash
# Fedora/RHEL package management
dnf install firefox
dnf search browser
rpm -qa
```

#### Pacman Compatibility Layer
```bash
# Arch Linux package management  
pacman -S firefox
pacman -Ss browser
makepkg -si
```

#### Portage Compatibility Layer
```bash
# Gentoo package management
emerge firefox
emerge --search browser
USE="wayland -pulseaudio" emerge firefox
```

### Configuration Management Integration

#### systemd Integration (Ubuntu/Debian/Fedora style)
```bash
systemctl enable nginx
systemctl start postgresql
journalctl -u ssh
```

#### OpenRC Integration (Gentoo/Alpine style)
```bash
rc-service nginx start
rc-update add postgresql default
rc-status
```

#### BSD-style rc.conf
```bash
# /etc/rc.conf
nginx_enable="YES"
postgresql_enable="YES"
```

### Filesystem Integration

#### Btrfs with Snapshots (openSUSE style)
```bash
# Automatic snapshots before updates
sigma-update --with-snapshot
snapper list
snapper rollback 42
```

#### ZFS Support (FreeBSD style)
```bash
# ZFS filesystem management
zpool create sigma-pool /dev/sda
zfs create sigma-pool/home
zfs snapshot sigma-pool/home@backup
```

#### APFS Integration (macOS compatibility)
```bash
# APFS filesystem support
mount -t apfs /dev/sdb1 /mnt/macos-data
```

## Desktop Environment Integration

### GNOME Integration (Ubuntu/Fedora style)
- Native GNOME Shell support
- GTK application integration
- GNOME Online Accounts
- Software Center integration

### KDE Plasma Integration (openSUSE/Kubuntu style)  
- Full Plasma desktop environment
- Qt application support
- KDE Connect integration
- Discover package manager

### Xfce Integration (Xubuntu style)
- Lightweight desktop environment
- Panel customization
- Thunar file manager
- Power management integration

### Custom Desktop Environments
- **Zenith Desktop**: SigmaOS native environment
- **Moksha**: Enlightenment fork (Bodhi Linux inspired)
- **Pantheon**: elementary OS style interface

## Security Model Integration

### AppArmor (Ubuntu/SUSE style)
```bash
# Mandatory Access Control
aa-enforce /usr/bin/firefox
aa-status
```

### SELinux (Fedora/RHEL style)
```bash
# Security Enhanced Linux
setenforce 1
setsebool httpd_can_network_connect on
```

### grsecurity (Hardened systems)
```bash
# Kernel hardening
gradm -E  # Enable RBAC
paxctl -c /usr/bin/binary  # Control executable features
```

### Qubes-style Isolation
```bash
# VM-based isolation
sigma-qube create work-qube
sigma-qube start personal-qube
```

## Network Configuration Integration

### NetworkManager (Most distributions)
```bash
nmcli device wifi connect "MyNetwork" password "password"
nmcli connection show
```

### systemd-networkd (Arch/systemd style)
```bash
networkctl status
networkctl up eth0
```

### ifconfig/netctl (Traditional Unix/BSD)
```bash
ifconfig eth0 192.168.1.100/24 up
netctl start ethernet-static
```

### Netplan (Ubuntu 18+ style)
```yaml
# /etc/netplan/01-network-manager-all.yaml
network:
  version: 2
  renderer: networkd
  ethernets:
    eth0:
      dhcp4: true
```

## Package Repository Integration

### Multiple Repositories
SigmaOS can access repositories from multiple distributions:

- **Ubuntu/Debian**: `archive.ubuntu.com`, `deb.debian.org`
- **Fedora**: `download.fedoraproject.org`
- **Arch Linux**: `mirror.archlinux.org`
- **openSUSE**: `download.opensuse.org`
- **Alpine**: `dl-cdn.alpinelinux.org`
- **Gentoo**: `distfiles.gentoo.org`

### Repository Configuration
```toml
# /etc/sigma/repositories.toml
[ubuntu]
enabled = true
url = "http://archive.ubuntu.com/ubuntu"
components = ["main", "universe", "multiverse"]
key = "3B4FE6ACC0B21F32"

[arch]
enabled = true
url = "https://mirror.archlinux.org"
architecture = "x86_64"

[fedora]
enabled = false
url = "https://download.fedoraproject.org"
release = "37"
```

## Migration Tools

### From Ubuntu/Debian
```bash
sigma-migrate --from ubuntu \
  --import-packages \
  --import-configs \
  --import-users
```

### From Arch Linux
```bash
sigma-migrate --from arch \
  --import-aur-packages \
  --import-pacman-configs
```

### From Fedora
```bash
sigma-migrate --from fedora \
  --import-rpm-packages \
  --import-dnf-configs
```

### From Gentoo
```bash
sigma-migrate --from gentoo \
  --import-use-flags \
  --import-portage-configs
```

## Best Practices

### Package Management
1. **Use native SigPkg when possible** for best integration
2. **Enable reproducible builds** for critical systems
3. **Use USE flags** for performance-critical applications
4. **Regular snapshots** before major changes

### Security
1. **Enable all security layers** (AppArmor, sandboxing, etc.)
2. **Use jails** for untrusted applications
3. **Regular security updates** from all enabled repositories
4. **Monitor CVEs** across all package sources

### Performance  
1. **Choose appropriate profile** for your use case
2. **Compile with USE flags** for optimization
3. **Use local mirrors** for faster downloads
4. **Enable parallel compilation** for source builds

### Maintenance
1. **Regular cleanup** of package caches
2. **Monitor disk usage** of multiple package systems
3. **Update repository metadata** regularly
4. **Backup configuration** before major changes

## Troubleshooting

### Package Conflicts
```bash
# Resolve conflicts between package systems
sigma-resolve-conflicts --interactive

# Force specific package source
sigpkg install --source=ubuntu firefox
sigpkg install --source=gentoo --use-flags="wayland" firefox
```

### Repository Issues  
```bash
# Refresh all repository metadata
sigma-repo refresh --all

# Check repository status
sigma-repo status

# Disable problematic repository
sigma-repo disable ubuntu-proposed
```

### Migration Problems
```bash
# Check migration status
sigma-migrate status

# Rollback partial migration
sigma-migrate rollback

# Manual package mapping
sigma-migrate map ubuntu-package sigma-package
```

This integration system provides SigmaOS users with unprecedented flexibility while maintaining system stability and security. Users can leverage the best features from any Linux distribution while enjoying the benefits of SigmaOS's advanced architecture.