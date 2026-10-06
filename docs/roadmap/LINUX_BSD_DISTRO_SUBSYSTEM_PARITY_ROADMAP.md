# 🌐 Linux & BSD Distro Subsystem Parity Roadmap for SigmaOS

This roadmap details the subsystem parity milestones inspired by leading Linux distributions (Arch, Debian, Fedora, Alpine, Void, NixOS, Clear Linux, CachyOS) and BSD operating systems (FreeBSD, OpenBSD, NetBSD, DragonFly BSD).

---

## ⚡ 1. Kernel & Low-Level Subsystems
- **EEVDF & BORE Scheduler**: Sub-80ns context switching with real-time deadline lanes and burst-oriented interactivity boosting.
- **SchedExt (eBPF Extensible Scheduler)**: Dynamic user-space eBPF scheduling policies loaded without kernel recompilation.
- **Linux Landlock V5 LSM**: Fine-grained filesystem and network access sandboxing for unprivileged processes.
- **MGLRU & Zswap Compressed Memory**: Multi-generational LRU page reclamation with zstd-compressed in-memory swap pools.

---

## 🛡️ 2. Security & Privilege Governance
- **OpenBSD Pledge & Unveil**: System call restriction (`pledge`) and path visibility masking (`unveil`).
- **FreeBSD Capsicum Capabilities**: Fine-grained file descriptor capability mode sandboxing.
- **Hardened Sudo Engine**: Zero-dependency privilege elevation hardened against C-ABI NUL-byte truncation and control character injection.

---

## 📦 3. Universal Package Architecture
- **Multi-Format Adapter Matrix**: Unified translation across 50+ Linux and BSD package formats (`.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.xbps`, `.eopkg`, `.hpkg`, `.tcz`, `pkgsrc`).
- **Post-Quantum Cryptography**: Dual-layer Dilithium-5 and PGP signature verification for package archives.
- **Portage USE Flag Solver**: Dynamic compile-time dependency resolution based on active USE flags.
