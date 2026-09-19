# 🚀 SigmaOS — Desktop Edition

SigmaOS is a secure, fast, opinionated Rust desktop operating system with atomic updates, capability-based applications, and a curated Zenith workflow.

Inspired by Omarchy Linux, SigmaOS evolves from a broad OS research platform into a **bootable, user-focused desktop distribution** with a clear, demonstrable path:

```
boot → install → login → Zenith desktop → package installation → update → rollback
```

---

## 📜 Key Engineering & Vision Documents
- 🎯 **Product Vision & Manifesto**: [docs/PRODUCT_VISION.md](docs/PRODUCT_VISION.md)
- 📋 **Release Criteria & Quality Gates**: [docs/RELEASE_CRITERIA.md](docs/RELEASE_CRITERIA.md)
- 🖥️ **Hardware Support Matrix**: [docs/SUPPORT_MATRIX.md](docs/SUPPORT_MATRIX.md)
- 🗺️ **Master Execution Roadmap**: [docs/ROADMAP.md](docs/ROADMAP.md)
- 📦 **Universal Package System Plan**: [docs/UNIVERSAL_PACKAGE_SYSTEM_IMPLEMENTATION_PLAN.md](docs/UNIVERSAL_PACKAGE_SYSTEM_IMPLEMENTATION_PLAN.md)
- ⚡ **Strategy vs Legacy Distros**: [docs/STRATEGY_TO_SURPASS_AND_DEFEAT_LINUX_BSD.md](docs/STRATEGY_TO_SURPASS_AND_DEFEAT_LINUX_BSD.md)

---

## 🧩 Core Product Features
- **Zenith Compositor & Keyboard-Driven Shell**: Responsive Wayland tiling desktop with Omarchy-inspired agentic steering (Bolt ⚡, Palette 🎨, Sentinel 🛡️).
- **Atomic System Updates**: Dual-root A/B images (`mkosi` / `sysupdate`) with sub-second Copy-on-Write (CoW) rollback.
- **Universal Package Engine (`sigpkg`)**: Native signed `.sigpkg` packages with support for 60+ Linux & BSD package extensions via containerized wrappers (`apx`).
- **Zorin Exec Guard Security**: Default-deny capability permission model that intercepts untrusted binaries and recommends verified native or WebApp alternatives.
- **Declarative System Preferences**: Simple, validated TOML preference profiles (`/system/profile.toml`, `/user/preferences.toml`).

---

## 🛠️ Quick Start & Verification
```bash
# Build the core library
cargo check --lib

# Run native Rust test suites
./run_sigma_tests.sh

# Run Python integration & fuzz tests
pytest tests/
```

---

## 📄 License
SigmaOS is licensed under the [MIT License](LICENSE).
