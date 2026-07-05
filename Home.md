# 🛡️ SigmaOS — Official Wiki

> **SigmaOS** is a sovereign, zero-dependency, AI-native operating system built entirely in Rust. It replaces POSIX legacy assumptions with a hyper-secure, capability-based microkernel designed for an AI-first, object-oriented future.

---

## 🚀 Quick Navigation

| Category | Description | Key Pages |
|---|---|---|
| 🏗️ **Architecture** | Kernel, IPC, memory, VFS | [Architecture](SigmaOS_Architecture.md) · [Subsystem-Map](Subsystem-Map.md) · [VFS](VFS.md) |
| 📦 **Package System** | sigpkg, repos, WASM | [sigpkg Spec](sigpkg-Spec.md) · [Package Manager](Sovereign-Package-Manager.md) |
| 🖥️ **Zenith Desktop** | UI, compositor, shell | [Zenith UI Architecture](Zenith-UI-Architecture.md) · [Zenith Desktop](Zenith-Desktop.md) · [Zenith SDK](Zenith-Desktop-SDK.md) |
| 🔒 **Security** | Sandboxing, IDS, crypto | [Security Framework](Sovereign-Security-Framework.md) · [Security Roadmap](Security_Roadmap.md) |
| 🧠 **AI & Automation** | Local LLM, agents, logic | [AI DataScience](AI_DataScience.md) · [AI Frameworks](AI_Frameworks.md) |
| 📘 **Education** | CBSE tools, algorithms | [Education Tools](Education_Tools.md) · [CS Education](CS_Education.md) |
| 🌍 **Localization** | Indic languages, India | [Indian Professional Tools](Indian_Professional_Tools.md) · [Indian Localization](Indian_Localization.md) |
| 📈 **Roadmaps** | Phase-wise plans | [Future Roadmap](Future-Roadmap.md) · [Development Roadmap](DEVELOPMENT_ROADMAP.md) |
| 🤝 **Contributing** | Get involved | [Contributing](CONTRIBUTING.md) · [Contributor Roadmap](Contributor-Roadmap.md) |

---

## 📊 Implementation Status at a Glance

```
Kernel Subsystems        ████████████████████ 100% (15+ modules)
Zenith Desktop UI        ██████████████████░░  90% (4 components)
AI / LLM Integration     ████████████████░░░░  80% (3 modules)
Package Manager          ████████████░░░░░░░░  60% (in progress)
Education Modules        ████████████░░░░░░░░  60% (2 modules)
Security Daemons         ████████████████░░░░  80% (2 modules)
Professional Tools       ████████░░░░░░░░░░░░  40% (planned)
Bootloader / UEFI        ████░░░░░░░░░░░░░░░░  20% (planned)
```

---

## 🗺️ Future Roadmap Summary

### Phase 1: Core & Hardware *(Month 0–4)*
Bootable ISO on real hardware, UEFI verified boot, OpenZFS/Btrfs integration, VirtIO driver suite, Intel SGX secure enclaves, Firecracker micro-VM support.

### Phase 2: Packages & UI Customization *(Month 4–8)*
Universal `sigpkg` registry, WASM/WASI app sandbox, i3/AwesomeWM tiling, picom compositor effects, rofi AI launcher, multi-monitor support.

### Phase 3: Security Hardening *(Month 8–12)*
Zeek network profiling, GnuPG package signing, fail2ban auto-blocking, Lynis audit rules, TPM2 measured boot, WireGuard native VPN.

### Phase 4: AI & Data Science *(Month 12–16)*
Offline Whisper voice shell, mlpack/OpenCog cognitive engines, DVC/MLflow telemetry versioning, n8n visual automation, Spark-style IPC aggregation.

### Phase 5: Indian Localization & Professional Suites *(Month 16–20)*
Indic transliteration, Bharat-FOSS/OpenForge SDKs, QGIS crop yield tools, GST/TDS calculators, ERPNext one-click deployment, offline CBSE curriculum.

> 📄 **Full details:** [Future-Roadmap.md](Future-Roadmap.md)

---

## 🔑 Core Principles

1. **Zero dependency** — no external crates, no `alloc`, pure `no_std` Rust kernel
2. **Capability-based security** — 64-bit hardware-enforced tokens replace legacy ACLs
3. **AI-native** — local LLM inference is a first-class OS primitive
4. **Sovereign** — no telemetry, no cloud lock-in, all data stays on-device
5. **India-first** — multilingual UI, regional tools, and Gov SDK pre-installed

---

## 📚 All Wiki Categories

### Core System
[Architecture](SigmaOS_Architecture.md) · [Subsystems](Subsystems.md) · [Kernel Internals](SigmaOS-Kernel-Internals.md) · [IPC](Syscall-Dispatcher.md) · [VFS](VFS.md) · [TCP Stack](TCP-Stack.md) · [HAL](Subsystem-Map.md)

### Package & Ecosystem
[sigpkg Spec](sigpkg-Spec.md) · [Package Manager](Sovereign-Package-Manager.md) · [WASM Runtime](WASM-Runtime.md) · [Flatpak Distribution](Flatpak-AppDistribution.md)

### Desktop & UI
[Zenith UI Architecture](Zenith-UI-Architecture.md) · [Zenith Desktop](Zenith-Desktop.md) · [Zenith SDK](Zenith-Desktop-SDK.md) · [Developer Guide](Zenith-Developer-Guide.md) · [UI/UX Improvements](UI_UX_Improvements.md) · [Window Managers](Window_Managers.md)

### Security & Privacy
[Security Framework](Sovereign-Security-Framework.md) · [Security Roadmap](Security_Roadmap.md) · [Cybersecurity Tools](Cybersecurity_Tools.md) · [Verified Boot](Verified-Boot.md) · [Sandbox Spec](SOVEREIGN_SANDBOX_SPEC.md)

### AI & Automation
[AI DataScience](AI_DataScience.md) · [AI Frameworks](AI_Frameworks.md) · [OS Observability](OS-Observability-and-Math.md) · [SigmaAI Agent](SigmaAI-Agent-CLI-Integration.md)

### Education
[Education Tools](Education_Tools.md) · [CS Education](CS_Education.md) · [Syllabi Index](Syllabus-Implementation-Map.md)

### Performance & Optimization
[Performance Enhancements](Performance_Enhancements.md) · [Advanced Performance](Advanced_Performance.md) · [Stability Playbook](Stability-Playbook.md)

### Professional & Creative
[Creative Tools](Creative_Tools.md) · [Creative Suite](Creative_Suite.md) · [Indian Professional Tools](Indian_Professional_Tools.md) · [Indian Localization](Indian_Localization.md)

### Roadmaps
[Future Roadmap](Future-Roadmap.md) · [Development Roadmap](DEVELOPMENT_ROADMAP.md) · [System Improvement Plan](System-Improvement-Plan.md)

### Contributing
[Contributing Guide](CONTRIBUTING.md) · [Contributor Roadmap](Contributor-Roadmap.md) · [Coding Standards](Coding-Standards.md) · [Testing Guide](Testing-Guide.md)

---

*Last Updated: July 2026 — SigmaOS Project*
