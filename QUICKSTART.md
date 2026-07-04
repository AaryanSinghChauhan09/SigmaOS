# SigmaOS Quick Start

> New here? Start here. This gets you from zero to running SigmaOS in under 10 minutes.

---

## Option A — Run the QEMU Demo (works today)

SigmaOS v15.0 has a kernel stub you can build and run in QEMU right now.

```bash
# 1. Install prerequisites (Ubuntu/Debian)
sudo apt install -y build-essential nasm cmake qemu-system-x86 \
  golang-go xorriso mtools grub-pc-bin grub-efi-amd64-bin

# 2. Clone
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# 3. Build
make clean && make all -j$(nproc)

# 4. Run
qemu-system-x86_64 -cdrom build/sigmaos.iso -m 2G -serial stdio
```

> **Note:** The current build is a kernel stub — it boots to early init output.
> A full interactive shell (`sigma-sh`) is coming in v0.1.
> Track progress: [docs/Minimal_SigmaOS_v0.1.md](docs/Minimal_SigmaOS_v0.1.md)

---

## Option B — Run the Web Desktop Demo (works today)

No install needed. Open `index.html` in your browser:

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Open the Zenith Desktop prototype
start index.html        # Windows
open index.html         # macOS
xdg-open index.html     # Linux
```

You'll see: the full Zenith Desktop UI, OmniShell terminal, file manager,
telemetry HUD — all running as a web app.

---

## Option C — Browse the Download Page

All 50+ distribution formats are documented at:

```
download.html   ← open this in your browser
```

Formats include: Native ELF, Electron, AppImage, WASM, APK/IPA, Docker/OCI,
RTOS images, Cloud QCOW2, and more.

---

## Build Profiles

Once the v0.1 bootable ISO ships, you can build any profile:

```bash
make PROFILE=standalone    all -j$(nproc)   # Full Zenith Desktop
make PROFILE=minimal       all -j$(nproc)   # v0.1 shell-only
make PROFILE=microkernel   all -j$(nproc)   # <512KB kernel
make PROFILE=cloud         all -j$(nproc)   # Headless cloud
make PROFILE=rtos          all -j$(nproc)   # Hard real-time
make PROFILE=mobile ARCH=arm64 all -j$(nproc)  # ARM64
make PROFILE=browser       all -j$(nproc)   # WASM bundle
```

---

## Want to Contribute?

The highest-impact tasks right now are:

| Task | Skill | File |
|------|-------|------|
| Round-robin scheduler | C++ / Rust (no_std) | `kernel/core/sigma_sched.cpp` |
| Buddy allocator | C++ / Rust (no_std) | `kernel/core/sigma_mm.cpp` |
| sigma-sh REPL | Rust | `userland/shell/sigma_shell.cpp` |
| sigma-pkg local mode | Rust | `userland/pkg/sigma_registry.cpp` |
| USB HID keyboard | Rust (driver) | `drivers/input/sigma_hid.rs` |
| VESA framebuffer | C / Rust | `drivers/display/sigma_vesa.cpp` |
| Package recipes | Any | `packages/` (new sigpkg specs) |
| Wiki pages | Markdown | `wiki_repo/` |

Read [CONTRIBUTING.md](CONTRIBUTING.md) for technical mandates (Rust, no_std, no third-party crates).

---

## Key Docs

| Document | What it covers |
|----------|---------------|
| [README.md](README.md) | Project overview + download links |
| [ROADMAP.md](ROADMAP.md) | Phase 1→4 execution plan |
| [DOWNLOAD.md](DOWNLOAD.md) | All 50+ format tables + build flags |
| [Architecture.md](Architecture.md) | System layers, subsystems, directory map |
| [INSTALL.md](INSTALL.md) | QEMU demo + build profiles + troubleshooting |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Code standards, PR process, CI requirements |
| [docs/Minimal_SigmaOS_v0.1.md](docs/Minimal_SigmaOS_v0.1.md) | v0.1 bootable ISO spec |
| [docs/Competitive_Analysis.md](docs/Competitive_Analysis.md) | How SigmaOS compares to Alpine/Arch/Ubuntu |
| [docs/Open_Source_Drivers.md](docs/Open_Source_Drivers.md) | Driver strategy + SDF guide |
| [STRATEGIC_VISION.md](STRATEGIC_VISION.md) | Long-term vision and positioning |

---

## Community

- **GitHub Issues** → [Report bugs / request features](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- **GitHub Discussions** → [Ask questions / share ideas](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
- **Wiki** → [Full documentation](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

---

*SigmaOS — Sovereign by Design. One codebase. Every format.*
