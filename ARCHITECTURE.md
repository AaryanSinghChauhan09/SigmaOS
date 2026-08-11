# SigmaOS Architecture

## Overview

SigmaOS is a microkernel-inspired, `#[no_std]` operating system written in safe Rust. It is designed for full digital sovereignty: every subsystem is implemented natively, with zero dependency on C libraries, OpenSSL, or standard Linux userland.

---

## System Layers

```
┌─────────────────────────────────────────────────┐
│            User Applications                     │
│  (native SigmaOS apps + Linux compat layer)     │
├─────────────────────────────────────────────────┤
│              Zenith Desktop                      │
│  (Wayland compositor, window manager, UI shards)│
├─────────────────────────────────────────────────┤
│            System Services / Daemons             │
│  (network, SSH, cron, screen reader, IME, pkg)  │
├─────────────────────────────────────────────────┤
│              SigmaOS Runtime                     │
│  (process manager, IPC, scheduler, syscalls)    │
├─────────────────────────────────────────────────┤
│              Security Layer                      │
│  (SELinux, AppArmor, capabilities, MAC, audit)  │
├─────────────────────────────────────────────────┤
│               Microkernel Core                   │
│  (memory, scheduler, IPC, drivers, VFS)         │
├─────────────────────────────────────────────────┤
│                 Bootloader                       │
│  (UEFI Secure Boot, TPM, verified chain)        │
└─────────────────────────────────────────────────┘
```

---

## Kernel Architecture (`src/kernel/`)

### Scheduler
Multi-level feedback queue (MLFQ) with thermal-aware and transformer-scheduler variants:
- `src/kernel/sched/scheduler.rs` — base Round-Robin + priority
- `src/kernel/sched/sigma_mlfq.rs` — Multi-Level Feedback Queue
- `src/kernel/sched/sigma_thermal_sched.rs` — thermal throttle-aware
- `src/kernel/sched/sigma_transformer_sched.rs` — AI-hint scheduler

### Memory Management
- `src/kernel/memory/pmm_vmm.rs` — Physical & Virtual Memory Manager
- `src/klib/buddy_allocator.rs` — buddy allocator (kernel heap)
- `src/kernel/slab_allocator.rs` — slab allocator for fixed-size objects
- `src/kernel/paging.rs` — x86-64 paging (4-level page tables)

### Interrupts & IRQ
- `src/kernel/irq/irq_controller.rs` — PIC/APIC IRQ management
- `src/interrupt/handler.rs` — IDT, exception handlers

### Processes
- `src/kernel/proc/process_lifecycle.rs` — fork/exec/wait lifecycle
- `src/kernel/proc/signals.rs` — POSIX signal delivery (SIGTERM, SIGKILL, etc.)
- `src/runtime/process/process.rs` — process control block

---

## Security Architecture (`src/security/`)

### Mandatory Access Control
```
                    ┌─────────────────┐
                    │  Security Policy │
                    │  (rules engine)  │
                    └────────┬────────┘
          ┌──────────────────┼──────────────────┐
          ▼                  ▼                  ▼
   ┌─────────────┐  ┌──────────────┐  ┌──────────────┐
   │  SELinux TE  │  │  AppArmor    │  │  Sigma-MAC   │
   │  (type enf.) │  │  (path MAC)  │  │  (capability)│
   └─────────────┘  └──────────────┘  └──────────────┘
```

Key files:
- `selinux.rs` — SELinux type enforcement
- `mac.rs` — AppArmor path-based profiles
- `capability.rs` / `capability_token.rs` — POSIX capabilities + bitmask enforcement
- `capability_enforcer.rs` — enforcement hook for every syscall
- `sigma_pledge.rs` — OpenBSD pledge-style syscall restriction
- `sigma_unveil.rs` — OpenBSD unveil-style path restriction
- `audit.rs` — tamper-evident audit log
- `qubes_isolation.rs` — Qubes OS VM isolation domains
- `intrusion.rs` — fail2ban-style intrusion detection

---

## Virtual Filesystem (`src/filesystem/` + `src/fs/`)

```
 Userspace
     │
     ▼
┌──────────┐
│   VFS    │  (src/filesystem/vfs.rs)
│  Layer   │
└────┬─────┘
     │
     ├── ext4 (src/fs/filesystem.rs)
     ├── XFS  (src/fs/xfs.rs)
     ├── Btrfs (src/filesystem/complete_filesystems.rs)
     ├── tmpfs / devfs / procfs (src/kernel/fs/)
     ├── EncryptedFS (src/crypto/)
     └── CowSnapshot (src/filesystem/cow_snapshot.rs)
```

**OpenFileDescription / FileDescriptor split** (sovereign-branch improvement):
- `OpenFileDescription` — system-wide, reference-counted file state
- `FileDescriptor` — process-private handle to an `OpenFileDescription`

---

## Networking (`src/net/` + `src/network/`)

- `src/net/tcpip_stack.rs` — native TCP/IP stack (no lwIP dependency)
- `src/net/dns.rs` — DNS resolver with split-DNS, hosts file, dnsmasq-style latency sorting
- `src/net/zenith.rs` — Zenith networking abstraction
- `src/network/protocols.rs` — protocol handlers
- `src/network/enterprise.rs` — enterprise networking (VPN, proxy)
- `src/security/vpn.rs` — WireGuard-compatible VPN

---

## Package Management (`src/sigpkg/` + `src/package/`)

SigmaPkg supports all major Linux package formats natively:

| Format | File |
|---|---|
| .deb (Debian/Ubuntu) | `universal_adapter.rs` |
| .rpm (Fedora/RHEL) | `universal_adapter.rs` + `rpm_compat.rs` |
| .pkg.tar.zst (Arch) | `arch_compat.rs` |
| .apk (Alpine) | `universal_adapter.rs` |
| .snap | `universal_adapter.rs` |
| Flatpak | `universal_adapter.rs` |
| AppImage | `universal_adapter.rs` |
| Nix flake | `universal_adapter.rs` |
| ebuild (Gentoo) | `universal_adapter.rs` |
| XBPS (Void) | `universal_adapter.rs` |
| txz (Slackware) | `universal_adapter.rs` |
| eopkg (Solus) | `universal_adapter.rs` |
| guix | `universal_adapter.rs` |

---

## AI Subsystem (`src/ai/`)

- `orchestrator.rs` — S-AI multi-agent orchestrator (model routing, task negotiation)
- `llm.rs` — local LLM inference engine
- `agent.rs` — autonomous agent framework (Bolt / Palette / Sentinel)
- `autogen.rs` — multi-agent task generation
- `lift_engine.rs` — context-lift memory search
- `apm.rs` — AI performance monitor
- `voice.rs` — speech synthesis/recognition

---

## Kernel Library (`src/klib/`) — Zero stdlib

All standard collections reimplemented for `#[no_std]`:

| Module | Replaces |
|---|---|
| `vec.rs` | `std::vec::Vec` |
| `hashmap.rs` | `std::collections::HashMap` |
| `btreemap.rs` | `std::collections::BTreeMap` |
| `hashset.rs` | `std::collections::HashSet` |
| `vecdeque.rs` | `std::collections::VecDeque` |
| `buddy_allocator.rs` | `std::alloc` |
| `hash.rs` | `std::hash::Hash` |

---

## Compatibility Layers (`src/compatibility/`)

SigmaOS can run software from these ecosystems natively:
- **Linux** (glibc/musl syscall translation via S-COSMOS)
- **Windows** (Win32/NT via ReactOS-inspired layer)
- **macOS** (mach port compatibility)
- **WASM** (sandbox runtime)
- **FreeDOS** / **TempleOS** (legacy)

---

## Build System

```bash
cargo build --release          # build SigmaOS library
bash scripts/build-iso.sh      # build bootable ISO
bash scripts/smoke-test.sh     # run smoke tests
python3 scripts/qemu_smoke_test.py  # QEMU emulation test
```

Target: `x86_64-unknown-none` (bare-metal, no OS underneath)

---

## Directory Structure

```
SigmaOS/
├── src/
│   ├── kernel/     # Microkernel (scheduler, memory, IRQ, crypto)
│   ├── klib/       # Zero-stdlib collections & allocators
│   ├── security/   # MAC, capabilities, audit, crypto, VPN
│   ├── filesystem/ # VFS, ext4, Btrfs, CoW snapshots
│   ├── net/ network/ # TCP/IP stack, DNS, VPN, enterprise
│   ├── ai/         # LLM, agents, orchestrator
│   ├── driver/     # Hardware driver framework
│   ├── drivers/    # Specific drivers (USB, input, GPU, serial)
│   ├── compatibility/ # Linux/Windows/macOS compat layers
│   ├── distro/     # Distro-specific implementations
│   ├── sigpkg/     # Package management
│   ├── shell/      # Sigma shell (REPL, commands)
│   ├── desktop/    # Desktop environment
│   ├── graphics/   # GPU driver, compositor, image decoder
│   ├── audio/      # Audio subsystem
│   ├── accessibility/ # Screen reader, magnifier, keyboard
│   ├── productivity/ # Office suite, notes, email, recorder
│   └── ...
├── kernel/         # Low-level kernel modules (C/asm-adjacent)
├── bootloader/     # UEFI bootloader
├── crypto/         # Cryptographic primitives
├── tools/          # Native tool replacements (ls, grep, curl…)
├── scripts/        # Build, test, ISO creation scripts
├── web_ui/         # Zenith desktop web UI
├── docs/           # Documentation
└── WIKI/           # Wiki content (synced to GitHub Wiki)
```
