# SigmaOS Wiki — Home

Welcome to the **SigmaOS** developer wiki, the canonical source of truth for the project.

---

## 🗺 Navigation

| Section | Description |
|---------|-------------|
| [Architecture](Architecture) | Kernel rings, memory model, driver architecture |
| [Security Model](Security-Model) | Zero-Trust engine, capability rings, PQC crypto |
| [API Reference](API-Reference) | `navigator.sigmaos` full API docs |
| [Build Guide](Build-Guide) | How to build the bootable ISO from source |
| [Daemons](Daemons) | sigmad-process, sigmad-ai, sigmad-sync reference |
| [Hardware Support](Hardware-Support) | Driver status for NVMe, GPU, Wi-Fi, Audio |
| [Package Manager](Package-Manager) | sigma-pkg and `pkg.ensure` via Alpine apk |
| [Roadmap](Roadmap) | Phase timeline and milestone tracking |
| [Contributing](Contributing) | How to contribute drivers, apps, and docs |

---

## What is SigmaOS?

SigmaOS is the world's most advanced **Sovereign AI Operating System**. It is built entirely from scratch in C++ (kernel) and Go (daemons), with a Chromium-based desktop environment powered by native `navigator.sigmaos` Web APIs.

### Core Architecture (v0.1)

```
Bootable ISO (Buildroot + GRUB)
│
├── SigmaOS Kernel (C++)
│   ├── SovereignVMM — CoW, demand paging, 4-level page tables
│   ├── Zero-Trust Engine — PAM/ACL, runtime threat scoring
│   └── Drivers — NVMe, xHCI, E1000, HDA, 802.11
│
├── Go Daemons
│   ├── sigmad-process :17382 — bwrap shell.exec, pkg.ensure, script.install
│   ├── sigmad-ai      :17383 — TinyLlama local AI (summarize, complete)
│   └── sigmad-sync    :17384 — rclone workspace autosync
│
├── Chromium + Extension
│   └── inject.js → navigator.sigmaos (fs, process, ai, workspace, system)
│
└── Web Shell (localhost:3000)
    ├── index.html      — Desktop launcher + App Store
    ├── settings/caps.html — Capability grant UI
    ├── docs.html       — API documentation
    └── apps/
        ├── notes/      — SigmaNotes (fs + ai demo)
        ├── terminal/   — SigmaTerm (process.spawn demo)
        └── paint/      — SigmaPaint (WASM + fs demo)
```

---

## Quick Start (Developers)

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS
cd SigmaOS
./build-iso.sh standalone
# ISO output: output/sigmaos-v0.1-standalone.iso

# Test in QEMU:
qemu-system-x86_64 -m 2G -cdrom output/sigmaos-v0.1-standalone.iso -boot d
```

---

## Branches

| Branch | Purpose |
|--------|---------|
| `main` | Stable development trunk |
| `release/standalone` | Desktop ISO builds |
| `release/cloud` | Cloud-native container profile |
| `release/mobile` | ARM64 / mobile profile |
| `release/rtos` | Real-time / embedded profile |
| `release/browser` | Browser-WASM profile |
| `release/dual-boot` | Dual-boot installer |
| `release/distributed` | Distributed cluster profile |
| `release/microkernel` | Microkernel research branch |
| `release/app` | App platform / PWA runtime |
| `performance-optimized` | AVX-512, hugepages, CPU tuning |
| `prepare-sigmaos-launch` | v0.1 launch assets and press kit |
| `gh-pages` | Static documentation site |
