# Installer Guide

SigmaOS uses `sigmaos-install` for system installation with built-in safety features.

## Location

The installer is at: `iso/archiso/airootfs/usr/local/bin/sigmaos-install`

## Safety Features

### Default: Dry-Run Mode
The installer defaults to `DRY_RUN=1`, which simulates installation without writing to disk.

### Active Root Guard
Automatically detects the host's root disk and refuses to install to it.

### Mount Protection
Aborts if any partition on the target device is mounted.

### Explicit Confirmation
Destructive installation requires both:
- `--destructive` flag
- `--confirm-disk <DEVICE>` flag

## Usage

### Dry-Run (Safe)
```bash
sudo sigmaos-install /dev/vda
# Simulates installation, writes nothing
```

### Destructive Install (VM Only)
```bash
sudo sigmaos-install --destructive --confirm-disk /dev/vda /dev/vda
# Installs to /dev/vda (ONLY in VMs)
```

## Installation Process

1. **Partitioning** (GPT table)
   - EFI partition: 1024M, FAT32
   - Root partition: Remaining space, Btrfs

2. **Formatting**
   - EFI: mkfs.vfat (FAT32)
   - Root: mkfs.btrfs with ZSTD compression

3. **Btrfs Subvolumes**
   - @ (root filesystem)
   - @home (user data)
   - @snapshots (system snapshots)
   - @var_log (logs)

4. **Package Installation**
   - Bootstraps with pacstrap
   - Installs base system, desktop, networking, audio
   - 40+ packages including Hyprland, Waybar, PipeWire

5. **System Configuration**
   - Generates fstab
   - Enables NetworkManager and Bluetooth
   - Configures hostname and locale

## Post-Installation

After installation completes:

```bash
# Chroot into new system
arch-chroot /mnt/target

# Set root password
passwd

# Create user
useradd -m -G wheel -s /bin/bash sigma
passwd sigma

# Configure sudo
EDITOR=nano visudo
# Uncomment: %wheel ALL=(ALL) ALL

# Install bootloader
bootctl install

# Configure locale
echo 'en_US.UTF-8 UTF-8' > /etc/locale.gen
locale-gen

# Set hostname
echo 'sigmaos' > /etc/hostname

# Exit chroot
exit

# Reboot
reboot
```

## Troubleshooting

### "Target disk hosts active OS"
The installer detected you're trying to install to your host disk. Use a VM.

### "Partitions are mounted"
Unmount partitions on target device first:
```bash
sudo umount /dev/vda*
```

### Bootloader installation failed
Ensure EFI partition is mounted and system is in UEFI mode.

### Network not working
Enable NetworkManager:
```bash
systemctl enable NetworkManager
systemctl start NetworkManager
```
