# SigmaOS Architecture

See the [Architecture Wiki page](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture) for the full deep-dive.

## Quick Overview

    User Applications (Native, Flatpak, AppImage, AUR)
             ↓
    Desktop Environment (Zenith + Sigma Shell + Wayland)
             ↓
    System Services (sigma-init, D-Bus, Polkit, logging)
             ↓
    S-AI Layer (Orchestrator, LLM Router, Copilot)
             ↓
    Security Layer (SELinux, AppArmor, Sentinel, pledge)
             ↓
    Sigma Kernel (EEVDF/BORE, Memory, IPC, VFS, Drivers)
             ↓
    Hardware (x86_64, ARM64, RISC-V, UEFI, PCIe)

## Key Design Principles

1.  **Zero-dependency**: Core kernel with no external C runtime dependencies
2.  **Security-first**: Every layer has defence-in-depth mechanisms
3.  **AI-native**: AI orchestration built into the OS, not bolted on
4.  **Compatibility**: Run existing Linux apps without modification
5.  **Performance**: EEVDF+BORE scheduler, eBPF, NUMA-aware design

## Module Map

| Module | Path | Purpose |
|--------|------|---------|
| Kernel | `src/kernel/` | Core scheduler, memory, IPC |
| AI | `src/ai/` | S-AI orchestrator and agents |
| Security | `src/security/` | MAC, crypto, sandboxing |
| Network | `src/network/` | TCP/UDP, firewall, WireGuard |
| Container | `src/container/` | OCI runtime, sandboxing |
| Boot | `src/boot/` | UEFI, TPM, sigma-init |
| Package | `src/sigpkg/` | Package management |
| Distro | `src/distro/` | Linux/BSD parity |
| Desktop | `src/shell/` | Shell, terminal, aliases |
| klib | `src/klib/` | No-std collection library |
