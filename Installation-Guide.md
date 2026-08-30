# 📖 Installation Guide

This guide provides an **Arch Linux-style, step-by-step installation procedure** for deploying SigmaOS on bare-metal hardware (`x86_64`, `aarch64`, `riscv64`) or virtualized environments (QEMU/KVM, Xen, Bhyve).

***

## 🛠️ Pre-Installation

### 1. Acquire Installation Medium

Download the latest bootable sovereign ISO image (`sigmaos-bootable-x86_64.iso`) or compile from source:

```bash
# Build bootable ISO target
make iso
```

### 2. Verify Medium Signature

Verify the ISO using Post-Quantum Dilithium-5 cryptographic attestation:

```bash
sigpkg verify-signature sigmaos-bootable-x86_64.iso --key dilithium5:root
```

### 3. Boot the Live Environment

Flash to a USB drive using `dd`:

```bash
dd if=sigmaos-bootable-x86_64.iso of=/dev/sdX bs=4M status=progress conv=fsync
```

***

## 💾 Disk Partitioning & Filesystems

SigmaOS supports standard GPT partition tables (`GptDiskPartitionEngine`) and FreeBSD/OpenBSD slice labels (`BsdDisklabelEngine`).

### 1. Create Partition Table (GPT)

```bash
# Create 1MiB sector-aligned GPT layout
sigma-parted /dev/nvme0n1 mklabel gpt
sigma-parted /dev/nvme0n1 mkpart ESP fat32 1MiB 512MiB
sigma-parted /dev/nvme0n1 set 1 boot on
sigma-parted /dev/nvme0n1 mkpart root ext4 512MiB 100%
```

### 2. Format Filesystems

```bash
# Format UEFI System Partition
mkfs.vfat -F32 /dev/nvme0n1p1

# Format Root Partition with Ext4 / Btrfs / ZFS CoW Engine
mkfs.ext4 -F -O fast_commit /dev/nvme0n1p2
```

***

## ⚙️ System Bootstrap (`sigstrap`)

Mount target partitions and bootstrap the minimal sovereign system core:

```bash
# Mount target root
mount /dev/nvme0n1p2 /mnt
mkdir -p /mnt/boot/efi
mount /dev/nvme0n1p1 /mnt/boot/efi

# Bootstrap core packages (Arch pacstrap equivalent)
sigstrap /mnt base sigmaos-kernel zenith-desktop sigpkg systemd-init
```

***

## 🔧 System Configuration (`sigchroot`)

Enter the installed chroot environment:

```bash
sigchroot /mnt
```

### 1. Timezone & Locale

Configure system timezone and UTF-8 locale:

```bash
ln -sf /usr/share/zoneinfo/UTC /etc/localtime
echo "LANG=C.UTF-8" > /etc/locale.conf
```

### 2. Hostname & Network

```bash
echo "sigma-node" > /etc/hostname
cat << EOF > /etc/hosts
127.0.0.1   localhost
::1         localhost
127.0.1.1   sigma-node.localdomain sigma-node
EOF
```

### 3. Bootloader Setup

Install the sovereign UEFI boot manager (`limine` / `grub` parity):

```bash
sigma-bootctl install --esp-path=/boot/efi
```

***

## 🚀 Post-Installation

Exit chroot, unmount partitions, and reboot into the sovereign system:

```bash
exit
umount -R /mnt
reboot
```
