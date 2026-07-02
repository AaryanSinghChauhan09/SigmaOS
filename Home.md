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
| **Download SigmaOS** | [DOWNLOAD.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/DOWNLOAD.md) |
| **Run it in QEMU right now** | [Quick Start](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/QUICKSTART.md) |
| **Understand the architecture** | [Architecture Overview](Architecture-Overview) |
| **Use the AI CLI agent** | [sigma-agent](sigma-agent) |
| **Automate workflows** | [sigma-agent-workflow](sigma-agent-workflow) |
| **Migrate from Linux** | [Migration Guide](Migration-Guide) |
| **Build an app** | [SDK Guide](SDK-Guide) · [Your First App](Your-First-App) |
| **Add a driver** | [Driver Development](Driver-Development) |
| **Contribute code** | [Community Governance](Community-Governance) · [Developer Guide](Developer_Guide) |
| **Compare to other distros** | [SigmaOS vs Linux](SigmaOS-vs-Linux) |
| **Understand the roadmap** | [ROADMAP.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/ROADMAP.md) |

---

## 🤖 AI Agent (sigma-agent)

sigma-agent is SigmaOS's built-in AI CLI assistant — 36 modules, 22 subcommands.

```bash
sigma-agent                          # interactive REPL
sigma-agent "install sigma-edit"     # one-shot NL command
sigma-agent "set dark mode"          # any GUI action via NL
sigma-agent doctor                   # health check
sigma-agent daemon start             # background AI service
sigma-agent workflow install --all   # install automation templates
sigma-agent security scan            # security audit
```

Every GUI action has a CLI equivalent. Full docs: [sigma-agent](sigma-agent) · [Workflow Automation](sigma-agent-workflow)

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
| [sigma-agent](sigma-agent) | AI CLI agent — 36 modules, 22 subcommands |
| [sigma-agent-workflow](sigma-agent-workflow) | n8n-style workflow automation |
| [Migration Guide](Migration-Guide) | Moving from Ubuntu/Fedora/Arch to SigmaOS |
| [SigmaOS vs Linux](SigmaOS-vs-Linux) | Feature-by-feature comparison |
| [Linux Absorption Architecture](Linux-Absorption-Architecture) | Running Linux apps on SigmaOS |
| [SDK Guide](SDK-Guide) | Build native SigmaOS apps |
| [Community Governance](Community-Governance) | Contributor roles, RFC process, voting |
| [Security Model](Security-Model) | PQC, pledge/unveil, zero-trust |
| [Professional Tools & Apps](Professional-Tools-And-Apps) | Full app and tool ecosystem |
| [sigpkg Specification](sigpkg-Spec) | Package format and registry |
| [OSS Reference Map](OSS-Reference-Map) | Inspirational open-source projects |
| [Ideas Backlog (1000+)](Ideas-Backlog-1000) | Development ideas backlog |
| [Profession Profiles](PROFILES) | 1000+ role-specific shard bundles |

---

*SigmaOS — Sovereign by Design. One codebase. Every format.*
*GitHub: [AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)*
