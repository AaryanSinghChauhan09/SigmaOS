# SigmaOS Wiki

> **SigmaOS Zenith** — The Sovereign Industrial Microkernel. A minimal, blazing-fast, Chromium-based operating system built for silicon sovereignty.

---

## What is SigmaOS?

SigmaOS is a radical rethinking of what an operating system can be. It boots straight to Chromium in under 3 seconds, turns the browser itself into the OS shell, and gives web apps access to raw Unix primitives — `pipe`, `spawn`, `mmap`, `/dev` files — that were previously locked away behind native APIs.

Built on a minimal Linux base (Buildroot + systemd), SigmaOS replaces the traditional desktop stack with a browser-first, capability-gated platform where every process runs inside a bubblewrap sandbox.

---

## How is it different from ChromeOS?

| Feature | ChromeOS | SigmaOS |
|---|---|---|
| Shell environment | Chrome browser + Linux VM | Chrome as the OS shell itself |
| Unix access for web apps | Crostini (heavy VM) | Direct via native messaging daemons |
| Package management | `.deb` via Crostini | Zero-install via `navigator.sigmaos.pkg.ensure()` |
| Process spawn | Not available to PWAs | `navigator.sigmaos.process.spawn()` |
| Capability system | Origin-based permissions | Explicit per-capability grants |
| Window management | Browser tabs | Native WebKit frameless floating windows |
| Boot time | ~10–15 seconds | Under 3 seconds |
| Kernel | Linux (full) | Custom sovereign microkernel + Buildroot |

---

## Wiki Pages

### Getting Started
- [Building from Source](Building-from-Source) — Step-by-step build guide for Ubuntu 22.04
- [Architecture Overview](Architecture-Overview) — Kernel → daemons → extension → web-shell diagram
- [Security Model](Security-Model) — Capability system, zero-trust enforcement, and sandboxing

### API Reference
- [navigator.sigmaos API](API-Reference) — Full reference for all `navigator.sigmaos.*` APIs
- [Syscall Dispatcher](Syscall-Dispatcher) — Kernel syscall table and dispatch mechanism
- [Hardware Abstraction Layer](HAL) — HAL interface across x86_64, ARM, and RISC-V

### Application Development
- [Writing Your First SigmaOS App](Your-First-App) — Hello world PWA tutorial
- [App Manifest Format](App-Manifest) — Schema reference for SigmaOS app manifests

### Subsystems
- [Kernel Architecture](Kernel) — Scheduler, memory manager, and microkernel design
- [Networking Stack](Networking) — TCP/IP suite, socket API, and DNS resolver
- [Zenith Desktop](Zenith-Desktop) — The Sovereign Lattice Desktop environment

---

## Quick Start

```bash
# Clone the repo
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build a bootable ISO
make clean && make iso

# Boot in QEMU
qemu-system-x86_64 -cdrom build/sigmaos.iso -serial stdio -m 2G
```

---

## Current Version

**v15.2 — Zenith Release Microkernel**

See the [Changelog](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CHANGELOG.md) for what's new.
