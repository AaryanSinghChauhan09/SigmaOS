# SigmaOS Architecture Overview

## High-Level Architecture

```
┌─────────────────────────────────────────────────┐
│              User Applications                  │
├─────────────────────────────────────────────────┤
│     Zenith Desktop  │  sigma-sh  │  sigma-pkg   │
├─────────────────────────────────────────────────┤
│                System Services                  │
│  Network  │  Storage  │  Audio  │  Display      │
├─────────────────────────────────────────────────┤
│              SigmaOS Microkernel                │
│  Scheduler │ IPC │ Memory │ Capabilities        │
├─────────────────────────────────────────────────┤
│            Hardware Drivers (Rust/C++)          │
├─────────────────────────────────────────────────┤
│          Hardware (x86_64 / aarch64 / riscv64)  │
└─────────────────────────────────────────────────┘
```

## Key Subsystems

### Microkernel (`src/kernel/`)
- Minimal trusted computing base
- Capability-based access control
- IPC channels (synchronous + async)
- BORE+EEVDF scheduler

### Security (`src/security/`)
- Input validation (SSRF-safe IPv4/IPv6)
- MAC: SELinux + AppArmor concepts
- Post-quantum crypto (Dilithium-5, Kyber-1024)
- Pledge/unveil sandboxing

### Package Manager (`src/package/`, `src/sigpkg/`)
- Native .spkg format
- Multi-distro: APT, RPM, ALPM, APK
- AUR-compatible user repository
- Atomic transactions + rollback

### Filesystem (`src/filesystem/`)
- VFS with pluggable backends
- Native SigmaFS (journaled, CoW)
- ext4 compatibility
- OverlayFS for immutable root

### Network (`src/network/`)
- Full TCP/IPv4/IPv6 stack
- TLS 1.3 built-in
- mDNS/DNS-SD discovery
- eBPF firewall

## Design Principles

1. **Safety first** — Rust's type system prevents most memory vulnerabilities
2. **Least privilege** — capability model limits damage from compromised components
3. **Defense in depth** — multiple independent security layers
4. **Multi-distro compatibility** — run software from any major Linux distribution
5. **Future-proof** — post-quantum cryptography built in from the start
