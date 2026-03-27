# Σ SIGMAOS: SUGGESTIONS & MISSING FEATURES
## Compared to Industry-Standard Linux Distros & Competitor OS
### Version 5.0 — Updated 27 March 2026

---

## 🔴 CRITICAL — Not Yet Working / Needs Implementation

| # | Feature | Status | Comparison |
|---|---------|--------|------------|
| 1 | **Actual Hardware Driver Loading** — Drivers are declared but not loaded at boot | ⛔ Scaffold | Ubuntu/Fedora: Full udev + modprobe pipeline |
| 2 | **Filesystem Journaling** — VFS exists but no crash-recovery journaling | ⛔ Missing | ext4/btrfs: Full journaling with fsck |
| 3 | **Network Stack (TCP/IP)** — Socket structs exist but no actual packet I/O | ⛔ Scaffold | Linux: Full netfilter + iptables + TCP stack |
| 4 | **Process Scheduler** — Round-robin declared but no actual time-slicing | ⛔ Scaffold | Linux: CFS, RT scheduler, SCHED_DEADLINE |
| 5 | **Real Memory Management** — mmap syscall wrapper exists but no page table walking | ⛔ Partial | Linux: Full buddy allocator + slab + NUMA |
| 6 | **USB / PCI Device Enumeration** — PCI scanner struct exists, needs MMIO | ⛔ Scaffold | Linux: Full PCI-e / USB xHCI enumeration |
| 7 | **Display Server / Compositor** — Framebuffer stubs exist but no actual rendering | ⛔ Scaffold | X11/Wayland: Full compositing with GPU accel |
| 8 | **Audio Subsystem** — HAL declared but no ALSA/PulseAudio equivalent | ⛔ Scaffold | Linux: ALSA kernel driver + PipeWire |
| 9 | **Bootloader → Kernel Handoff** — ASM boot stub exists but no GDT/IDT real init | ⛔ Scaffold | GRUB2: Full multiboot compliant handoff |
| 10 | **Package Manager** — App store UI exists but no actual package install/remove | ⛔ Scaffold | apt/pacman/dnf: Full dependency resolution |

---

## 🟡 PARTIAL — Needs Enhancement

| # | Feature | Current State | Needed |
|---|---------|---------------|--------|
| 11 | **Camera App** | v7.0 with OOP + Scratch blocks (just fixed) | HAL stubs need actual V4L2/DirectShow integration |
| 12 | **Browser Core** | HTML renderer scaffold | Needs CSS parser, JS engine (or embed Servo) |
| 13 | **Terminal Emulator** | Print-only via sigma_printf | Needs actual PTY allocation, VT100 escape codes |
| 14 | **File Explorer** | Static directory listing | Needs real VFS traversal, drag-drop, icon view |
| 15 | **IDE (sigma_ide.html)** | Syntax highlighting HTML | Needs actual LSP integration, file save to VFS |
| 16 | **Calculator** | HTML/JS functional | Needs native C++ backend for sovereign mode |
| 17 | **Clock/Calendar** | HTML functional | Needs RTC hardware sync for bare-metal |
| 18 | **Settings Panel** | HTML UI | Needs actual sysctl-equivalent configuration backend |
| 19 | **Bharat Legal Suite** | Tax/Labour/BNS calculators functional (JS) | Needs live sync with government gazette APIs |
| 20 | **Bharat Procedural Matrix** | Interactive checklist UI (JS) | Needs backend case management database |

---

## 🟢 WORKING — Industry Competitive

| # | Feature | Status |
|---|---------|--------|
| 21 | **SigmaLibC** — Full sovereign libc (string, memory, I/O, printf) | ✅ Complete |
| 22 | **SigmaOOP.hpp** — Smart pointers, Array, Map, String, Object hierarchy | ✅ Complete |
| 23 | **Makefile** — v8.0 multi-arch zero-dependency build system | ✅ Complete |
| 24 | **Bootloader ASM** — x86_64 real→long mode transition | ✅ Structural |
| 25 | **Distro Absorber Framework** — Arch/Debian/Alpine/Gentoo/NixOS | ✅ Complete |
| 26 | **Automation Subsystem** — Omni Automator + Extensions + Scripts + Pipeline | ✅ Complete |
| 27 | **Sovereign CI/CD** — GitHub Actions runner, build verification | ✅ Complete |
| 28 | **HTML Userland Apps** — 33+ functional web-based applications | ✅ Complete |
| 29 | **Legal Tools** — Indian salary/tax/labour calculators | ✅ Complete |
| 30 | **USER_MANUAL.md** — 857KB comprehensive OS guide | ✅ Complete |

---

## 🔵 NEW FEATURE SUGGESTIONS

### OS Core Enhancements

| # | Suggestion | Priority | Inspiration |
|---|-----------|----------|-------------|
| 31 | **SigmaFS** — Implement a basic read/write filesystem (FAT32 or custom) | 🔴 High | Linux ext2/FAT |
| 32 | **SigmaNet** — Minimal TCP/IP stack with ARP, ICMP, TCP, UDP | 🔴 High | lwIP, smoltcp |
| 33 | **SigmaDisplay** — Linear framebuffer compositor with window management | 🔴 High | Wayland/Weston |
| 34 | **SigmaShell** — Interactive command-line with history, pipes, redirection | 🔴 High | bash/zsh |
| 35 | **SigmaInit** — Init system (service management, runlevels) | 🟡 Med | systemd/OpenRC |
| 36 | **SigmaDevTree** — Device tree parser for hardware enumeration | 🟡 Med | Linux devicetree |
| 37 | **SigmaDebug** — GDB stub for remote kernel debugging | 🟡 Med | KGDB |
| 38 | **SigmaCrypto** — Native AES-256, SHA-256, RSA without OpenSSL | 🟡 Med | BearSSL/libsodium concepts |
| 39 | **SigmaTest** — Unit test framework (assertions, test runners) | 🟢 Low | Google Test concepts |
| 40 | **SigmaLog** — Kernel ring buffer logger (dmesg equivalent) | 🟢 Low | printk/dmesg |

### Application Enhancements

| # | Suggestion | Priority | Inspiration |
|---|-----------|----------|-------------|
| 41 | **Video Player** — Native media decoder (H.264 software decode) | 🟡 Med | VLC/mpv |
| 42 | **PDF Viewer** — Render PDF documents natively | 🟡 Med | MuPDF |
| 43 | **Spreadsheet App** — Grid-based data entry with formulas | 🟡 Med | LibreOffice Calc |
| 44 | **Email Client** — SMTP/IMAP native client | 🟡 Med | Thunderbird |
| 45 | **Voice Assistant** — TTS/STT via SigmaAI neural net | 🟢 Low | Siri/Google Asst |

### Automation & Data Science

| # | Suggestion | Priority | Inspiration |
|---|-----------|----------|-------------|
| 46 | **SigmaML** — Extend PureC NeuralNet with backprop training | 🟡 Med | TinyML |
| 47 | **SigmaPlot** — Native graph plotting (SVG output from C++) | 🟡 Med | matplotlib concepts |
| 48 | **SigmaCSV** — CSV parser/writer for data pipelines | 🟢 Low | pandas concepts |
| 49 | **SigmaJSON** — Native JSON parser (zero-copy) | 🟡 Med | simdjson |
| 50 | **SigmaCron** — Scheduled task execution daemon | 🟡 Med | cron/systemd-timer |

---

## 🐛 KNOWN BUGS FIXED (This Session)

| # | Bug | File | Fix |
|---|-----|------|-----|
| 1 | Camera app used raw `typedef unsigned char uint8_t` conflicting with sovereign types | `sigma_camera_sovereign.cpp` | Refactored to use `sigma_u8`/`sigma_u32` from SigmaOOP.hpp |
| 2 | Camera app did not inherit from SigmaObject (breaking OOP hierarchy) | `sigma_camera_sovereign.cpp` | Now extends SigmaObject with type_name() override |
| 3 | Camera BlockLogicNode used LIFO (stack) order instead of FIFO (queue) — Scratch uses sequential | `sigma_camera_sovereign.cpp` | Fixed to append-to-tail FIFO pipeline |
| 4 | Camera app lacked window controls (close/minimize/maximize) | `sigma_camera_sovereign.cpp` | Added WindowState struct + control methods |
| 5 | Camera app had no main() entry point — couldn't compile standalone | `sigma_camera_sovereign.cpp` | Added proper main() + start_camera_app() |

---

## 📋 LINUX DISTRO FEATURE COMPARISON

| Feature | SigmaOS | Ubuntu | Arch | Fedora | Alpine | NixOS |
|---------|---------|--------|------|--------|--------|-------|
| Custom libc | ✅ SigmaLibC | ❌ glibc | ❌ glibc | ❌ glibc | ✅ musl | ❌ glibc |
| Zero-dependency build | ✅ | ❌ | ❌ | ❌ | Partial | ❌ |
| Custom OOP framework | ✅ SigmaOOP | ❌ | ❌ | ❌ | ❌ | ❌ |
| Real kernel bootable | ⚠️ Scaffold | ✅ | ✅ | ✅ | ✅ | ✅ |
| Package manager | ⚠️ UI only | ✅ apt | ✅ pacman | ✅ dnf | ✅ apk | ✅ nix |
| Init system | ⚠️ Scaffold | ✅ systemd | ✅ systemd | ✅ systemd | ✅ OpenRC | ✅ systemd |
| Display server | ⚠️ HTML | ✅ Wayland | ✅ X11/Way | ✅ Wayland | ✅ X11 | ✅ X11/Way |
| Browser | ⚠️ Scaffold | ✅ Firefox | ✅ | ✅ Firefox | ❌ | ✅ |
| Legal tools suite | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Indian compliance | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Integrated IDE | ✅ HTML | ❌ (install) | ❌ | ❌ | ❌ | ❌ |
| AI/ML native | ✅ PureC NN | ❌ (install) | ❌ | ❌ | ❌ | ❌ |

---

*This document is auto-maintained by SigmaOS Sovereign CI. Last sync: 27 March 2026.*
