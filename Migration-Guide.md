# Migration Guide — Moving to SigmaOS

> Step-by-step guide for migrating from Ubuntu, Fedora, Arch, Debian, or any Linux distro.

---

## Before You Migrate

SigmaOS runs Linux apps through its compatibility layer. You don't need to give up your existing software — it all runs on SigmaOS.

### Prerequisites:

- 4GB RAM minimum (8GB recommended)

- 20GB disk space

- x86_64, ARM64, or RISC-V processor

- UEFI firmware (Legacy BIOS supported via dual-boot profile)

---

## Option 1: Dual Boot (Safest)

Keep your existing Linux distro and add SigmaOS alongside it.

```bash

# Download SigmaOS Zenith ISO

wget https://github.com/AaryanSinghChauhan09/SigmaOS/releases/latest/download/sigmaos-zenith.iso

# Create bootable USB

dd if=sigmaos-zenith.iso of=/dev/sdX bs=4M status=progress

# Or: sigma-agent "create bootable usb sigmaos-zenith.iso /dev/sdX"

# Boot from USB → select "Install alongside existing OS"

# SigmaOS installer detects existing GRUB/systemd-boot automatically

```

---

## Option 2: Virtual Machine (Try First)

Run SigmaOS in QEMU without touching your current system:

```bash

# Install QEMU

apt install qemu-system-x86  # or: dnf install qemu-kvm

# Run SigmaOS ISO

qemu-system-x86_64 \
  -m 2G -smp 2 \
  -drive file=sigmaos-zenith.iso,format=raw,readonly=on \
  -boot d \
  -enable-kvm \
  -vga virtio

# Or use the provided script:

./qemu-boot.sh
```

---

## Migrating Your Software

### From Ubuntu/Debian (apt → sigma-pkg)

| Ubuntu command | SigmaOS equivalent |
|---|---|
| `apt install <pkg>` | `sigma-pkg install <pkg>` |
| `apt remove <pkg>` | `sigma-pkg remove <pkg>` |
| `apt update` | `sigma-pkg update` |
| `apt search <query>` | `sigma-pkg search <query>` |
| `apt list --installed` | `sigma-pkg list` |
| `dpkg -i <file.deb>` | `sigma-pkg absorb <file.deb>` |

### Absorb your existing .deb packages:

```bash

# Export installed package list from Ubuntu

dpkg --get-selections | awk '{print $1}' > my_packages.txt

# Install them on SigmaOS (sigma-pkg resolves equivalents automatically)

sigma-agent "install all packages from my_packages.txt"

# Or manually:

while read pkg; do sigma-pkg install "$pkg" 2>/dev/null || true; done < my_packages.txt
```

### From Fedora/RHEL (dnf → sigma-pkg)

| Fedora command | SigmaOS equivalent |
|---|---|
| `dnf install <pkg>` | `sigma-pkg install <pkg>` |
| `dnf remove <pkg>` | `sigma-pkg remove <pkg>` |
| `dnf upgrade` | `sigma-pkg update` |
| `rpm -i <file.rpm>` | `sigma-pkg absorb <file.rpm>` |

### From Arch (pacman → sigma-pkg)

| Arch command | SigmaOS equivalent |
|---|---|
| `pacman -S <pkg>` | `sigma-pkg install <pkg>` |
| `pacman -R <pkg>` | `sigma-pkg remove <pkg>` |
| `pacman -Syu` | `sigma-pkg update` |
| `pacman -Qs <query>` | `sigma-pkg search <query>` |

---

## Migrating Your Dotfiles and Config

```bash

# sigma-agent can help migrate configs

sigma-agent "migrate my .bashrc to sigma-sh"
sigma-agent "import my vim config"
sigma-agent "set up my development environment from requirements.txt"

# Copy home directory configs

rsync -av ~/.config/ ~/new-sigmaos-install/.config/
rsync -av ~/.local/ ~/new-sigmaos-install/.local/

# Import SSH keys

cp -r ~/.ssh/ ~/new-install/.ssh/
chmod 700 ~/new-install/.ssh
```

---

## Shell Transition

If you use bash/zsh, sigma-sh is compatible:

```bash

# sigma-sh is POSIX-compatible — most scripts run unmodified

# Source your existing .bashrc:

echo 'source ~/.bashrc' >> ~/.sigma_profile

# Or use sigma-agent shell integration for enhanced experience:

sigma-agent install --shell-integration

# Now: ai "your request", aifix <file>, ai-dark, ai-sysinfo

```

---

## Running Linux Apps That Aren't Packaged Yet

```bash

# AppImages (portable — just run them)

chmod +x MyApp.AppImage && ./MyApp.AppImage

# Or: sigma-pkg absorb MyApp.AppImage && sigma-pkg install myapp

# Flatpaks

sigma-pkg install --flatpak org.mozilla.firefox
sigma-pkg install --flatpak org.gimp.GIMP

# Docker/OCI containers

sigma-compat container ubuntu:22.04 bash  # full Ubuntu shell

sigma-compat container node:18 node -e "console.log('hello')"

# Direct Linux binary execution

sigma-compat run /path/to/linux-binary --arg1 --arg2
```

---

## Development Environment Migration

### Rust developers

```bash
sigma-pkg install rust-toolchain

# Everything else is identical — cargo, rustfmt, clippy all work

```

### Python developers

```bash
sigma-pkg install sigma-python3
pip install -r requirements.txt  # pip works identically

```

### Node.js developers

```bash
sigma-pkg install sigma-node
npm install  # npm/yarn/pnpm work identically

```

### Docker/container developers

```bash
sigma-pkg install sigma-pod

# sigma-pod is OCI-compatible — docker-compose.yml files work directly

sigma-pod compose up
```

---

## Post-Migration Checklist

```bash

# 1. Health check

sigma-agent doctor

# 2. Install workflow automation templates

sigma-agent workflow install --all

# 3. Set up AI agent

sigma-agent daemon start
sigma-agent install --shell-integration

# 4. Security hardening

sigma-agent security scan
sigma-agent workflow run security-hardening

# 5. Set preferences

sigma-agent "set dark mode"
sigma-agent memory add "migrated from Ubuntu" --fact

# 6. Verify your key apps work

sigma-agent "list installed"
```

---

## Getting Help

```bash
sigma-agent explain "how does sigma-pkg differ from apt"
sigma-agent "what is the sigma-os equivalent of systemctl"
sigma-agent multi --agent teacher "explain how SigmaOS works"
```

Community: https://github.com/AaryanSinghChauhan09/SigmaOS/discussions

---

*See also: [Linux Absorption Architecture](Linux-Absorption-Architecture) · [Getting Started](Getting-Started) · [sigma-agent](sigma-agent)*
