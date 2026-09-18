# General recommendations

This page provides a comprehensive list of post-installation recommendations for SigmaOS systems, covering system administration, desktop environment configuration, package management, and security hardening.

## System administration

### User management

#### Add a user account

Create a new user account:

```bash
sudo useradd -m -G wheel -s /bin/zsh username
sudo passwd username
```

#### Configure sudo

Enable sudo for the wheel group:

```bash
EDITOR=nano visudo
```

Uncomment:
```
%wheel ALL=(ALL) ALL
```

#### Manage groups

Add user to additional groups:

```bash
sudo usermod -aG groupname username
```

Common groups:
- `wheel` - sudo access
- `audio` - audio device access
- `video` - video device access
- `network` - network management
- `storage` - storage device access

### Service management

#### Enable/disable services

Enable a service to start at boot:

```bash
sudo systemctl enable service-name
```

Disable a service:

```bash
sudo systemctl disable service-name
```

#### Start/stop services

Start a service:

```bash
sudo systemctl start service-name
```

Stop a service:

```bash
sudo systemctl stop service-name
```

#### Check service status

Check service status:

```bash
sudo systemctl status service-name
```

List all running services:

```bash
sudo systemctl list-units --type=service --state=running
```

### System updates

#### Update system

Update package lists and upgrade system:

```bash
sudo sigpkg update
sudo sigpkg upgrade
```

#### Automatic updates

Configure automatic updates:

```bash
sudo sigpkg config --set auto-updates daily
```

#### Rollback

Rollback to previous system generation:

```bash
sudo sigpkg rollback
```

List available generations:

```bash
sudo sigpkg list-generations
```

## Package management

### Essential packages

Install essential packages:

```bash
sudo sigpkg install base-devel vim zsh git
```

### Package search

Search for packages:

```bash
sigpkg search keyword
```

### Package information

Get package information:

```bash
sigpkg info package-name
```

### Package removal

Remove a package:

```bash
sudo sigpkg remove package-name
```

Remove package and dependencies:

```bash
sudo sigpkg remove --cascade package-name
```

### Package cleanup

Clean package cache:

```bash
sudo sigpkg clean
```

Remove unused packages:

```bash
sudo sigpkg autoremove
```

## Desktop environment

### Zenith compositor

#### Install Zenith desktop

Install the Zenith desktop environment:

```bash
sudo sigpkg install zenith-desktop
```

#### Configure Zenith

Zenith configuration is stored in `~/.config/zenith/config.toml`:

```toml
[desktop]
theme = "dark"
workspace = "default"

[keybindings]
# Keybindings configuration
```

#### Start Zenith automatically

Enable the display manager:

```bash
sudo systemctl enable zenith-display-manager
sudo systemctl start zenith-display-manager
```

### Display server

#### Wayland configuration

Configure Wayland environment variables in `~/.bash_profile`:

```bash
export WAYLAND_DISPLAY=wayland-1
export MOZ_ENABLE_WAYLAND=1
export QT_QPA_PLATFORM=wayland
```

#### XWayland support

Install XWayland for X11 application compatibility:

```bash
sudo sigpkg install xwayland
```

## Graphics and audio

### Graphics drivers

#### Intel graphics

Install Intel graphics drivers:

```bash
sudo sigpkg install xf86-video-intel intel-media-driver
```

#### AMD graphics

Install AMD graphics drivers:

```bash
sudo sigpkg install xf86-video-amdgpu mesa
```

#### NVIDIA graphics

Install NVIDIA proprietary drivers:

```bash
sudo sigpkg install nvidia nvidia-utils
```

For open-source Nouveau drivers:

```bash
sudo sigpkg install xf86-video-nouveau
```

### Audio configuration

#### PipeWire

Install PipeWire audio server:

```bash
sudo sigpkg install pipewire pipewire-pulse wireplumber
```

Enable PipeWire services:

```bash
systemctl --user enable pipewire pipewire-pulse wireplumber
systemctl --user start pipewire pipewire-pulse wireplumber
```

#### Audio troubleshooting

Test audio output:

```bash
speaker-test -t wav -c 2
```

List audio devices:

```bash
pactl list sinks
pactl list sources
```

## Networking

### Network configuration

#### NetworkManager

Install NetworkManager:

```bash
sudo sigpkg install networkmanager
```

Enable NetworkManager:

```bash
sudo systemctl enable NetworkManager
sudo systemctl start NetworkManager
```

#### Wired connections

Configure wired connection:

```bash
nmcli connection edit "Wired connection 1"
```

#### Wireless connections

Connect to Wi-Fi:

```bash
nmcli device wifi connect SSID password password
```

#### Firewall

Install and configure firewall:

```bash
sudo sigpkg install sigma-firewall
sudo systemctl enable sigma-firewall
sudo systemctl start sigma-firewall
```

### DNS configuration

#### Set DNS servers

Configure DNS in NetworkManager:

```bash
nmcli connection modify "Wired connection 1" ipv4.dns "8.8.8.8 8.8.4.4"
```

#### DNS over HTTPS

Configure DNS over HTTPS:

```bash
sudo sigpkg install doh-client
```

## Input devices

### Keyboard configuration

#### Set keyboard layout

Set keyboard layout in `/etc/vconsole.conf`:

```
KEYMAP=us
```

#### X11 keyboard layout

Configure X11 keyboard layout:

```bash
setxkbmap us
```

### Mouse configuration

#### Mouse acceleration

Configure mouse acceleration:

```bash
xinput set-prop "Device Name" "libinput Accel Speed" 0.5
```

## File systems

### Mount additional drives

#### Identify drives

List available drives:

```bash
lsblk
```

#### Format drive

Format drive as ext4:

```bash
sudo mkfs.ext4 /dev/sdX1
```

#### Mount drive

Create mount point:

```bash
sudo mkdir /mnt/drive
```

Mount drive:

```bash
sudo mount /dev/sdX1 /mnt/drive
```

#### Auto-mount on boot

Add to `/etc/fstab`:

```
/dev/sdX1 /mnt/drive ext4 defaults 0 2
```

### Btrfs subvolumes

#### Create subvolume

Create Btrfs subvolume:

```bash
sudo btrfs subvolume create /mnt/@subvolume
```

#### List subvolumes

List subvolumes:

```bash
sudo btrfs subvolume list /mnt
```

#### Snapshot subvolume

Create snapshot:

```bash
sudo btrfs subvolume snapshot /mnt/@ /mnt/@snapshot
```

## Security

### Security hardening

#### Enable firewall

Enable SigmaOS firewall:

```bash
sudo systemctl enable sigma-firewall
sudo systemctl start sigma-firewall
```

Configure firewall rules:

```bash
sudo sigma-firewall enable service ssh
sudo sigma-firewall enable port 8080
```

#### Secure SSH

Configure SSH security:

```bash
sudo nano /etc/ssh/sshd_config
```

Recommended settings:
```
PermitRootLogin no
PasswordAuthentication no
PubkeyAuthentication yes
```

#### Enable secure boot

Verify secure boot status:

```bash
mokutil --sb-state
```

### User security

#### Password policy

Configure password policy:

```bash
sudo sigpkg install libpam-pwquality
```

#### Two-factor authentication

Enable two-factor authentication:

```bash
sudo sigpkg install libpam-google-authenticator
```

## Performance optimization

### System performance

#### Enable BORE scheduler

Enable BORE scheduler for better interactivity:

```bash
sudo sigpkg install bore-sched
echo "sched_bore=1" | sudo tee /etc/sysctl.d/99-bore.conf
```

#### Memory management

Configure memory management:

```bash
echo "vm.swappiness=10" | sudo tee /etc/sysctl.d/99-swap.conf
echo "vm.vfs_cache_pressure=50" | sudo tee /etc/sysctl.d/99-vfs.conf
```

#### CPU performance

Enable CPU performance governor:

```bash
sudo sigpkg install cpupower
sudo cpupower frequency-set -g performance
```

### Storage performance

#### Enable TRIM

Enable TRIM for SSD:

```bash
sudo systemctl enable fstrim.timer
sudo systemctl start fstrim.timer
```

#### I/O scheduler

Set I/O scheduler:

```bash
echo mq-deadline | sudo tee /sys/block/sdX/queue/scheduler
```

## Backup and recovery

### System backup

#### Timeshift

Install Timeshift for system snapshots:

```bash
sudo sigpkg install timeshift
```

Configure Timeshift:

```bash
sudo timeshift-gtk
```

#### rsync backup

Create rsync backup script:

```bash
#!/bin/bash
rsync -av --delete /home/ /backup/home/
rsync -av --delete /etc/ /backup/etc/
```

### System recovery

#### Recovery mode

Boot into recovery mode from bootloader.

#### Chroot recovery

Chroot into system for recovery:

```bash
mount /dev/sda2 /mnt
mount /dev/sda1 /mnt/boot
arch-chroot /mnt
```

## Aesthetics

### Fonts

#### Install fonts

Install common fonts:

```bash
sudo sigpkg install ttf-dejavu ttf-liberation noto-fonts
```

#### Font configuration

Configure fonts in `~/.config/fontconfig/fonts.conf`:

```xml
<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <alias>
    <family>sans-serif</family>
    <prefer><family>Noto Sans</family></prefer>
  </alias>
</fontconfig>
```

### Themes

#### Install themes

Install GTK themes:

```bash
sudo sigpkg install gtk-theme-adwaita
```

#### Icon themes

Install icon themes:

```bash
sudo sigpkg install papirus-icon-theme
```

## System monitoring

### System monitoring tools

#### Install monitoring tools

Install system monitoring tools:

```bash
sudo sigpkg install htop iotop nethogs
```

#### Process monitoring

Monitor processes:

```bash
htop
```

#### Network monitoring

Monitor network usage:

```bash
nethogs
```

### System logs

#### View logs

View system logs:

```bash
journalctl -xe
```

#### Log rotation

Configure log rotation:

```bash
sudo sigpkg install logrotate
```

## Troubleshooting

### Boot issues

#### Recovery mode

Boot into recovery mode from bootloader.

#### Initramfs issues

Rebuild initramfs:

```bash
sudo mkinitcpio -P
```

### Performance issues

#### High CPU usage

Identify high CPU processes:

```bash
top
htop
```

#### High memory usage

Identify memory usage:

```bash
free -h
ps aux --sort=-%mem | head
```

### Network issues

#### DNS issues

Test DNS resolution:

```bash
nslookup example.com
```

#### Connection issues

Test network connectivity:

```bash
ping -c 4 example.com
traceroute example.com
```

## Additional resources

- [Installation Guide](Installation-Guide) — Installation instructions
- [List of Applications](List-of-Applications) — Common applications
- [System Administration](System-Administration-and-Services) — System management
- [Security & Hardening](Security-Sandboxing-and-Hardening) — Security configuration
- [Performance Tuning](Performance-Tuning-and-Kernel) — Performance optimization

---

**[Installation Guide](Installation-Guide)** | **[List of Applications](List-of-Applications)** | **[Frequently Asked Questions](Frequently-Asked-Questions)**
