# 🖥️ SigmaOS Standalone Branch — `release/standalone`

> **Born to Boot Alone: Full-featured SigmaOS on bare metal, no host OS required.**

The `release/standalone` branch is the **flagship single-machine installation** of SigmaOS, absorbing:
- **Arch Linux** (minimal base, user-assembled system)
- **Gentoo** (compile-from-source, hardware-optimized)
- **Void Linux** (runit init, xbps package manager)
- **NixOS** (reproducible system configurations)
- **Fedora Workstation** (modern desktop defaults)

---

## 🚀 Boot Sequence (Full Standalone)

```
UEFI/BIOS
    ↓
sigma_bootloader.elf    (Stage 1: FAT32-resident)
    ↓
sigma_stage2.cpp        (Stage 2: long mode, page tables)
    ↓
sigma_kernel_main()     (Kernel init, PCI scan, driver probe)
    ↓
sigma_init.cpp          (PID 1: service manager)
    ↓
sigma_login.cpp         (Getty/login prompt)
    ↓
sigma_sh_run()          (Shell session)
    ↓
zenith_desktop          (Optional: Zenith GUI session)
```

---

## 📦 Standalone Package Sets

Inspired by **Arch Linux's package groups** and **Debian metapackages**:

| Package Set | Contents | Size |
|-------------|----------|------|
| `sigma-base` | kernel, shell, coreutils (31 tools) | ~2MB |
| `sigma-network` | TCP/IP stack, ifconfig, ping, DNS | ~500KB |
| `sigma-storage` | ZFS, fdisk, tar, mount | ~800KB |
| `sigma-security` | PQC, firewall, strace | ~1.2MB |
| `sigma-gui` | Zenith desktop, compositor | ~8MB |
| `sigma-dev` | Compiler, debugger, awk, sed | ~3MB |
| `sigma-full` | All of the above | ~16MB |

---

## 🔧 Hardware Support Matrix

| Component | Driver | Status |
|-----------|--------|--------|
| Storage (SATA) | `sigma_ahci.cpp` | ✅ Ready |
| Storage (NVMe) | `sigma_nvme.cpp` | ✅ Ready |
| Storage (VirtIO) | `sigma_virtio_blk.cpp` | ✅ Ready |
| Storage (ATA/IDE) | `sigma_ata_driver.cpp` | ✅ Ready |
| Network (e1000) | `sigma_e1000.cpp` | ✅ Ready |
| Network (VirtIO) | `sigma_virtio_net.cpp` | 🔧 In progress |
| Display (VGA) | `sigma_vga.cpp` | ✅ Ready |
| Display (Framebuffer) | `sigma_fb.cpp` | ✅ Ready |
| Keyboard (PS/2) | `sigma_ps2.cpp` | ✅ Ready |
| USB HID | `sigma_xhci.cpp` | ✅ Ready |
| Audio (AC97) | `sigma_ac97.cpp` | ✅ Ready |
| PCI Bus | `sigma_pci.cpp` | ✅ Ready |

---

## 🛠 Init System (`sigma_init.cpp`)

Absorbed from **runit** (Void Linux), **OpenRC** (Alpine/Gentoo), **s6** (supervision suite):

```
/etc/sigma/services/
  ├── network      (sigma_net_daemon)
  ├── storage      (sigma_vfs_daemon)
  ├── cron         (sigma_cron)
  ├── syslog       (sigma_syslog)
  └── login        (sigma_getty)
```

Each service is a plain executable:
- `run` — starts the service (exec-based, no scripts)
- `finish` — called on exit with exit code

No shell scripting. No systemd. No udev. **Pure sovereign binaries.**

---

## 📀 Installation

```bash
# Build standalone ISO
make PROFILE=standalone iso
# Output: sigmaos-standalone-amd64.iso (~32MB)

# Write to USB
dd if=sigmaos-standalone-amd64.iso of=/dev/sdX bs=4M status=progress

# Boot in QEMU for testing
./qemu-boot.sh standalone
```

---

## 🔒 Integrity Verification

Absorbed from **Gentoo's verified kernel** and **NixOS hash-locked closures**:

- Each installed file has a SHA-256 hash (sovereign impl)
- Boot-time integrity check via `sigma_secure_boot.cpp`
- Package signatures use Dilithium PQC (`sigma_dilithium.cpp`)

---

*Branch: `release/standalone` | Target: Physical x86_64 hardware + QEMU*
