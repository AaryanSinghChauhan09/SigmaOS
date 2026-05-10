# Σ SIGMAOS MILESTONE 1: BOOTABLE ISO BUILD GUIDE

This document provides a technical guide for generating the first bootable SigmaOS ISO based on Arch Linux.

## 🛠️ Toolchain

* **archiso**: The official tool for building Arch Linux live media.
* **Calamares**: The universal framework for system installers.
* **Zenith-ISO-Profile**: Custom configuration for the live environment.

## 📋 Steps to Generation

### 1. Environment Setup

Install `archiso` on an Arch-based host:

```bash
sudo pacman -S archiso
cp -r /usr/share/archiso/configs/releng/ ~/sigmaos-iso
```

### 2. Package Manifest

Add the core SigmaOS foundation packages to `packages.x86_64`:

* `linux-lts`
* `plasma-desktop`
* `wayland`
* `calamares`
* `networkmanager`
* `sigma-tools-bin` (Custom AUR package)

### 3. Custom Branding

Inject SigmaOS branding into the live environment:

* Replace `/usr/share/plasma/look-and-feel/org.kde.plasma.desktop` with `sigma-neon` theme.
* Set default wallpaper to `zenith_neon.png`.

### 4. Installer Configuration

Configure `calamares` modules for SigmaOS:

* **Partitioning**: Default to Btrfs with subvolumes.
* **Branding**: Update logos and slideshow for the installation process.

### 5. ISO Build

Execute the build script:

```bash
sudo mkarchiso -v -w /tmp/archiso-tmp -o ~/iso-output ~/sigmaos-iso
```

## ✅ Success Criteria

* ISO boots successfully in **QEMU**.
* Network connectivity is established automatically.
* Calamares installer launches and completes successfully.
* Reboot into installed system shows the **Zenith Identity Layer**.

---

### Status: Milestone 1 ACHIEVED
