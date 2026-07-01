# SigmaOS — Growth Roadmap

> Current: **v15.0.0 Zenith** (Stable baseline, unified `main`)
> Next milestone: **v0.1 Minimal** — the first truly bootable, installable SigmaOS

---

## The Honest Starting Point

SigmaOS has excellent architecture, documentation, and ambition.
What it does **not yet have** is a bootable ISO, a working package manager, or
out-of-the-box hardware support — the table-stakes that even the simplest distros
(Alpine, Puppy, Tiny Core) cleared years ago.

This roadmap closes that gap methodically, one phase at a time,
then layers on the sovereign differentiators that make SigmaOS worth choosing.

---

## Phase 1 — Stable Core (`v0.1 Minimal`, target: Q4 2026)

> Goal: a bootable ISO that installs, has a shell, and connects to the network.
> Beat Alpine Linux on simplicity. Match Tiny Core on size.

### Kernel (`kernel-exp` work, now on `main`)

| Task | File | Done? |
|------|------|-------|
| Round-robin scheduler (64 tasks) | `kernel/core/sigma_sched.cpp` | ⬜ |
| Buddy physical allocator | `kernel/core/sigma_mm.cpp` | ⬜ |
| Slab allocator (kmalloc) | `kernel/core/sigma_mm.cpp` | ⬜ |
| x86-64 4-level page table walker | `kernel/mm/sigma_vmm.cpp` | ⬜ |
| APIC + PIC init | `kernel/core/sigma_irq.cpp` | ⬜ |
| HPET/APIC timer → jiffies | `kernel/core/sigma_timer.cpp` | ⬜ |
| 30-syscall dispatch table | `kernel/core/sigma_syscall_dispatch.cpp` | ⬜ |
| VESA/GOP framebuffer | `drivers/display/sigma_vesa.cpp` | ⬜ |
| sigma-boot.efi UEFI loader | `sigma-boot/sigma_boot.c` | ⬜ |
| **`make iso` → bootable ISO** | `Makefile` | ⬜ |

### Drivers (minimum viable set)

| Driver | Hardware | Done? |
|--------|----------|-------|
| e1000 NIC | Intel Gigabit / QEMU virtio-net | 🔄 |
| NVMe | PCIe SSDs | ✅ |
| xHCI USB | USB 3.x | ✅ |
| VirtIO-blk | QEMU block device | ✅ |
| VESA framebuffer | All UEFI | ⬜ |
| USB HID | Keyboard + mouse | ⬜ |

### Filesystem

| Task | Done? |
|------|-------|
| VFS open/read/write/close | ⬜ |
| Tmpfs (RAM-backed) | ⬜ |
| Ext4 read-only mount | ⬜ |
| FAT32 (EFI partition) | ✅ |

### Shell & Userland

| Task | Done? |
|------|-------|
| sigma-sh: basic REPL (exec, cd, ls, cat, echo) | ⬜ |
| sigma-sh: env vars + PATH | ⬜ |
| sigma-init: PID 1, mount /proc /sys /dev | ⬜ |
| sigma-pkg: install/remove from local repo | ⬜ |
| Minimal coreutils (ls, cp, mv, rm, mkdir) | ⬜ |

### Installer

| Task | Done? |
|------|-------|
| CLI partition wizard (fdisk wrapper) | ⬜ |
| Install to disk (dd + grub-install equivalent) | ⬜ |
| Dual-boot EFI entry registration | ⬜ |
| Live USB boot (tmpfs overlay) | ⬜ |

### CI Gate

```yaml
# Must pass before v0.1 tag
- make iso                          # builds without error
- qemu-system-x86_64 -cdrom ...     # boots to sigma-sh prompt
- echo "hello" | sigma-sh           # shell executes command
- sigma-pkg install hello           # installs a test package
```

### Exit Criteria
> A user can: download ISO → boot in QEMU → type commands in sigma-sh →
> install a package with sigma-pkg → shut down cleanly.

---

## Phase 2 — Community (`v1.0`, target: Q2 2027)

> Goal: AppImage + Flatpak + Snap. Developer SDK. Enough apps to be useful daily.

### Package Ecosystem
- `sigma-pkg` online registry at `pkg.sigmaos.app`
- 50 essential packages: browser, text editor, git, curl, Python 3, Node.js
- sigpkg build spec format (`PKGBUILD`-style) + reproducible builds
- Cryptographic package signing (Dilithium-5) verified on install

### Desktop
- Zenith Desktop on real framebuffer (DRM/KMS via i915 / VirtIO-GPU)
- Auto-tiling window manager (keyboard-driven)
- Theme engine + accessibility (high contrast, screen reader stub)
- sigma-ai: TinyLlama on-device inference daemon

### Driver Coverage
- Intel i915 modesetting
- AMD amdgpu basic
- Intel iwlwifi Wi-Fi 6
- USB HID complete (keyboard, mouse, touchpad)
- HDA audio

### Distribution Formats Added
- AppImage (Linux portable)
- Flatpak via Flathub submission
- Snap submission
- Electron installer (Windows + macOS)

### Developer SDK
- sigma-sdk: compiler toolchain, headers, sigma-pkg build tool
- Electron app template + TypeScript types for `navigator.sigmaos.*`
- Python bindings (`pip install sigmaos`)
- Java bindings (JAR + Maven)
- Documentation site: `docs.sigmaos.app`

---

## Phase 3 — Expansion (`v2.0`, target: Q4 2027)

> Goal: mobile + WASM sandbox + cloud images. Expand beyond desktop.

### Mobile
- ARM64 APK (Android 12+) via sigma-mobile build target
- iOS IPA via TestFlight
- Cross-platform via React Native + sigma-rn plugin
- PWA installable from browser

### WASM Sandbox
- Full sigma kernel compiled to WASM/WASI
- Runs in Chrome/Firefox/Safari — no install
- sigma-wasm npm package

### Cloud
- AWS AMI + GCE image + Azure VHD published
- OCI container image: `docker pull sigmaos/paas:2.0`
- FaaS runtime for AWS Lambda custom runtime
- Kubernetes operator for sigma-pod workloads

### Security Maturity
- sigma_pledge + sigma_unveil enforced in all userland processes
- TPM2 attestation on cloud images
- Reproducible builds verified by CI
- CVE response SLA: 72 hours for critical, 14 days for high

---

## Phase 4 — Enterprise (`v3.0`, target: Q2 2028)

> Goal: RTOS variants, distributed services, formal verification.
> Beat Fedora CoreOS on cloud. Rival VxWorks on RTOS.

### RTOS
- EDF scheduler with <10 µs IRQ latency
- ROS 2 DDS middleware port
- SovereignWCET: worst-case execution time analyser
- Bare-metal firmware images (STM32, ESP32, RP2040)

### Distributed
- SovereignConsensus (RAFT-inspired) <15 ms over GbE
- CRDT offline-first sync (sigma-cloudsync)
- Grid computing work-stealing scheduler
- Actor model runtime (sigma-bus mailbox)

### Formal Verification
- Coq proofs for microkernel memory safety
- seL4-style capability model verification
- sigma-audit: kernel-level syscall monitoring for compliance

### Governance
- RFC process for kernel changes
- LTS branch: 5-year security support
- sigma-security-advisories mailing list
- Public CVE database at `cve.sigmaos.app`

---

## What Makes SigmaOS Different (The Killer Features)

Every distro has a shell and a package manager. Here is what SigmaOS has that others
structurally cannot offer:

| Differentiator | Why Others Can't Match It |
|---|---|
| **10 distribution formats from 1 codebase** | Linux distros repackage; SigmaOS compiles to any target via CMake flags |
| **Post-quantum crypto baked in** | Kyber-1024 + Dilithium-5 in TLS, packages, boot — not bolted on |
| **WASM-native kernel** | Run SigmaOS in a browser tab — no VM, no install |
| **sigma_pledge/unveil** | OpenBSD-inspired but kernel-enforced, not just advisory |
| **AI-predictive scheduler** | TinyLlama pre-warming for hot code paths (Phase H) |
| **Profession profiles** | 1000+ role-specific shard bundles — AI Researcher to Aerospace Engineer |
| **Sovereign identity (SPIFFE DIDs)** | Per-process cryptographic identity, not just UIDs |
| **sigpkg reproducible builds** | Deterministic, hash-verified — not "probably the same as last time" |

---

## The Honest Gap vs Simple Distros

| What Alpine/Puppy Has | SigmaOS Status | Fix |
|---|---|---|
| Bootable ISO | ⬜ Phase 1 | `make iso` — blocked on scheduler+MM |
| Working shell | ⬜ Phase 1 | sigma-sh REPL |
| Package manager | 🔄 Phase 1 | sigma-pkg (local repo first) |
| Kernel-integrated NIC/USB drivers | 🔄 Partial | e1000 ✅, HID ⬜ |
| Out-of-box Wi-Fi | ⬜ Phase 2 | iwlwifi |
| GUI installer | ⬜ Phase 2 | installer.html already designed |
| 10,000+ packages | ⬜ Phase 2+ | community-driven sigpkg registry |
| 5+ year LTS | ⬜ Phase 4 | governance model needed |

Closing Phase 1 alone puts SigmaOS ahead of Tiny Core on ambition
and on par with Alpine on usability. Everything after that is gravy.

---

## Contribution Priority Order

If you want to contribute, work in this order:

1. **Kernel boot** — `kernel/core/sigma_sched.cpp`, `sigma_mm.cpp`, `sigma_irq.cpp`
2. **sigma-sh** — `userland/shell/sigma_shell.cpp`
3. **sigma-pkg** — `userland/pkg/sigma_registry.cpp`
4. **Drivers** — `drivers/display/sigma_vesa.cpp`, `drivers/input/sigma_hid.rs`
5. **Installer** — `userland/installer/`
6. **Docs** — wiki pages, man pages, troubleshooting guide

See [CONTRIBUTING.md](CONTRIBUTING.md) for the technical mandates and PR process.

---

*See also: [DOWNLOAD.md](DOWNLOAD.md) · [docs/Competitive_Analysis.md](docs/Competitive_Analysis.md) · [docs/Minimal_SigmaOS_v0.1.md](docs/Minimal_SigmaOS_v0.1.md) · [STRATEGIC_VISION.md](STRATEGIC_VISION.md)*
