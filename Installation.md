# 🚀 Installation Guide

SigmaOS is designed for rapid deployment across sovereign hardware and cloud infrastructure.

## 🛠 Supported Platforms
*   **Bare-metal**: Zero-dependency Sovereign Lattice installer for x86_64 and ARM64.
*   **Cloud Images**: AWS/GCP/Azure optimized images with pre-baked performance tuning (neutralizing **Clear Linux**).
*   **ARM64 Builds**: RPi5-native optimizations and sovereign micro-edition builds (crushing **RPi-Distro** and **Alpine**).

## 📥 Getting Started
1. **Download ISO**: Obtain the latest Sovereign Lattice ISO from the [Releases](https://github.com/AaryanSinghChauhan09/SigmaOS/releases) page.
2. **Flash to Media**: Use `dd` or a sovereign flashing tool.
   ```bash
   sudo dd if=sigmaos-zenith.iso of=/dev/sdX bs=4M status=progress
   ```
3. **Boot & Ignite**: Enter the UEFI menu and select the SigmaOS Lattice entry.

## 🏗 Industrial Matrix Builds
SigmaOS utilizes a matrix-based build system to ensure perfect hardware sovereignty:
- `sigma-core-x86`: For standard workstations.
- `sigma-sovereign-arm64`: Optimized for RPi5 and Apple Silicon.
- `sigma-micro`: Minimal footprint for edge computing.

## 🛡 Verification
Always verify the integrity of your download using the sovereign PGP signature:
```bash
gpg --verify sigmaos-zenith.iso.sig
```
