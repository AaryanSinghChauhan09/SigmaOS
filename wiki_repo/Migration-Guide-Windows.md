# Migrating from Windows to SigmaOS

> SigmaOS v15.0 "Zenith" — Windows Migration Guide

## Why Migrate?

| Feature | Windows | SigmaOS |
|---|---|---|
| License | Proprietary (paid) | Open source (free) |
| Privacy | Telemetry by default | Zero telemetry |
| Security | Reactive antivirus | Proactive MAC + Pledge + Vault |
| AI | Copilot (cloud) | Local AI (offline) |
| Package management | Microsoft Store / winget | sigpkg (unified, AI-assisted) |
| Update control | Forced updates | User-controlled |

---

## Pre-Migration Checklist

- [ ] Back up Documents, Downloads, Desktop to external storage
- [ ] Export browser bookmarks (Chrome: Settings → Bookmarks → Export)
- [ ] Note which applications you use daily
- [ ] Confirm hardware compatibility at `https://hardware.sigmaos.dev`

---

## Installation

### Step 1: Download SigmaOS ISO

Download from: `https://releases.sigmaos.dev/v15.0/sigmaos-zenith-amd64.iso`

### Step 2: Create Bootable USB (from Windows)

Use **Rufus** (recommended):
1. Download Rufus from `https://rufus.ie`
2. Select the SigmaOS ISO
3. Select your USB drive
4. Click Start (GPT partition for UEFI mode)

Or use the SigmaOS USB Writer tool.

### Step 3: Enable UEFI Boot

1. Hold `Shift` + Restart → Advanced Startup → UEFI Settings
2. Or press `F2/Del` during POST to enter BIOS
3. Disable **Secure Boot** (or enroll the SigmaOS MOK key first)
4. Set USB as first boot device

### Step 4: Install

Boot from USB → "Install SigmaOS" → Choose partition layout:
- **Wipe and install**: Replaces Windows entirely
- **Dual boot**: Keeps Windows, installs SigmaOS alongside (GRUB manages boot)

---

## Windows App Equivalents

| Windows | SigmaOS |
|---|---|
| Microsoft Office | LibreOffice (`sigpkg install libreoffice`) |
| Chrome / Edge | Firefox (`sigpkg install firefox`) |
| Notepad | Mousepad / Kate |
| Paint | GIMP (`sigpkg install gimp`) |
| Task Manager | `sigma-sysmon` GUI or `ps aux` in terminal |
| Control Panel | Zenith Settings Hub |
| Windows Defender | Built-in MAC + IDS |
| PowerShell | `sigma-shell` (POSIX-compatible + scripting) |
| Winget | `sigpkg` |

---

## File System Mapping

| Windows Path | SigmaOS Equivalent |
|---|---|
| `C:\Users\Name\Documents` | `/home/name/Documents` |
| `C:\Program Files` | `/usr/bin` and `/usr/lib` |
| `C:\Windows\System32` | `/usr/sigma/system` |
| `C:\ProgramData` | `/var/lib` |
| Registry | `/etc/sigma/` (TOML config files) |

---

## Keyboard Shortcuts

| Windows | SigmaOS (Zenith Desktop) |
|---|---|
| `Win + R` | `Super + R` (Run dialog) |
| `Win + D` | `Super + D` (Show desktop) |
| `Alt + F4` | `Alt + F4` (Close window) |
| `Ctrl + Alt + Del` | `Ctrl + Alt + T` (Terminal) |
| `Win + I` | `Super + I` (Settings) |

---

## Accessing Windows Files (Dual Boot)

NTFS partitions from Windows are automatically detected and mountable:

```bash
# List detected Windows partitions
sigma-disk list

# Mount Windows C: drive
sudo sigma-disk mount /dev/sda2 /mnt/windows

# Access your files
ls /mnt/windows/Users/YourName/Documents
```

---

## Getting Help

- AI assistant: `sigma-ai "how do I open a .docx file?"`
- Community forum: `https://forum.sigmaos.dev/windows-migrants`
- Beginner guide: `sigpkg install sigma-welcome-guide`
