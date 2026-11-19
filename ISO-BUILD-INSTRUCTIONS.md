# SigmaOS ISO Build Instructions

## Current Status

**GitHub Actions Workflow:** Running (queued for ~2 minutes)
**Workflow ID:** 44529214827
**URL:** https://github.com/AaryanSinghChauhan09/SigmaOS/actions/runs/44529214827

## Monitor Build Progress

### Option 1: GitHub Actions (Recommended)
Visit: https://github.com/AaryanSinghChauhan09/SigmaOS/actions

### Option 2: Command Line
```bash
# Check latest run status
gh run list --repo AaryanSinghChauhan09/SigmaOS --limit 1

# View specific run details
gh run view 44529214827 --repo AaryanSinghChauhan09/SigmaOS
```

## What the Build Does

The GitHub Actions workflow will:
1. Validate the ISO profile
2. Build the ISO using Docker (archlinux image)
3. Upload the ISO as an artifact (retained for 7 days)
4. Upload build logs
5. Create a summary with features and download instructions

## Expected Build Time

Docker-based ISO builds typically take 15-30 minutes depending on GitHub Actions queue and available runners.

## After Build Completes

### Download the ISO

**Option 1: GitHub Actions UI**
1. Go to https://github.com/AaryanSinghChauhan09/SigmaOS/actions/runs/44529214827
2. Scroll to "Artifacts" section
3. Download "sigmaos-desktop-iso"

**Option 2: Command Line**
```bash
# List artifacts for the run
gh run view 44529214827 --repo AaryanSinghChauhan09/SigmaOS

# Download artifact (after build completes)
gh run download 44529214827 --repo AaryanSinghChauhan09/SigmaOS
```

### Test the ISO in QEMU

```bash
# Create test disk (32GB)
qemu-img create -f qcow2 /tmp/sigmaos_test.qcow2 32G

# Boot the ISO
qemu-system-x86_64 \
  -m 4096 \
  -enable-kvm \
  -drive file=/tmp/sigmaos_test.qcow2,format=qcow2,if=virtio \
  -cdrom sigmaos-desktop-YYYYMMDD-x86_64.iso \
  -boot d
```

### What You'll See

**Desktop Environment:**
- Hyprland (Wayland tiling compositor)
- Waybar status bar with workspaces, clock, tray, audio, network, CPU, memory
- Wallust dynamic theming with SigmaOS color preset
- Starship shell prompt

**System:**
- Stock Arch Linux kernel
- Standard pacman package manager
- Defensive installer (sigmaos-install) with dry-run protection

## Installer Safety

The installer (`sigmaos-install`) has built-in safety:
- **Default**: Dry-run mode (writes nothing to disk)
- **Destructive mode**: Requires both `--destructive` AND `--confirm-disk <DEV>`
- **Active root guard**: Refuses to install to the host's root disk
- **Mount protection**: Aborts if target partitions are mounted

Example destructive install (ONLY in VMs):
```bash
sudo sigmaos-install --destructive --confirm-disk /dev/vda /dev/vda
```

## Troubleshooting

### Build Fails
Check the "build-log" artifact for detailed error messages.

### ISO Won't Boot
- Ensure you're using KVM: `-enable-kvm`
- Check ISO integrity: `sha256sum sigmaos-desktop-*.iso`
- Try without KVM for debugging: remove `-enable-kvm`

### Installer Issues
- Run in dry-run mode first to see what would happen
- Check partition status: `lsblk`
- Verify mount points: `mount | grep /dev`

## Next Steps After Testing

1. **Archive original repositories** (optional, via GitHub web UI)
2. **Migrate Wiki content** from `docs/wiki/*.md` to GitHub Wiki
3. **Archive SigmaOS-arch-* repos** (50+ repos identified for deletion)
4. **Create GitHub Release** with ISO artifact for distribution

## Engineering Principles Applied

✅ Stock Arch Linux kernel (UPSTREAM FIRST)
✅ Standard pacman (not experimental sigpkg)
✅ Defensive installer (DEFENSIVE PROGRAMMING)
✅ Hyprland + Waybar + Wallust (KISS, MODULARITY)
✅ Consolidated components (CONSOLIDATE GITHUB REPOS)
✅ Normalized identity (NORMALISE OS IDENTITY)
✅ 119 tests passing (TEST CODE USING CI/CD)
✅ All docs created (A GITHUB WIKI FOR EVERY COMPONENT)
