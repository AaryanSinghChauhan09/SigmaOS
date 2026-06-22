# Build Guide

This guide walks you through building a bootable SigmaOS ISO from source.

---

## Prerequisites

- Ubuntu 22.04+ or Debian 12+ (x86_64)
- 16 GB RAM minimum (for Buildroot compilation)
- 50 GB free disk space
- Internet access (for fetching Buildroot packages and TinyLlama model)

---

## Quick Build

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Install host tools + build ISO (standalone desktop profile)
./build-iso.sh standalone

# Output: output/sigmaos-v0.1-standalone.iso
```

---

## Manual Steps

### 1. Install Host Dependencies

```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential git golang-go wget unzip bc cpio rsync \
    qemu-system-x86 bubblewrap libseccomp-dev
```

### 2. Build Go Daemons

```bash
# Build each daemon for Linux amd64
for d in sigmad-process sigmad-ai sigmad-sync; do
  cd userland/daemons/$d
  GOOS=linux GOARCH=amd64 go build -o ../../../build/bin/$d .
  cd ../../..
done
```

### 3. Configure Buildroot

```bash
cd buildroot
make sigmaos_defconfig
```

The `sigmaos_defconfig` enables:
- `BR2_PACKAGE_CHROMIUM=y` — Chromium browser
- `BR2_PACKAGE_LLAMA_CPP=y` — On-device AI
- `BR2_PACKAGE_BUBBLEWRAP=y` — Process sandboxing
- `BR2_PACKAGE_ALPINE_APK_TOOLS=y` — Package installation
- `BR2_PACKAGE_RCLONE=y` — Cloud sync

### 4. Build the ISO

```bash
make -j$(nproc)
# Takes ~40 min on first run, ~5 min on rebuilds
```

Output: `buildroot/output/images/rootfs.iso`

---

## Testing in QEMU

```bash
qemu-system-x86_64 \
  -m 2G \
  -cdrom output/sigmaos-v0.1-standalone.iso \
  -boot d \
  -enable-kvm \
  -cpu host
```

Default credentials: **user** `sigma` / **pass** `sigma`

Open `http://localhost:3000` for the web shell. Open `http://localhost:3000/docs.html` for API docs.

---

## Profile-Specific Builds

```bash
./build-iso.sh cloud       # Container / cloud-native profile
./build-iso.sh mobile      # ARM64 / mobile profile
./build-iso.sh rtos        # Real-time embedded profile
./build-iso.sh browser     # WebAssembly / browser profile
```

---

## Flash to USB

```bash
sudo dd if=output/sigmaos-v0.1-standalone.iso \
    of=/dev/sdX bs=4M status=progress
```

Replace `/dev/sdX` with your USB drive. Use `lsblk` to identify it.

---

## Troubleshooting

| Issue | Fix |
|-------|-----|
| `bwrap` not found | `sudo apt install bubblewrap` |
| Go build fails | `go version` must be ≥ 1.21 |
| Chromium crash on boot | Increase QEMU RAM to 4G |
| TinyLlama slow | Add `-cpu host -enable-kvm` to QEMU flags |
