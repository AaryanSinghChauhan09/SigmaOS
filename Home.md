# SigmaOS

> **v15.0.0 Zenith — Stable Release** · All branches unified on `main`
>
> 📥 **[Download SigmaOS](download.html)** — 50+ formats: App · Standalone · RTOS · Mobile · Microkernel · Dual Boot · Distributed · Cloud · Browser · Kernel
>
> 🚀 **[Quick Start →](QUICKSTART.md)** · 📋 **[Roadmap →](ROADMAP.md)** · 🔍 **[Competitive Analysis →](docs/Competitive_Analysis.md)**

SigmaOS is a next-generation operating system built on the principles of stability, hardware support, modern filesystems, robust package management, and formal security.

It draws inspiration from the best aspects of various Linux distributions:
- **Debian‑style stability** → predictable releases.
- **Fedora‑style innovation** → cutting‑edge drivers/security.
- **Arch‑style flexibility** → modular FS and userland.
- **Ubuntu‑style ecosystem** → strong community and package management.

## Roadmap

### Phase 1: Core System & Stability
- Unify branches into a stable `main`.
- Kernel scheduler: finalize Round Robin/EDF into a robust, tested default.
- Memory allocator: stress‑test and formally verify.
- Syscall layer: expand non‑POSIX ABI for consistency.
- Release cadence: adopt predictable stable releases.

### Phase 2: Hardware Support
- Networking: expand NIC support beyond e1000.
- Storage: add NVMe, SSD optimizations.
- USB/HID: implement keyboard, mouse, and USB stack.
- Graphics: move from VGA framebuffer to modern GPU drivers.
- Audio: add basic sound subsystem.

### Phase 3: File Systems & Storage
- Enhance FS support: journaling, encryption, sovereign FS.
- Add modern FS equivalents: ext4‑like, btrfs‑like features.
- Virtualization drivers: VirtIO for cloud/server use cases.

### Phase 4: Package Management & Build System
- Develop `sigpkg`: sovereign package manager.
- Deterministic builds: reproducible recipes, cryptographic verification.
- Profiles: `sigma-core`, `sigma-desktop`, `sigma-cloud`.

### Phase 5: Security & Sovereignty
- Sandboxing: sovereign equivalents.
- Audit framework: syscall monitoring.
- Secure boot: expand cryptographic verification, rollback protection.
- Exploit mitigations: hardened allocators, memory safety.

### Phase 6: Userland & Ecosystem
- Expand utilities: sovereign replacements for GNU tools.
- Shell (`sigma-sh`): scripting, automation, developer ergonomics.
- SDK/toolchain: sovereign SDK for driver/app development.

### Phase 7: Community & Adoption
- Contribution workflow: PRs only into main, modular tasks.
- Wiki expansion: roadmap, coding standards, migration guides.
- Target domains: secure systems, research, silicon sovereignty.

---

## Download

SigmaOS ships in **50+ distribution formats** across 10 categories. All formats are
PQC-signed (Kyber-1024 + Dilithium-5) and built from a single unified `main` branch
via CMake profile flags.

| Category | Formats | Status |
|---|---|---|
| 📦 App | Native, Electron, Java, .NET, Python, AppImage/Snap/Flatpak, WASM, Mobile, ELF, sigpkg | Stable/Preview |
| 🖥️ Standalone | Native EXE, AppImage, Portable EXE, Electron, JAR, PyInstaller, WASM Bundle | Stable/Preview |
| ⚙️ RTOS | Monolithic, Microkernel, Layered, Exokernel, POSIX Layer, Bare-Metal | Stable/Preview |
| 📱 Mobile | APK/IPA, Hybrid, Cross-Platform, PWA, Game Engines | Stable/Preview |
| 🧩 Microkernel | Pure, Hybrid, Modular, Exokernel, POSIX Layer | Stable/Preview |
| 💻 Dual Boot | Traditional, Separate Disk, Chainload, Virtualized, Live USB | Stable/Preview |
| 🌐 Distributed | Client-Server, P2P, Cluster, Grid, SOA, Ledger, Actor Model | Stable/Preview |
| ☁️ Cloud | Public, Private, Hybrid, Multi-Cloud, Community, IaaS, PaaS, SaaS, FaaS | Stable/Preview |
| 🌍 Browser | Native Desktop, Mobile, Embedded/WebViews, Headless, Lite, Specialised | Stable/Preview |
| 🖥️ Kernel | Monolithic, Microkernel, Hybrid, Exokernel, Nanokernel, Modular, Mono+Modular | Stable/Preview |

👉 **[Open the Download Page →](download.html)**

```bash
# Quick start — clone and build any profile
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
make PROFILE=standalone all -j$(nproc)   # full desktop
make PROFILE=rtos        all -j$(nproc)   # hard real-time
make PROFILE=cloud       all -j$(nproc)   # headless cloud
make PROFILE=microkernel all -j$(nproc)   # <512KB kernel
make PROFILE=mobile ARCH=arm64 all -j$(nproc)  # ARM64 mobile
```

---
