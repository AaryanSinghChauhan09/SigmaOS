# SigmaOS Configuration Guide

## Table of Contents

1.  [System Configuration](#system-configuration)
2.  [Network Configuration](#network-configuration)
3.  [User Management](#user-management)
4.  [Service Management](#service-management)
5.  [Desktop Configuration](#desktop-configuration)
6.  [Audio Configuration](#audio-configuration)
7.  [Printer Configuration](#printer-configuration)
8.  [Kernel Configuration](#kernel-configuration)

## System Configuration

### Hostname

Set the system hostname:

```bash
# Set hostname
sighostname my-sigmaos-system

# View current hostname
hostname
```

### Locale

Configure system locale:

```bash
# List available locales
siglocale list

# Set locale
siglocale set en_US.UTF-8

# Generate locale
siglocale generate
```

### Timezone

Configure system timezone:

```bash
# List timezones
sigtimezone list

# Set timezone
sigtimezone set America/New_York

# Enable NTP
sigtime ntp enable
```

### Kernel Parameters

Configure kernel parameters:

```bash
# View current parameters
sysctl -a

# Set parameter temporarily
sysctl -w kernel.max_files=65536

# Set parameter permanently
echo "kernel.max_files=65536" >> /etc/sysctl.conf
```

## Network Configuration

### Wired Network

Configure wired network interfaces:

```bash
# Automatic DHCP
nmcli device connect eth0

# Static IP
nmcli connection modify eth0 ipv4.addresses 192.168.1.100/24
nmcli connection modify eth0 ipv4.gateway 192.168.1.1
nmcli connection modify eth0 ipv4.dns "8.8.8.8 8.8.4.4"
```

### Wireless Network

Configure wireless network:

```bash
# Scan for networks
nmcli device wifi list

# Connect to network
nmcli device wifi connect "SSID" password "password"

# Enable auto-connect
nmcli connection modify "SSID" connection.autoconnect yes
```

### Firewall

Configure firewall rules:

```bash
# Enable firewall
sigfirewall enable

# Allow SSH
sigfirewall allow 22/tcp

# Allow HTTP
sigfirewall allow 80/tcp

# List rules
sigfirewall list
```

## User Management

### User Accounts

Manage user accounts:

```bash
# Create user
siguser add username

# Set password
siguser set-password username

# Delete user
siguser delete username

# List users
siguser list
```

### User Groups

Manage user groups:

```bash
# Add user to group
siguser group-add username wheel

# Remove user from group
siguser group-remove username wheel

# List groups
siguser group-list
```

### Sudo Configuration

Configure sudo access:

```bash
# Edit sudoers file
visudo

# Allow wheel group
%wheel ALL=(ALL) ALL

# Allow specific command
username ALL=(ALL) /usr/bin/pacman
```

## Service Management

### Systemd-Style Service Management

Manage services using SigmaInit:

```bash
# Start service
siginit start service-name

# Stop service
siginit stop service-name

# Restart service
siginit restart service-name

# Enable service (start on boot)
siginit enable service-name

# Disable service
siginit disable service-name

# Check service status
siginit status service-name

# List all services
siginit list
```

### Common Services

Essential services to enable:

```bash
# Network service
siginit enable NetworkManager

# Display manager
siginit enable zenith-display-manager

# Audio service
siginit enable sigma-audio

# Bluetooth service
siginit enable sigma-bluetooth
```

## Desktop Configuration

### Display Manager

Configure display manager:

```bash
# Set default display manager
sigset-display-manager zenith

# Configure auto-login
sigdisplay autologin username

# Configure session
sigdisplay session zenith
```

### Window Manager

Configure window manager behavior:

```bash
# Set window mode (tiling/stacking/floating)
sigwm mode tiling

# Configure keybindings
sigwm keybind Mod4+Return "terminal"

# Configure workspaces
sigwm workspace add 1
sigwm workspace add 2
```

### Theme Configuration

Configure system theme:

```bash
# Set GTK theme
sigtheme gtk sigmaos-dark

# Set icon theme
sigtheme icons sigmaos-icons

# Set cursor theme
sigtheme cursor sigmaos-cursor

# Set font
sigfont "DejaVu Sans" 12
```

### Panel Configuration

Configure desktop panel:

```bash
# Add applet
sigpanel add applet clock
sigpanel add applet battery
sigpanel add applet network

# Configure panel position
sigpanel position top

# Configure panel size
sigpanel size 48
```

## Audio Configuration

### Audio Device Configuration

Configure audio devices:

```bash
# List audio devices
sigaudio list

# Set default device
sigaudio default device-id

# Configure volume
sigaudio volume 50

# Mute audio
sigaudio mute
```

### Audio Mixer

Configure audio mixer:

```bash
# Open audio mixer
sigaudio-mixer

# Configure channels
sigaudio-mixer channel Master 80
sigaudio-mixer channel PCM 70
```

### Bluetooth Audio

Configure Bluetooth audio:

```bash
# Pair device
sigbluetooth pair device-mac

# Connect device
sigbluetooth connect device-mac

# Set as audio output
sigbluetooth audio device-mac
```

## Printer Configuration

### CUPS Configuration

Configure printing system:

```bash
# Start CUPS service
siginit start cups

# Enable CUPS on boot
siginit enable cups

# Access web interface
# http://localhost:631
```

### Printer Setup

Add and configure printers:

```bash
# List printers
sigprinter list

# Add printer
sigprinter add "Printer Name" /dev/usb/lp0

# Set default printer
sigprinter default "Printer Name"

# Test print
sigprinter test "Printer Name"
```

## Kernel Configuration

### Custom Kernel Parameters

Configure custom kernel parameters:

```bash
# Edit kernel command line
edit /etc/kernel/cmdline

# Add parameters
quiet splash
```

### Kernel Modules

Load kernel modules:

```bash
# List loaded modules
lsmod

# Load module
modprobe module-name

# Unload module
modprobe -r module-name

# Load module on boot
echo "module-name" >> /etc/modules-load.d/custom.conf
```

### Kernel Updates

Update kernel:

```bash
# Update kernel
sigpkg update kernel

# Rebuild initramfs
sigmkinitramfs

# Update bootloader
sigupdate-bootloader
```

## Additional Configuration

### Environment Variables

Set environment variables:

```bash
# User environment
export EDITOR=vim
export PATH=$PATH:/usr/local/bin

# System-wide environment
echo "export EDITOR=vim" >> /etc/environment
```

### Automatic Startup

Configure applications to start automatically:

```bash
# Add autostart entry
sigautostart add "Application Name" /usr/bin/app

# Remove autostart entry
sigautostart remove "Application Name"

# List autostart entries
sigautostart list
```

### Performance Tuning

Optimize system performance:

```bash
# Set CPU governor
sigcpu governor performance

# Enable zram
sigzram enable

# Configure swappiness
echo "vm.swappiness=10" >> /etc/sysctl.conf
```

## Configuration Files

### System Configuration Files

*   `/etc/sigmaos/config` - Main system configuration
*   `/etc/sigmaos/network` - Network configuration
*   `/etc/sigmaos/users` - User configuration
*   `/etc/sigmaos/services` - Service configuration

### Desktop Configuration Files

*   `~/.config/zenith/` - Zenith desktop configuration
*   `~/.config/sigwm/` - Window manager configuration
*   `~/.config/sigtheme/` - Theme configuration

## Backup Configuration

Backup important configuration files:

```bash
# Backup system configuration
tar -czf sigmaos-config-backup.tar.gz /etc/sigmaos

# Backup user configuration
tar -czf user-config-backup.tar.gz ~/.config

# Restore configuration
tar -xzf sigmaos-config-backup.tar.gz -C /
```

## Additional Resources

*   [Installation Guide](./INSTALLATION)
*   [Package Management Guide](./PACKAGE_MANAGEMENT)
*   [Security Hardening Guide](./SECURITY)
*   [Development Guide](./DEVELOPMENT)
