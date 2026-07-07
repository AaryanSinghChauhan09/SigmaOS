# Migrating from Ubuntu to SigmaOS

> SigmaOS v15.0 "Zenith" — Ubuntu Migration Guide

## Why Migrate?

| Feature | Ubuntu | SigmaOS |
|---|---|---|
| Base language | C (kernel) | Rust (memory-safe) |
| Package manager | apt/snap | sigpkg (SAT-resolver + AI) |
| Init system | systemd | Sovereign Init (parallel, lightweight) |
| AI integration | None | Local AI agent (offline) |
| Security | AppArmor | MAC + Pledge + Seccomp + Vault |
| Filesystem | ext4 default | SigmaFS + Btrfs snapshots |
| Snapshots | Manual | Automatic generation rollback |

---

## Pre-Migration Checklist

- [ ] Back up all data to external drive or cloud
- [ ] Note installed packages: `dpkg --get-selections > packages.txt`
- [ ] Export config files: `cp -r ~/.config ~/config_backup`
- [ ] Note cron jobs: `crontab -l > my_cron.txt`
- [ ] Note systemd services: `systemctl list-units --type=service --state=running`

---

## Installation

### Step 1: Download SigmaOS ISO

```bash
wget https://releases.sigmaos.dev/v15.0/sigmaos-zenith-amd64.iso
```

### Step 2: Verify Signature

```bash
sigma-verify sigmaos-zenith-amd64.iso sigmaos-zenith-amd64.iso.sig
```

### Step 3: Create Bootable USB

```bash
sudo dd if=sigmaos-zenith-amd64.iso of=/dev/sdX bs=4M status=progress
```

### Step 4: Boot and Install

Boot from USB → Select "Install SigmaOS" → Follow the Calamares-style installer.

**Dual Boot**: Select "Install alongside Ubuntu" to keep your Ubuntu partition.

---

## Post-Migration: Translating Commands

| Ubuntu/apt | SigmaOS/sigpkg |
|---|---|
| `sudo apt update` | `sigpkg update` |
| `sudo apt upgrade` | `sigpkg upgrade` |
| `sudo apt install pkg` | `sigpkg install pkg` |
| `sudo apt remove pkg` | `sigpkg remove pkg` |
| `apt search query` | `sigpkg search query` |
| `dpkg --list` | `sigpkg list` |

---

## Translating Systemd Services

| systemd | Sovereign Init |
|---|---|
| `systemctl start nginx` | `sigma-init start nginx` |
| `systemctl enable nginx` | `sigma-init enable nginx` |
| `systemctl status nginx` | `sigma-init status nginx` |
| `journalctl -u nginx` | `sigma-journal query --service nginx` |
| `/etc/systemd/system/nginx.service` | `/etc/sigma/services/nginx.service` |

### Service File Format

SigmaOS `.service` files use a compatible INI format:

```ini
[Unit]
Name = nginx
Description = Nginx Web Server
Requires = network.service

[Service]
ExecStart = /usr/bin/nginx -g "daemon off;"
RestartOnFailure = true

[Install]
WantedBy = multi-user.target
```

---

## Common Package Equivalents

| Ubuntu | SigmaOS |
|---|---|
| `vim` | `sigpkg install vim` |
| `python3` | `sigpkg install python3` |
| `build-essential` | `sigpkg install sigma-dev-tools` |
| `openssh-server` | `sigpkg install sigma-ssh` |
| `ufw` | Built-in via `sigma-firewall` CLI |

---

## Getting Help

- AI assistant: `sigma-ai "how do I configure nginx on SigmaOS?"`
- Community: `https://forum.sigmaos.dev`
- IRC: `#sigmaos` on Libera.Chat
