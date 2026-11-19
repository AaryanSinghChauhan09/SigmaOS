# ISO Build Guide

This guide explains how to build the SigmaOS live ISO image.

## Quick Start

### Option 1: GitHub Actions (Recommended)

The ISO is built automatically on push to main branch:
1. Push changes to SigmaOS repository
2. Go to Actions tab: https://github.com/AaryanSinghChauhan09/SigmaOS/actions
3. Download ISO from artifacts when build completes

### Option 2: Docker (Any Linux Host)

```bash
# Install Docker
sudo apt-get install -y docker.io
sudo systemctl start docker
sudo usermod -aG docker $USER
# Log out and back in

# Build ISO
cd SigmaOS
scripts/build_archiso.sh --docker
```

### Option 3: Native Arch Linux

```bash
sudo pacman -S archiso
sudo scripts/build_archiso.sh --native
```

## Architecture

The ISO uses:
- **Stock Arch Linux kernel** (linux, linux-firmware, linux-headers)
- **Hyprland** Wayland tiling compositor
- **Waybar** status bar
- **Wallust** dynamic theming
- **PipeWire** audio stack
- **116 curated packages**

## Build Process

1. **Validation**: Checks archiso profile structure
2. **Package installation**: Downloads 116 packages from Arch repositories
3. **Filesystem**: Creates squashed root filesystem
4. **Boot configuration**: Configures systemd-boot for UEFI
5. **ISO generation**: Creates bootable ISO image

## Output

- **Location**: `out/sigmaos-desktop-YYYYMMDD-x86_64.iso`
- **Size**: ~2-3 GB
- **Retention**: 7 days (GitHub Actions)

## Testing in QEMU

```bash
qemu-img create -f qcow2 /tmp/sigmaos_test.qcow2 32G
qemu-system-x86_64 \
  -m 4096 \
  -enable-kvm \
  -drive file=/tmp/sigmaos_test.qcow2,format=qcow2,if=virtio \
  -cdrom out/sigmaos-desktop-YYYYMMDD-x86_64.iso \
  -boot d
```

## Troubleshooting

### Docker permission denied
```bash
sudo usermod -aG docker $user
# Log out and back in
```

### mkarchiso not found
```bash
sudo pacman -S archiso  # On Arch only
```

### Build fails
Check `out/build.log` for errors
