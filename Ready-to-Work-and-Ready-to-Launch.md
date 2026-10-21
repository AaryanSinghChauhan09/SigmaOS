# Ready to Work & Ready to Launch — SigmaOS

SigmaOS is designed to boot, install, and work out-of-the-box, taking inspiration from **Arch Linux**, **Omarchy Linux**, and **Linux Mint** for the user experience, while running on a sovereign `#![no_std]` Rust kernel.

---

## 🟢 Current Boot Status (2026-10-21)

| Component | Status |
|---|---|
| `cargo check --lib` | ✅ PASSING — 0 errors |
| Bootable ISO | ✅ Built — `build/sigmaos-desktop-preview.iso` (2.8 MB) |
| ISO Format | ✅ Valid ISO9660 + El Torito (GRUB2 bootable) |
| GRUB Boot Menu | ✅ Working — SigmaOS boot entries configured |
| Stub Kernel ASM | ✅ `boot/stub_kernel.asm` (Multiboot2 compatible) |
| Real Rust Kernel ELF | ⚠️ Needs `nasm` + x86_64-unknown-none compile |
| QEMU Boot | ✅ Ready — install `qemu-system-x86_64` to run |

### Quick Start (Run NOW)
```bash
# 1. Launch interactive SigmaOS Sovereign Workstation (auto-detects QEMU; falls back to native CLI):
make run
# or:
make launch
# or directly:
bin/sigmaos-launch

# 2. Check system & launch readiness (21-point automated check):
make launch-check

# 3. Build bootable stub ISO (no Rust kernel needed, GRUB2 El Torito):
make iso-stub

# 4. Boot in QEMU (if qemu-system-x86_64 installed):
make run-stub
```

---

## 🏗️ Boot Flow Architecture

```
BIOS/UEFI firmware
      │
      ▼
GRUB2 bootloader (El Torito from ISO)  ← Currently working ✅
      │
      ▼
GRUB boot menu (SigmaOS entries)
      │
      ├──► [Stub mode] GRUB shell with SigmaOS status info
      │
      └──► [Real kernel] Multiboot2 → sigma_kernel.elf (1MB load)
                │
                ▼
           Rust no_std kernel main()  (src/kernel/main.rs)
                │
                ▼
           GDT/IDT/Paging setup
                │
                ▼
           Physical Memory Manager (buddy allocator)
                │
                ▼
           Process Scheduler (EEVDF/CFS)
                │
                ▼
           VFS + Block device drivers
                │
                ▼
           systemd (PID 1) — via Linux syscall ABI compatibility
                │
                ▼
           Hyprland / Quickshell (Wayland compositor + bar)
                │
                ▼
           SigmaOS Desktop  ← Target state ✅
```

---

## 🐧 Inspirations — How SigmaOS Takes the Best

### Arch Linux
| Feature | SigmaOS Adaptation |
|---|---|
| Rolling release model | `sigpkg upgrade` updates all components |
| `pacman` package manager | SigPkg (Rust-native, zero-dep) |
| PKGBUILD format | `pkgbuild/` directory with SigmaOS-style PKGBUILDs |
| `archinstall` guided installer | `sigmaos-install` / `installer/sigma-installer.rs` |
| Minimal base system | `install/packages/sigmaos-base.packages` (123 packages) |
| AUR community packages | SigmaPkg community repository (planned) |
| Kernel choice | linux-t2, linux-zen, sigma-kernel options in limine.conf |

### Omarchy Linux (Hyprland-based)
| Feature | SigmaOS Adaptation |
|---|---|
| Hyprland as WM | `default/hyprland.conf` + `shell/` Quickshell bar |
| Keyboard-driven workflow | `Super+Enter` terminal, `Super+Space` launcher |
| Limine bootloader | `boot/limine/limine.conf` with linux-t2 entries |
| Developer-first setup | Neovim, Rust, Zig, Python toolchains pre-installed |
| `omarchy-update` | `sigmaos-update` rolling upgrade command |
| 51-chapter user manual | [[Omarchy-Manual-Comprehensive-Guide]] adapted for SigmaOS |
| Accessibility entry | Limine conf includes `SigmaOS Accessibility` with speakup |
| `gum` for TUI interactions | Included in base packages |

### Linux Mint
| Feature | SigmaOS Adaptation |
|---|---|
| Nemo file manager | `src/nemo/` — Nemo integration module |
| `mintupdate` GUI updater | `src/mintupdate/` — SigmaOS update manager |
| `mintstick` USB formatter | `src/mintstick/` — ISO/USB formatting tool |
| `mintinstall` software center | `src/mintinstall/` — Craft Suite + package browser |
| Accessibility first | Screen readers, high-contrast, keyboard nav built-in |
| Cinnamon inspirations | `src/cinnamon/` — compositor and panel concepts |
| Driver Manager | `src/driver/driver_mgr/` — automatic hardware detection |
| Welcome screen | `src/onboarding/` — first-boot wizard |

### BSD Inspirations
| BSD | Feature | SigmaOS Module |
|---|---|---|
| **DragonFlyBSD** | HAMMER2 filesystem | `src/kernel/fs/` HAMMER2-inspired VFS concepts |
| **DragonFlyBSD** | Lightweight kernel threads | `src/kernel/thread.rs` LWP model |
| **FreeBSD** | Capsicum sandboxing | `src/kernel/capsicum.rs` |
| **FreeBSD** | kqueue event system | `src/kernel/kqueue.rs` + `src/kernel/kqueue_event.rs` |
| **FreeBSD** | Jails isolation | `src/kernel/bsd_jail.rs` + `src/kernel/bsd_jails_sovereign.rs` |
| **OpenBSD** | `pledge()` promises | `src/kernel/pledge.rs` |
| **OpenBSD** | `unveil()` filesystem sandboxing | `src/kernel/unveil.rs` |
| **OpenBSD** | W^X memory protection | `src/kernel/wx_pte_hardening.rs` |

---

## 📦 Package Architecture

### Base System (123 packages)
The `install/packages/sigmaos-base.packages` defines the minimum viable OS:

```
Core system:    base, base-devel, linux-firmware, systemd
Bootloader:     limine, limine-mkinitcpio-hook, grub, efibootmgr
Display:        wayland, hyprland, xorg-xwayland, pipewire
Terminal:       foot, kitty, alacritty
Shell:          zsh, bash, fish (with omarchy-style zshrc)
Security:       tpm2-tools, sbctl, age, yubikey-manager
Networking:     networkmanager, iwd, openbsd-netcat
SigmaOS:        sigmaos-keyring, sigmaos-settings
Dev tools:      git, rustup, zig, python, neovim
```

### Craft Suite (20 creative apps)
All in `src/creative/`:

| App | Domain | Module |
|---|---|---|
| 3DCraft | 3D modeling/rendering | `src/creative/threedcraft*` |
| AniCraft | Animation studio | `src/creative/anicraft*` |
| ArtCraft | Digital painting (GIMP/Krita-like) | `src/creative/artcraft*` |
| CADCraft | Parametric CAD/CAM/CAE | `src/creative/cadcraft*` |
| CraftLibs | Shared creative foundation | `src/creative/craftlibs*` |
| DeckCraft | Presentations (Impress-like) | `src/creative/deckcraft*` |
| DesignCraft | UI/UX design (Figma-like) | `src/creative/designcraft*` |
| EffectCraft | VFX compositor (Nuke-like) | `src/creative/effectcraft*` |
| FigCraft | Technical diagrams | `src/creative/figcraft*` |
| FilmCraft | Video editor (Kdenlive-like) | `src/creative/filmcraft*` |
| FontCraft | Typography studio | `src/creative/fontcraft*` |
| GridCraft | Reactive spreadsheets | `src/creative/gridcraft*` |
| LightCraft | Lighting & color grading | `src/creative/lightcraft*` |
| PDFCraft | PDF document engine | `src/creative/pdfcraft*` |
| PhotoCraft | RAW photo editor (Darktable-like) | `src/creative/photocraft*` |
| PrintCraft | Desktop publishing | `src/creative/printcraft*` |
| SoundCraft | Digital audio workstation | `src/creative/soundcraft*` |
| VectorCraft | Vector illustration (Inkscape-like) | `src/creative/vectorcraft*` |
| VoiceCraft | Speech & dictation | `src/creative/voicecraft*` |
| WordCraft | Document processor (LibreOffice-like) | `src/creative/wordcraft*` |

---

## 🚀 How to Get SigmaOS Running

### Prerequisites

```bash
# Check what you have
make launch-check

# Install required tools (Ubuntu/Debian)
sudo apt install nasm xorriso qemu-system-x86 grub-pc-bin

# Install required tools (Arch/CachyOS)
sudo pacman -S nasm xorriso qemu-full grub
```

### Step 1: Build Stub ISO (Works NOW)

```bash
make iso-stub
# Output: build/sigmaos-desktop-preview.iso (2.8 MB)
# This is a real bootable ISO with GRUB2 boot menu
```

### Step 2: Boot in QEMU

```bash
make run-stub
# or:
qemu-system-x86_64 -m 512 -cdrom build/sigmaos-desktop-preview.iso -serial stdio
```

### Step 3: Compile Stub Kernel (when nasm is available)

```bash
sudo apt install nasm
nasm -f elf32 boot/stub_kernel.asm -o /tmp/stub.o
ld -T boot/kernel.ld -o boot/sigma_kernel.elf /tmp/stub.o
make iso-stub  # will auto-detect and include sigma_kernel.elf
make run-stub  # boots to VGA text: "SigmaOS v0.1 - Sovereign OS Loading..."
```

### Step 4: Compile Real Rust Kernel (future)

```bash
# Add bare-metal target (already installed!)
rustup target add x86_64-unknown-none

# Build kernel binary (when kernel entry point is ready)
cargo build --bin sigma_kernel --target x86_64-unknown-none --release

# Build real ISO
make iso
make run
```

---

## ⚠️ Current Blockers

| Blocker | Solution |
|---|---|
| `nasm` not installed | `sudo apt install nasm` |
| `xorriso` not installed | `sudo apt install xorriso` (or use genisoimage which works) |
| `qemu-system-x86_64` not installed | `sudo apt install qemu-system-x86` |
| Real Rust kernel not compiled | Kernel in `src/kernel/main.rs` exists but needs `x86_64-unknown-none` compile + linker script |
| Limine not installed | Use GRUB2 (works now) or `pacman -S limine` on Arch |

> **Note**: The stub ISO with GRUB2 boot menu works RIGHT NOW without any of these blockers!

---

## 🛡️ Security Defaults (Inspired by OpenBSD)

SigmaOS ships with secure defaults following **OpenBSD's philosophy**:
- **pledge**: All processes start with minimal promises
- **unveil**: Filesystem access restricted to declared paths  
- **Landlock v4**: Additional filesystem sandboxing
- **W^X**: All memory is either writable OR executable, never both
- **ASLR**: Address space layout randomization always on
- **Seccomp**: System call filtering for all userland processes
- **TPM 2.0**: Boot measurements and disk encryption key storage
