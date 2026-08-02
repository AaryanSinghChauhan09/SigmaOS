# SigmaOS Wiki

Welcome to the **SigmaOS** official wiki — the sovereign, zero-dependency operating system written entirely in Rust.

---

## Quick Links

| Topic | Description |
|---|---|
| [Architecture](Architecture) | System layers, kernel design, subsystem map |
| [Getting Started](Getting-Started) | Build from source, run in QEMU |
| [Security](Security) | Vulnerability reporting, security model, hardening |
| [Linux Distro Inspirations](Linux-Distro-Inspirations) | Concepts absorbed from 25+ Linux distros |
| [Package Management](Package-Management) | SigmaPkg, supported formats, adapters |
| [AI Subsystem](AI-Subsystem) | Local LLM, agents, orchestration |
| [Kernel Collections (klib)](Klib) | Native replacements for std::collections |
| [Compatibility Layers](Compatibility) | Linux, Windows, WASM, FreeDOS compat |
| [Roadmap](Roadmap) | Future development plans |
| [Contributing](Contributing) | How to contribute code and docs |
| [What Works / What Doesn't](What-Is-Working-and-What-Is-Not) | Current implementation status |
| [Diagnostics Guide](Diagnostics) | Build errors, known blockers, fixes |

---

## What is SigmaOS?

SigmaOS is an **experimental, research-grade operating system** designed from first principles with these goals:

1. **Zero stdlib dependency** — all standard collections, allocators, and utilities reimplemented natively in `src/klib/`
2. **Full security stack** — SELinux, AppArmor, capability tokens, Qubes-style isolation, PQC crypto
3. **Multi-distro compatibility** — run software from Debian, Arch, Fedora, Alpine, NixOS natively
4. **Local AI integration** — LLM inference, multi-agent orchestration, AI-assisted scheduling
5. **Digital sovereignty** — no reliance on external services, binaries, or proprietary firmware

---

## Architecture at a Glance

```
User Apps → Zenith Desktop → System Daemons
    ↓              ↓               ↓
         SigmaOS Runtime (IPC, process mgr)
                   ↓
         Security Layer (SELinux, MAC, caps)
                   ↓
         Microkernel (memory, scheduler, VFS)
                   ↓
         Bootloader (UEFI, Secure Boot, TPM)
```

---

## Current Status (2026)

| Subsystem | Status |
|---|---|
| Kernel scheduler (MLFQ, thermal) | ✅ Implemented |
| Memory management (PMM/VMM, slab, buddy) | ✅ Implemented |
| Virtual filesystem (VFS, ext4, Btrfs, XFS) | ✅ Implemented |
| Security (SELinux, AppArmor, capabilities) | ✅ Implemented |
| Networking (TCP/IP, DNS, TLS, VPN) | ✅ Implemented |
| Package management (15 formats) | ✅ Implemented |
| AI subsystem (LLM, agents) | ✅ Implemented |
| Bootloader (UEFI) | 🔄 In progress (pointer safety) |
| GPU drivers | 🔄 In progress |
| Native shell (Sigma Shell) | ✅ Implemented |
| Desktop (Zenith) | 🔄 In progress |
| Accessibility | ✅ Implemented |

---

## Recent Merges

- `jules-8362645389262009630` — Fedora/Qubes/TinyCore compatibility, PKI, VFS improvements
- `jules-8622502909885545855` — SSSD security, WANDR research, Bodhi/Moksha, CachyOS, Garuda
- `master-diagnostics-guide` — SIGTERM handling, AI agents, klib collections, Mint Linux
- `sovereign-universal-self-sufficiency-plan` — VFS FileDescriptor refactor, security hardening

---

## License

SigmaOS is dual-licensed under **MIT** and **GPL-2.0**. See the `LICENSE` file for details.
