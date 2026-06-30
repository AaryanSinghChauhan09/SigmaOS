# Welcome to the SigmaOS Wiki

> **SigmaOS** — A sovereign, self-sufficient operating system. No external dependencies. No compromises.
>
> *"Fuse the paranoia of Whonix, the reproducibility of NixOS, the polish of Elementary, and the immutability of Flatcar — into one sovereign identity."*

---

## 🚀 Quick Navigation

| Area | Wiki Page | Status |
|---|---|---|
| Roadmap (v0.1 → v1.0) | [Roadmap](Roadmap.md) | ✅ Active |
| Absorption Matrix | [Absorption-Matrix](Absorption-Matrix.md) | 🔄 Updated |
| Security Model | [Security-Model](Security-Model.md) | ✅ Formal spec |
| Architecture | [Architecture](Architecture.md) | ✅ Stable |
| Contributing | [Contributing](Contributing.md) | ✅ Active |
| Coding Standards | [Coding-Standards](Coding-Standards.md) | ✅ Stable |
| Sovereign System Profiles | [Sovereign-System-Profiles](Sovereign-System-Profiles.md) | ✅ Stable |
| Packaging & Immutability | [Sovereign-Packaging-and-Immutability](Sovereign-Packaging-and-Immutability.md) | ✅ Stable |
| Zenith Desktop | [Zenith-Desktop-SDK](Zenith-Desktop-SDK.md) | ✅ Stable |
| Driver Support | [Driver-Support](Driver-Support.md) | ✅ Phase 5 |
| Container Orchestrator | [Container-Orchestrator](Container-Orchestrator.md) | ✅ Stable |
| CI/CD Workflows | [CI-Workflows](CI-Workflows.md) | ✅ Stable |

---

## 🏗️ Architecture at a Glance

```
┌─────────────────────────────────────────────────────────────┐
│               ZENITH DESKTOP (Phase 4)                      │
│   Compositor · App Store · Tiling WM · AI Scheduler         │
├─────────────────────────────────────────────────────────────┤
│           SOVEREIGN ORCHESTRATOR (Phase 3)                  │
│   sigma-container · sigma-sandbox · IPC Bus                 │
├──────────────────────────┬──────────────────────────────────┤
│   KERNEL SUBSYSTEMS      │   SOVEREIGN USERLAND             │
│   Driver Registry        │   sigma-sh (Rust shell)          │
│   EEVDF Scheduler        │   sigpkg (package manager)       │
│   Memory / VMM           │   sigma-core-utils               │
│   Syscall Audit          │   sigma-crypto-vault             │
│   sigma-shield (BPF)     │   SigmaVCS (version control)    │
├──────────────────────────┴──────────────────────────────────┤
│            SOVEREIGN LIBC (zero glibc dependency)           │
│   sigma_malloc · sigma_memcpy · sigma_strcmp                │
└─────────────────────────────────────────────────────────────┘
```

---

## 📦 What SigmaOS Replaces

SigmaOS systematically absorbs open-source tools and replaces them with sovereign equivalents. See the **[Absorption Matrix](Absorption-Matrix.md)** for the full list (70+ tools mapped).

**Key replacements:**
- `sigma-sh` → Bash/Zsh/Fish
- `sigpkg` → apt/pacman/npm
- `SigmaVCS` → Git
- `sigma-sandbox` → SELinux/AppArmor
- `sigma-crypto` → OpenSSL/libsodium
- `sigma-vault` → KeePass/Bitwarden
- `zenith-compositor` → Wayland/X11

---

## 🔒 Security Layers

- **Zero-Trust VFS**: Explicit RBAC at filesystem level
- **Capability Sandbox**: Every process gets minimal cap token (Capsicum-inspired)
- **Syscall Audit**: BPF-based audit with policy-as-code
- **Secure Boot**: TPM 2.0 + Ed25519 + A/B rollback
- **Hardened Allocator**: Magic cookies, guard pages, poison-on-free
- **PQC Crypto**: Kyber-1024 + Ed25519 + ChaCha20-Poly1305

→ Full spec: [Security-Model](Security-Model.md)

---

## 🛠️ Getting Started

```bash
# Clone the repo
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build the kernel (requires Rust nightly + x86_64-unknown-none target)
rustup target add x86_64-unknown-none
cargo build --target x86_64-unknown-none -p sigmaos-kernel

# Build sigma-sh shell
cargo build -p sigma-sh

# Build sigpkg package manager  
cargo build -p sigpkg

# Run in QEMU
just qemu
```

See [Contributing](Contributing.md) for full setup instructions.

---

## 📊 Current Status

| Milestone | Version | Status |
|---|---|---|
| Genesis (boot + VGA + shell) | v0.1 | ✅ Done |
| Stability + contributor funnel | v0.2 | 🔄 In Progress |
| Networking + security hardening | v0.3 | 🎯 Planned |
| Desktop + GPU | v0.4 | 🎯 Planned |
| Kernel observability + fleet | v0.5 | 🎯 Planned |
| Sovereign production | v1.0 | 🎯 Planned |

---

*SigmaOS Wiki · [Issue Tracker](https://github.com/AaryanSinghChauhan09/SigmaOS/issues) · [Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)*
