# SigmaOS Dev Spin — Developer & Engineer Edition

The **SigmaOS Dev** spin is the flagship developer-centric edition of SigmaOS, combining sovereign kernel performance with a complete, batteries-included development ecosystem. Inspired by Clear Linux and Fedora Silverblue.

---

## 👨‍💻 Included Toolchains

| Tool | Version Target | Purpose |
|------|---------------|---------|
| GCC / Clang | Latest stable | C/C++ native compilation |
| Rust toolchain | rustup managed | Systems programming |
| Go | Latest stable | Cloud-native & tooling |
| Python 3.x | 3.12+ | Scripting, data, ML |
| Node.js / Bun | LTS + latest | Web tooling & runtimes |
| OpenJDK | 21 LTS | Java ecosystem |

## 🔧 Build Systems

- **CMake** — native integration with `toolchain-x86_64-elf.cmake`
- **Meson** — sovereign build parity
- **Bazel** — hermetic reproducible builds (Nix-style)
- **GNU Make** — legacy compatibility

## 🗂 Version Control

- **Git** — primary VCS
- **Fossil** — sovereign self-hosted alternative
- **Mercurial** — legacy project support

## 🐳 Container & Virtualization

- **Docker / Podman** — OCI-compliant container runtimes
- **QEMU/KVM** — full hardware virtualization
- **systemd-nspawn** — lightweight kernel namespacing
- **Sovereign WASI runtime** — run .wasm apps natively (via `sigma_wasi.h`)

## 📦 Package Ecosystem

- **OmniPackage** — Sovereign native package manager (`.sigpkg` format)
- **.deb / .rpm** compatibility layer via translation shim
- **Flatpak** — sandboxed universal apps
- **Snap** — optional snap daemon bridge

## 🛠 Default IDE/Editor Stack

- Neovim + sovereign LSP config
- VS Code (Flatpak)
- Helix editor (Rust-native TUI)

---

## 🚀 Installation

```bash
sigma-spin install dev
```

## 📚 See Also

- [Sovereign Toolchain](Compiler-Toolchain.md)
- [WASI Compatibility Layer](Sovereign-Sandbox.md)
- [OmniPackage Manager](OmniPackage-Manager.md)
