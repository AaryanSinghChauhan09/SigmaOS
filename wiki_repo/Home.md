# SigmaOS Wiki

> **v15.0.0 Zenith** · All branches unified on `main` · July 2026

## Quick Links

| | |
|--|--|
| 🚀 [Quick Start](QUICKSTART.md) | ⚙️ [Build Guide](BUILD.md) |
| 📋 [Full Roadmap](FUTURE_ROADMAP.md) | 🔴 [Open Issues](CURRENT_PROBLEMS_MANIFEST.md) |
| 🧠 [Kernel Internals](Kernel.md) | 🔒 [Security Model](SECURITY_MODEL.md) |
| 📦 [Package Manager](sigma-pkg.md) | 🖥️ [Zenith Desktop](Zenith-Desktop.md) |
| 🤖 [AI Integration](sigma-ai.md) | 🌐 [Networking](sigma-net.md) |
| 🇮🇳 [India Stack](India-Stack.md) | 📊 [Performance](Performance-Benchmarks.md) |

---

## What is SigmaOS?

SigmaOS is a next-generation **sovereign operating system** built ground-up for:

- **Privacy** — zero telemetry, local-first, PQC encryption everywhere
- **Performance** — MLFQ+CFS+EDF unified scheduler, io_uring I/O, SIMD-optimized paths
- **Security** — post-quantum cryptography, formal verification, MAC, TPM-backed attestation
- **AI-native** — on-device LLM inference, adaptive scheduler, AI shell completion
- **Sovereignty** — no dependency on US/Chinese proprietary software; India Stack built-in

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                   ZENITH DESKTOP                        │
│  App Launcher · Window Manager · System Tray · Widgets  │
├─────────────────────────────────────────────────────────┤
│                 USERLAND / APPS                         │
│  sigma-edit · sigma-files · sigma-browser · sigma-mail  │
│  sigma-terminal · sigma-settings · sigma-calc · etc.    │
├─────────────────────────────────────────────────────────┤
│              sigma-pkg  /  sigma-sdk                    │
├─────────────────────────────────────────────────────────┤
│                   KERNEL (Rust/Zig)                     │
│  ┌──────────┬──────────┬──────────┬──────────────────┐  │
│  │ MLFQ+CFS │  PMM/VMM │ VFS+Ext4 │ TCP/UDP/Wi-Fi/BT │  │
│  │ EDF Sched│ Buddy+Slab│ CryptFS  │ ARP/DNS/DHCP/TLS │  │
│  ├──────────┼──────────┼──────────┼──────────────────┤  │
│  │  IPC     │ GPU/DRM  │  Sound   │  USB/xHCI/HID    │  │
│  │  eBPF    │  KMS     │  HDA     │  Container/OCI   │  │
│  ├──────────┴──────────┴──────────┴──────────────────┤  │
│  │           HARDWARE ABSTRACTION LAYER (HAL)         │  │
│  │   x86-64 · ARM64 · RISC-V · PCI/PCIe · ACPI       │  │
│  └────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────┤
│               BOOT (sigma-boot.efi / GRUB2)             │
└─────────────────────────────────────────────────────────┘
```

---

## Current Implementation Status (Phase 10)

### ✅ Kernel Core
| Subsystem | File | Completeness |
|-----------|------|-------------|
| MLFQ+CFS+EDF Scheduler | `kernel/core/sigma_sched.rs` | ✅ 95% |
| Physical Memory Manager (Buddy) | `kernel/core/sigma_pmm.rs` | ✅ 90% |
| Virtual Memory (MMU/page tables) | `kernel/core/sigma_mm.rs` | ✅ 85% |
| IRQ Controller (APIC/GIC) | `kernel/core/sigma_irq_controller.rs` | ✅ 90% |
| Syscall Dispatch (30+ syscalls) | `kernel/core/sigma_syscall_dispatch.rs` | ✅ 80% |
| Process Manager | `kernel/core/sovereign_process_manager.rs` | ✅ 85% |
| IPC (Pipe/MsgQ/SHM) | `kernel/core/sigma_ipc_pipe.rs` | ✅ 95% |
| VFS + Ext4 | `kernel/core/sigma_vfs_ext4.rs` | ✅ 80% |
| eBPF Subsystem | `kernel/bpf/sigma_ebpf.rs` | ✅ 70% |
| epoll / io_uring | `kernel/core/sigma_epoll.rs` | ✅ 75% |

### ✅ Networking
| Subsystem | File | Completeness |
|-----------|------|-------------|
| Full IP/TCP/UDP stack | `kernel/core/sigma_network_stack.rs` | ✅ 85% |
| TCP state machine (RFC 793) | `kernel/core/sigma_tcp_stack.rs` | ✅ 90% |
| Wi-Fi 802.11ax / WPA3 | `kernel/core/sigma_wifi_stack.rs` | ✅ 80% |
| Bluetooth 5.3 (HCI/GATT) | `kernel/core/sigma_bluetooth.rs` | ✅ 75% |
| ARP/DHCP/DNS | `kernel/core/sigma_network_stack.rs` | ✅ 80% |
| e1000 NIC driver | `drivers/net/sigma_e1000.rs` | ✅ 90% |

### ✅ Drivers
| Driver | Status |
|--------|--------|
| NVMe (MMIO) | ✅ 90% |
| USB xHCI + HID | ✅ 85% |
| GPU/DRM/KMS | ✅ 80% |
| Sound (HDA/PipeWire) | ✅ 80% |
| VirtIO-net/blk | ✅ 85% |
| Intel e1000 NIC | ✅ 90% |

### ✅ Security
| Feature | Status |
|---------|--------|
| ML-KEM-768 (Kyber) | ✅ Complete |
| ML-DSA-65 (Dilithium) | ✅ Complete |
| PBKDF2/HKDF key derivation | ✅ Complete |
| AppArmor-style MAC | ✅ 80% |
| Zero-trust enforcer | ✅ 75% |
| Secure boot chain | ✅ 70% |
| Sandbox (pledge/unveil) | ✅ 80% |

### ✅ AI/ML
| Feature | Status |
|---------|--------|
| sigma-ai daemon | ✅ 70% |
| On-device LLM inference | ✅ 60% |
| Adaptive scheduler | ✅ 70% |
| AI IDS | ✅ 65% |
| Tokenizer | ✅ 75% |

---

## Deployment Profiles

| Profile | Use Case | Size |
|---------|----------|------|
| **Zenith** (default) | Full desktop OS | ~800 MB |
| **Cloud** | Headless server/container host | ~150 MB |
| **Microkernel** | Minimal embedded | ~20 MB |
| **RTOS** | Hard real-time | ~15 MB |
| **Mobile** | ARM64 phone/tablet | ~400 MB |
| **IoT** | Raspberry Pi + sensors | ~100 MB |
| **Dual-boot** | Coexist with Windows/Linux | ~600 MB |
| **Security** | Air-gapped hardened | ~300 MB |
| **AI** | ML research + inference | ~2 GB |
| **Browser** | WASM demo (runs in browser) | ~50 MB |

---

## Roadmap Summary

| Phase | Timeline | Focus |
|-------|----------|-------|
| I | Q3 2026 | UEFI boot, ISO, SATA, virtio-GPU |
| J | Q4 2026 | ARM64, RISC-V, eBPF JIT, Linux compat |
| K | Q1 2027 | PQ-TLS 1.3, TPM, FIDO2, MAC |
| L | Q2 2027 | SDK v2, App store, Zenith desktop v2 |
| M | Q3 2027 | sigma-ai v2, edge ML, adaptive sched |
| N | Q4 2027 | sigma-pod v2, cloud images, enterprise |
| O+ | 2028+ | India Stack, Defence, IoT, Quantum |
| v2.0 | 2028 | Production sovereign OS |

See [FUTURE_ROADMAP.md](FUTURE_ROADMAP.md) for complete details.

---

## Contributing

- **Kernel:** `kernel/core/` — Rust `#![no_std]`
- **Drivers:** `drivers/` — follow DDK in `drivers/ddk/`
- **Apps:** `userland/apps/` or `usr/apps/`
- **Wiki:** `wiki_repo/` — Markdown, auto-synced

Read [CONTRIBUTING.md](CONTRIBUTING.md) and [KERNEL_DEVELOPER_HANDBOOK.md](KERNEL_DEVELOPER_HANDBOOK.md).

---

*SigmaOS — Sovereignty. Speed. Security.* | [GitHub](https://github.com/AaryanSinghChauhan09/SigmaOS)
