# Build SigmaOS ISO on Arch Linux VM

## Quick Start

### Option 1: Use Existing Arch Linux System

If you have access to an Arch Linux system or VM:

```bash
# Install archiso
sudo pacman -S archiso

# Clone SigmaOS
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build ISO
sudo scripts/build_archiso.sh --native
```

### Option 2: Create Arch Linux VM

#### Using QEMU (on Linux Mint)

```bash
# Download Arch Linux ISO
wget https://geo.mirror.pkgbuild.com/iso/latest/archlinux-x86_64.iso

# Create VM disk
qemu-img create -f qcow2 /tmp/arch_vm.qcow2 32G

# Boot Arch ISO
qemu-system-x86_64 \
  -m 4096 \
  -enable-kvm \
  -drive file=/tmp/arch_vm.qcow2,format=qcow2,if=virtio \
  -cdrom archlinux-x86_64.iso \
  -boot d
```

#### Using VirtualBox

```bash
# Download Arch Linux ISO
wget https://geo.mirror.pkgbuild.com/iso/latest/archlinux-x86_64.iso

# Create VM in VirtualBox GUI:
# - New VM: 4GB RAM, 32GB disk
# - Attach archlinux-x86_64.iso as optical drive
# - Enable EFI
# - Start VM
```

#### Using Virtual Machine Manager (virt-manager)

```bash
# Install virt-manager on Linux Mint
sudo apt-get install virt-manager virtinst qemu-kvm libvirt-daemon-system libvirt-clients bridge-utils

# Download Arch Linux ISO
wget https://geo.mirror.pkgbuild.com/iso/latest/archlinux-x86_64.iso

# Create VM via virt-manager GUI or command line
virt-install \
  --name arch-build \
  --ram 4096 \
  --vcpus 2 \
  --disk path=/tmp/arch_build.qcow2,size=32 \
  --cdrom archlinux-x86_64.iso \
  --os-variant archlinux \
  --graphics spice
```

## Arch Linux Installation Summary

Once booted into Arch ISO:

```bash
# Partition disk
cfdisk /dev/vda
# Create: 512M EFI (type EF00), rest as Linux (type 8300)

# Format
mkfs.vfat -F32 /dev/vda1
mkfs.ext4 /dev/vda2

# Mount
mount /dev/vda2 /mnt
mkdir /mnt/boot
mount /dev/vda1 /mnt/boot

# Install base system
pacstrap /mnt base base-devel linux linux-firmware sudo git

# Generate fstab
genfstab -U /mnt >> /mnt/etc/fstab

# Chroot
arch-chroot /mnt

# Set timezone
ln -sf /usr/share/zoneinfo/UTC /etc/localtime
hwclock --systohc

# Set locale
echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
locale-gen
echo "LANG=en_US.UTF-8" > /etc/locale.conf

# Set hostname
echo "arch-build" > /etc/hostname

# Set root password
passwd

# Create user
useradd -m -G wheel -s /bin/bash builder
passwd builder
echo "%wheel ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/wheel

# Install network (if needed)
pacman -S networkmanager
systemctl enable NetworkManager

# Exit chroot
exit

# Unmount
umount -R /mnt

# Reboot
reboot
```

## Build SigmaOS ISO on Arch VM

After installing Arch and logging in as `builder`:

```bash
# Install archiso
sudo pacman -S archiso

# Clone SigmaOS
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Validate profile (optional)
scripts/build_archiso.sh --validate-only

# Build ISO
sudo scripts/build_archiso.sh --native

# The ISO will be in: out/sigmaos-desktop-YYYYMMDD-x86_64.iso
```

## Transfer ISO to Host

From Arch VM, transfer the ISO to your Linux Mint host:

```bash
# Using scp (if SSH is enabled on host)
scp out/sigmaos-desktop-*.iso aaryansinghchauhan@HOST_IP:/home/aaryansinghchauhan/

# Or use shared folder if using VirtualBox
# Or use virt-manager shared folder
```

## Test ISO in QEMU

On Linux Mint host:

```bash
# Create test disk
qemu-img create -f qcow2 /tmp/sigmaos_test.qcow2 32G

# Boot ISO
qemu-system-x86_64 \
  -m 4096 \
  -enable-kvm \
  -drive file=/tmp/sigmaos_test.qcow2,format=qcow2,if=virtio \
  -cdrom sigmaos-desktop-YYYYMMDD-x86_64.iso \
  -boot d
```

## Build Time Estimate

- Arch Linux installation: 10-15 minutes
- SigmaOS ISO build: 30-60 minutes (depending on internet speed and system resources)

## Troubleshooting

### mkarchiso not found
```bash
sudo pacman -S archiso
```

### Permission denied
```bash
sudo scripts/build_archiso.sh --native
```

### Out of space
```bash
# Ensure at least 20GB free space on Arch VM
df -h
```

### Network issues in VM
```bash
# Enable NetworkManager
sudo systemctl enable --now NetworkManager
# Or use systemd-networkd
```
