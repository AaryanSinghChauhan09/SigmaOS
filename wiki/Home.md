# 🚀 SigmaOS — The Post-Linux Sovereign Operating System

SigmaOS is the world’s first **sovereign operating system**, engineered from the ground up in Safe-Rust to deliver mathematical memory safety, sub-millisecond execution latency, and true computing independence beyond legacy Linux and BSD distributions.

---

## 📜 Key Project Resources & Manifesto
- 🚀 **Public Launch Announcement**: [docs/LAUNCH_ANNOUNCEMENT.md](docs/LAUNCH_ANNOUNCEMENT.md)
- 📜 **Technical Whitepaper**: [docs/WHITEPAPER.md](docs/WHITEPAPER.md)
- 📰 **Public Press Kit & Media Guide**: [docs/PRESS_KIT.md](docs/PRESS_KIT.md)
- ⚖️ **Contributor Charter & Governance**: [docs/GOVERNANCE_CHARTER.md](docs/GOVERNANCE_CHARTER.md) / [CONTRIBUTING.md](CONTRIBUTING.md)
- 🗺️ **Master Development Roadmap**: [FUTURE-DEVELOPMENT-ROADMAP.md](FUTURE-DEVELOPMENT-ROADMAP.md)

---

## 🧩 Core Architecture & Features
- **Boot to Web**: Minimal Linux (Buildroot) base, boots directly into Chromium in ~3s.
- **Browser as Shell**: Workspaces, window management, and hardware interfaces powered by web apps.
- **Unix Philosophy for Web Apps**: PWAs gain raw access to pipes, spawn, mmap, and `/dev`.
- **Zero-Bloat Package Management**: Alpine packages installed directly via browser APIs.
- **Strict Capabilities System**: Websites must explicitly request hardware/file access.
- **Safe-Rust 12-Shard Microkernel**: Twelve shard taxonomy replacing 500+ legacy apps with native abstractions ([SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V22.md](SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V22.md)).

---

## 🛠️ Quick Start & Building
```bash
# Build the core library
cargo check --lib

# Run the native test suite
./run_sigma_tests.sh
```

For complete installation and compilation guides, refer to [INSTALL.md](INSTALL.md) and [BUILD.md](BUILD.md).
