# SigmaOS Build Instructions

## Quick Start (Linux Mint/Ubuntu)

### 1. Enable Docker Access

```bash
# Add your user to docker group
sudo usermod -aG docker $USER

# Log out and log back in for changes to take effect
# Then verify:
groups
```

### 2. Build the ISO

```bash
cd /home/aaryansinghchauhan/SigmaOS
scripts/build_archiso.sh --docker
```

### 3. Test in QEMU

```bash
# Create test disk
qemu-img create -f qcow2 /tmp/sigmaos_test.qcow2 32G

# Boot ISO
qemu-system-x86_64 \
  -m 4096 \
  -enable-kvm \
  -drive file=/tmp/sigmaos_test.qcow2,format=qcow2,if=virtio \
  -cdrom out/sigmaos-desktop-YYYYMMDD-x86_64.iso \
  -boot d
```

## Alternative: Build on Arch Linux

If you have access to an Arch Linux system or VM:

```bash
sudo pacman -S archiso
sudo scripts/build_archiso.sh --native
```

## Validation Only

To validate the ISO profile without building:

```bash
scripts/build_archiso.sh --validate-only
```

## What Gets Built

The ISO includes:
- Stock Arch Linux kernel (bootable)
- Hyprland Wayland compositor with SigmaOS branding
- Waybar status bar
- Wallust dynamic theming
- PipeWire audio stack
- 116 curated packages for a functional desktop
- Defensive installer with dry-run protection

## Installer Safety

The installer (`sigmaos-install`) defaults to dry-run mode. To actually install:

```bash
# Destructive install (ONLY in VMs)
sudo sigmaos-install --destructive --confirm-disk /dev/vda /dev/vda
```

## Troubleshooting

### Docker permission denied
```bash
sudo usermod -aG docker $user
# Log out and back in
```

### QEMU not found
```bash
sudo apt-get install qemu-kvm libvirt-daemon-system libvirt-clients bridge-utils virtinst
```

### Build fails
Check the build log: `out/build.log`
