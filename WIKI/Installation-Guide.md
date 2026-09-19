# Installation Guide

This document is a guide for installing SigmaOS on supported hardware. The current version is aimed at experienced users who are comfortable with command-line operations and system configuration.

## Pre-installation

### System requirements

- **Architecture**: x86_64
- **Memory**: Minimum 2 GB RAM (4 GB recommended)
- **Storage**: Minimum 20 GB disk space
- **Network**: Internet connection for package installation
- **UEFI**: UEFI firmware with Secure Boot support (recommended)

### Acquire installation image

Download the latest SigmaOS installation image from the official repository:

```bash
wget https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/latest/sigmaos-installer.iso
```

Verify the image signature:

```bash
wget https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/latest/sigmaos-installer.iso.sig
gpg --verify sigmaos-installer.iso.sig sigmaos-installer.iso
```

### Boot the live system

1. Create a bootable USB drive:
   ```bash
   sudo dd if=sigmaos-installer.iso of=/dev/sdX bs=4M status=progress
   sync
   ```
   Replace `/dev/sdX` with your USB device.

2. Boot from the USB drive and select the SigmaOS installer in the boot menu.

3. You will be logged in as root and presented with a Zsh shell.

## Configure the live system

### Set the console keyboard layout

The default console keymap is US. Available layouts can be listed with:

```bash
localectl list-keymaps
```

To set your keyboard layout, use:

```bash
loadkeys de-latin1
```

### Connect to the internet

To ensure your network interface is listed and operational, use:

```bash
ip link
```

For Wi-Fi, use `iwctl`:

```bash
iwctl
```

Then within the iwd prompt:
```
device list
station wlan0 scan
station wlan0 get-networks
station wlan0 connect YOUR_NETWORK_NAME
exit
```

Verify connection:
```bash
ping archlinux.org
```

### Update the system clock

Use `timedatectl` to ensure the system clock is accurate:

```bash
timedatectl set-ntp true
```

Check the service status:
```bash
timedatectl status
```

### Partition the disks

Using `fdisk` or `cfdisk`, partition your disk as needed. A common setup for UEFI systems is:

```bash
fdisk /dev/sda
```

Within fdisk:
1. Create a new partition for EFI System (512M, type EFI System)
2. Create a partition for root (remaining space, type Linux filesystem)

Example layout:
- `/dev/sda1` - EFI System Partition (512M)
- `/dev/sda2` - Root partition (remaining space)

### Format the partitions

Format the partitions:

```bash
# Format EFI partition as FAT32
mkfs.fat -F32 /dev/sda1

# Format root partition as ext4
mkfs.ext4 /dev/sda2
```

### Mount the file systems

Mount the root partition:

```bash
mount /dev/sda2 /mnt
```

Create and mount the EFI partition:

```bash
mkdir -p /mnt/boot
mount /dev/sda1 /mnt/boot
```

## Installation

### Install essential packages

Use the SigmaPkg script to install the base system:

```bash
sigpkg-install /mnt base base-devel linux-firmware
```

This will install the essential packages for a minimal system.

### Configure the system

#### Generate fstab

Generate an fstab file:

```bash
genfstab -U /mnt >> /mnt/etc/fstab
```

Check the resulting `/mnt/etc/fstab` file:

```bash
cat /mnt/etc/fstab
```

#### Chroot into the new system

Change root into the new system:

```bash
arch-chroot /mnt
```

#### Set timezone

Set your timezone:

```bash
ln -sf /usr/share/zoneinfo/Region/City /etc/localtime
```

Example:
```bash
ln -sf /usr/share/zoneinfo/America/New_York /etc/localtime
```

Run `hwclock` to generate `/etc/adjtime`:

```bash
hwclock --systohc
```

#### Localization

Edit `/etc/locale.gen` and uncomment `en_US.UTF-8 UTF-8` and other needed locales.

Generate the locales:

```bash
locale-gen
```

Create the locale.conf file:

```bash
echo "LANG=en_US.UTF-8" > /etc/locale.conf
```

#### Network configuration

Create the hostname file:

```bash
echo "myhostname" > /etc/hostname
```

Add matching entries to `/etc/hosts`:

```bash
echo "127.0.0.1   localhost" >> /etc/hosts
echo "::1         localhost" >> /etc/hosts
echo "127.0.1.1   myhostname.localdomain myhostname" >> /etc/hosts
```

#### Configure initramfs

Create the initial ramdisk:

```bash
mkinitcpio -P
```

#### Set root password

Set the root password:

```bash
passwd
```

#### Boot loader installation

Install the bootloader (systemd-boot):

```bash
bootctl install
```

Create a bootloader entry:

```bash
nano /boot/loader/entries/sigmaos.conf
```

Add the following content:
```
title   SigmaOS
linux   /vmlinuz-linux
initrd  /initramfs-linux.img
options root=PARTUUID=PART_UUID_OF_ROOT_PARTITION rw
```

Replace `PART_UUID_OF_ROOT_PARTITION` with the actual PARTUUID of your root partition (use `blkid` to find it).

#### User management

Create a regular user:

```bash
useradd -m -G wheel -s /bin/zsh username
```

Set the user password:

```bash
passwd username
```

Enable sudo for the wheel group:

```bash
EDITOR=nano visudo
```

Uncomment the line:
```
%wheel ALL=(ALL) ALL
```

## Reboot

Exit the chroot environment:

```bash
exit
```

Unmount all partitions:

```bash
umount -R /mnt
```

Reboot the system:

```bash
reboot
```

Remove the installation media and boot into your new SigmaOS system.

## Post-installation

### General recommendations

After installation, see [General Recommendations](General-Recommendations) for system optimization and additional software installation.

### Package management

Update the system:

```bash
sudo sigpkg update
sudo sigpkg upgrade
```

Install additional packages:

```bash
sudo sigpkg install package-name
```

### Desktop environment

Install the Zenith desktop environment:

```bash
sudo sigpkg install zenith-desktop
```

Enable the display manager:

```bash
sudo systemctl enable zenith-display-manager
sudo systemctl start zenith-display-manager
```

## Troubleshooting

### Boot issues

If the system doesn't boot:
1. Check the bootloader configuration
2. Verify the kernel and initramfs files exist
3. Check the root partition UUID
4. Review boot messages for errors

### Network issues

If network doesn't work:
1. Check network interface status: `ip link`
2. Verify driver is loaded: `sigconfig drivers list`
3. Check configuration files in `/etc/systemd/network/`
4. Review logs: `journalctl -xe`

### Package installation issues

If package installation fails:
1. Update package lists: `sudo sigpkg update`
2. Check network connectivity
3. Verify repository availability
4. Check disk space: `df -h`

## Additional resources

- [General Recommendations](General-Recommendations) — Post-installation guide
- [List of Applications](List-of-Applications) — Common applications
- [System Administration](System-Administration-and-Services) — System management
- [Help:Reading](Help-Reading) — Understanding documentation

---

**[About SigmaOS](About-SigmaOS)** | **[General Recommendations](General-Recommendations)** | **[Frequently Asked Questions](Frequently-Asked-Questions)**
