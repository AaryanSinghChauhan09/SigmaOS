# 🛠️ SigmaOS Installation Guide (Arch-Style)

This guide provides step-by-step technical instructions for installing **SigmaOS Desktop Edition** on bare-metal systems, QEMU/KVM virtual machines, or VirtualBox targets.

SigmaOS follows the **Arch Linux Installation Philosophy**: transparent, modular, reproducible, and user-empowered.

---

## 📋 System Requirements

| Resource | Minimum | Recommended |
|----------|---------|-------------|
| **CPU** | x86_64-v2 (SSE4.2, SSSE3) | x86_64-v3 or ARM64 (64-bit multi-core) |
| **RAM** | 2 GB | 8 GB+ |
| **Storage** | 16 GB NVMe / SSD | 64 GB+ NVMe CoW storage |
| **Firmware** | UEFI (Recommended) or Legacy BIOS | UEFI with Secure Boot & TPM 2.0 |

---

## 🚀 Step 1: Pre-Installation & Live ISO Boot

1. **Download Live ISO Image**: Obtain `sigmaos-desktop-2026.08-x86_64.iso`.
2. **Flash to USB Drive**:
   ```bash
   dd if=sigmaos-desktop-2026.08-x86_64.iso of=/dev/sdX bs=4M status=progress conv=fsync
   ```
3. **Boot Live Environment**: Boot from the USB drive. You will land directly in the interactive sovereign shell (`sigma-sh`) with live Zenith compositor support.
4. **Verify Network Connectivity**:
   ```bash
   # Test network interface binding
   sigma-sh net status
   ping -c 3 sigma-os.org
   ```

---

## 💾 Step 2: Disk Partitioning & Filesystem Layout

SigmaOS uses a **Dual-Root A/B Partition Scheme** for atomic updates and sub-millisecond Copy-on-Write (CoW) system rollbacks.

```
   ┌────────────────────────────────────────────────────────────────────────┐
   │                        STORAGE PARTITION LAYOUT                        │
   ├──────────────┬───────────────┬───────────────┬─────────────────────────┤
   │ Partition    │ Type          │ Size          │ Mount Point             │
   ├──────────────┼───────────────┼───────────────┼─────────────────────────┤
   │ /dev/nvme0n1p1│ FAT32 (EFI)   │ 1 GB          │ /boot/efi               │
   │ /dev/nvme0n1p2│ Bcachefs/ZFS  │ 25 GB         │ /sovereign/sysroot_a    │
   │ /dev/nvme0n1p3│ Bcachefs/ZFS  │ 25 GB         │ /sovereign/sysroot_b    │
   │ /dev/nvme0n1p4│ Bcachefs/ZFS  │ Remaining     │ /home & /var/store      │
   └──────────────┴───────────────┴───────────────┴─────────────────────────┘
```

### Partitioning Execution:
```bash
# Partition drive using parted
parted -s /dev/nvme0n1 mklabel gpt
parted -s /dev/nvme0n1 mkpart ESP fat32 1MiB 1025MiB
parted -s /dev/nvme0n1 set 1 boot on
parted -s /dev/nvme0n1 mkpart sysroot_a bcachefs 1025MiB 26625MiB
parted -s /dev/nvme0n1 mkpart sysroot_b bcachefs 26625MiB 52225MiB
parted -s /dev/nvme0n1 mkpart data bcachefs 52225MiB 100%

# Format partitions
mkfs.vfat -F 32 -n "SIGMA_EFI" /dev/nvme0n1p1
sigpkg mkfs --cow /dev/nvme0n1p2
sigpkg mkfs --cow /dev/nvme0n1p3
sigpkg mkfs --cow /dev/nvme0n1p4
```

---

## 📦 Step 3: Mount Sysroot & Bootstrap (`sigpstrap`)

```bash
# Mount target rootfs partition
mount /dev/nvme0n1p2 /mnt
mkdir -p /mnt/boot/efi /mnt/home
mount /dev/nvme0n1p1 /mnt/boot/efi
mount /dev/nvme0n1p4 /mnt/home

# Bootstrap base system (Arch pacstrap parity)
sigpstrap /mnt base kernel zenith-desktop sigpkg doas
```

---

## ⚙️ Step 4: System Configuration

### 1. Generate Fstab Matrix (`sigfstab`):
```bash
sigfstab -U /mnt >> /mnt/etc/fstab
```

### 2. Enter Chroot Target:
```bash
sigma-chroot /mnt
```

### 3. Set System Profile & Preferences:
Create `/system/profile.toml`:
```toml
[system]
hostname = "sigma-workstation"
timezone = "UTC"
locale = "en_US.UTF-8"

[updates]
strategy = "atomic_dual_root"
auto_rollback_on_failure = true

[security]
exec_guard = "zorin_default_deny"
sandboxing = "pledge_unveil"
```

### 4. Create User & Configure Privilege Delegation (`doas`):
```bash
# Create sovereign user
useradd -m -G wheel -s /usr/bin/sigma-sh sovereign

# Configure /etc/doas.conf
echo "permit :wheel as root" > /etc/doas.conf
chmod 0400 /etc/doas.conf
```

---

## ⚡ Step 5: Install Bootloader (`SigmaBootloaderEngine`)

```bash
# Install SigmaBootloader to EFI System Partition
sigmaboot --install --target=/boot/efi --tpm-measured-boot

# Generate bootloader configuration
sigmaboot --update-config
```

---

## 🔄 Step 6: Finalize & Reboot

```bash
# Exit chroot
exit

# Unmount target partitions
umount -R /mnt

# Reboot into SigmaOS
reboot
```

Upon boot, login with user `sovereign`. You will land directly in the keyboard-driven Zenith desktop environment!
