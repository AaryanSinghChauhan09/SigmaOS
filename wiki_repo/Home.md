# SigmaOS Wiki

> **v15.0.0 Zenith — Stable** · One branch (`main`) · PQC-signed · Multi-format

---

## What is SigmaOS?

SigmaOS is a sovereign, multi-format operating system built from a single unified codebase.
It ships in **50+ distribution formats** — from a bare-metal RTOS to a browser-tab WASM app —
all signed with post-quantum cryptography (Kyber-1024 + Dilithium-5).

> *The only OS that boots on bare metal, runs in a browser, deploys as a cloud container,
> and installs as a mobile APK — all from one codebase.*

---

## 🚀 Start Here

| I want to… | Go to |
|---|---|
| **Download SigmaOS** | [DOWNLOAD.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/DOWNLOAD.md) · [download.html](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/download.html) |
| **Run it in QEMU right now** | [Quick Start](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/QUICKSTART.md) |
| **Understand the architecture** | [Architecture Overview](Architecture-Overview) |
| **Build the bootable ISO (v0.1)** | [Minimal v0.1 Spec](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Minimal_SigmaOS_v0.1.md) |
| **Contribute code** | [Contributing](Contributing) · [Developer Guide](Developer_Guide) |
| **Build an app** | [SDK Guide](SDK-Guide) · [Your First App](Your-First-App) |
| **Add a driver** | [Driver Development](Driver-Development) · [Open Source Drivers](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Open_Source_Drivers.md) |
| **Understand the roadmap** | [ROADMAP.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/ROADMAP.md) |
| **Compare to other distros** | [Competitive Analysis](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Competitive_Analysis.md) |

---

## 📦 Download Formats

| Category | Formats |
|---|---|
| App | Native, Electron, Java, .NET, Python, AppImage/Snap/Flatpak, WASM, Mobile, ELF, sigpkg |
| Standalone | Native ISO, AppImage, Portable EXE, WASM Bundle |
| RTOS | Monolithic, Microkernel, Layered, Exokernel, POSIX Layer, Bare-Metal |
| Mobile | APK, IPA, Hybrid, Cross-Platform, PWA |
| Microkernel | Pure, Hybrid, Modular, Exokernel, POSIX Layer |
| Dual Boot | Partition, Separate Disk, Chainload, Virtualized, Live USB |
| Distributed | Client-Server, P2P, Cluster, Grid, SOA, Ledger, Actor |
| Cloud | Public, Private, Hybrid, Multi, Community, IaaS, PaaS, SaaS, FaaS |
| Browser | Desktop, Mobile, WebViews, Headless, Lite, Specialised |
| Kernel | Monolithic, Microkernel, Hybrid, Exokernel, Nanokernel, Modular, Mono+Modular |

---

## 🗺️ Roadmap at a Glance

| Phase | Version | Goal | Status |
|---|---|---|---|
| 1 | **v0.1** | Bootable ISO + sigma-sh + sigma-pkg | 🔴 Building |
| 2 | v1.0 | Desktop + AppImage + 50 packages + SDK | ⬜ Planned Q2 2027 |
| 3 | v2.0 | Mobile + WASM + Cloud images | ⬜ Planned Q4 2027 |
| 4 | v3.0 | RTOS + Distributed + Formal verification | ⬜ Planned Q2 2028 |

---

## 🏗️ System Architecture

```
User Space     → PWAs · Zenith Desktop · sigma-ai · profession apps
Browser Shell  → Custom Chromium + navigator.sigmaos.* API
System Daemons → sigmad-health · sigmad-pkg · sigmad-netd · sigmad-vault
Syscall Layer  → sigma_pledge + sigma_unveil + seccomp-BPF + AVC
Kernel (Ring 0)→ MLFQ+EDF+CFS Scheduler · Buddy+Slab MM · PQC Security
                 TCP/IP+TLS1.3+Kyber · VFS+SigmaFS+Ext4 · IPC · eBPF
HAL            → x86_64 · ARM64 · RISC-V RV64GC
Hardware       → CPU · NVMe · GPU · NIC · USB · TPM2 · UEFI
```

---

## 🧩 Ecosystem

- **600+ shards** — atomic, independently testable capability modules
- **sigma-pkg** — PQC-signed package manager with reproducible builds
- **sigma-sdk** — multi-language SDK (Rust, JS/TS, Python, Java, .NET)
- **Zenith Desktop** — glassmorphic DE with `navigator.sigmaos.*` web API
- **sigma-vault** — TPM2-backed secrets store
- **sigma-pod** — OCI-compatible container runtime
- **sigma-ai** — on-device TinyLlama inference daemon

---

## 🔒 Security Pillars

1. **Post-Quantum Cryptography** — Kyber-1024 KEM + Dilithium-5 signatures
2. **sigma_pledge / sigma_unveil** — kernel-enforced capability restriction
3. **Zero-Trust** — SPIFFE workload identities, per-syscall attestation
4. **TPM2** — sealed key derivation, remote attestation
5. **W^X** — no page simultaneously writable and executable
6. **Reproducible Builds** — cryptographically verifiable binaries

---

## 📚 Key Documents

| Document | Description |
|---|---|
| [Architecture Overview](Architecture-Overview) | System layers, subsystems, deployment profiles |
| [Professional Tools & Apps](Professional-Tools-And-Apps) | Full app and tool ecosystem |
| [sigpkg Specification](sigpkg-Spec) | Package format, registry, PKGBUILD |
| [SDK Guide](SDK-Guide) | Build apps in Rust, JS, Python, Java, .NET |
| [Component Integration](Component-Integration) | GitHub org structure, component contracts |
| [OSS Reference Map](OSS-Reference-Map) | What to study from seL4, Nix, Smithay, smoltcp… |
| [Ideas Backlog (1000+)](Ideas-Backlog-1000) | Development ideas across 8 categories |
| [Open Source Drivers](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Open_Source_Drivers.md) | Driver strategy, SDF guide |
| [Hardware CI Matrix](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Hardware_CI_Matrix.md) | QEMU + real HW test matrix |
| [License Map](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/License_Map.md) | Per-directory SPDX licensing |
| [Competitive Analysis](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Competitive_Analysis.md) | vs Alpine, Arch, Ubuntu |
| [Minimal v0.1 Spec](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Minimal_SigmaOS_v0.1.md) | First bootable ISO checklist |
| [Security Model](Security-Model) | PQC, pledge/unveil, zero-trust |
| [Profession Profiles](PROFILES) | 1000+ role-specific shard bundles |

---

*SigmaOS — Sovereign by Design. One codebase. Every format.*
*GitHub: [AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)*
