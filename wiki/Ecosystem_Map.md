# SigmaOS Ecosystem Map

> Every component, where it lives, and how it fits together.
> Updated: v15.0 / July 2026

---

## Layer Diagram

```
┌──────────────────────────────────────────────────────────────────┐
│  APPLICATIONS (userland/apps/)                                    │
│  sigma-browser · sigma-edit · sigma-office · sigma-pdf           │
│  sigma-play · sigma-mail · sigma-chat · sigma-notes              │
├──────────────────────────────────────────────────────────────────┤
│  DESKTOP ENVIRONMENT (userland/desktop/)                          │
│  compositor · wm · panel · settings · launcher · notifications    │
│  widgets · theme · accessibility                                  │
├──────────────────────────────────────────────────────────────────┤
│  AI & RUNTIME (userland/ai/ + runtime/)                           │
│  sigma-ai (TinyLlama) · WASM runtime · sigma-pod OCI             │
├──────────────────────────────────────────────────────────────────┤
│  TOOLS & DAEMONS (userland/tools/ + userland/daemon/)             │
│  sigma-monitor · sigma-disks · sigma-logs · sigma-doctor         │
│  sigma-bench · sigma-strace · sigma-top · sigma-update           │
│  sigmad-health · sigmad-netd · sigmad-vault · sigmad-metrics     │
├──────────────────────────────────────────────────────────────────┤
│  SHELL & PACKAGES (userland/shell/ + userland/pkg/)              │
│  sigma-sh · sigma-scripting · sigma-coreutils · sigma-pkg        │
├──────────────────────────────────────────────────────────────────┤
│  NETWORK TOOLS (userland/net/)                                    │
│  sigma-ssh · sigma-curl · sigma-vpn · sigma-netctl               │
├──────────────────────────────────────────────────────────────────┤
│  USERLAND SECURITY (userland/vault/)                              │
│  sigma-vault (TPM2-backed secrets)                                │
├──────────────────────────────────────────────────────────────────┤
│  KERNEL NETWORKING (kernel/net/)                                  │
│  TCP · UDP · ICMP · DHCP · DHCPv6 · DNS/DoH · NTP · mDNS        │
│  TLS 1.3+Kyber · Firewall · WireGuard                            │
├──────────────────────────────────────────────────────────────────┤
│  KERNEL FILESYSTEMS (kernel/fs/)                                  │
│  VFS · Tmpfs · ProcFS · SysFS · FAT32 · Ext4 · SigmaFS          │
├──────────────────────────────────────────────────────────────────┤
│  KERNEL SECURITY (kernel/security/)                               │
│  pledge/unveil · ASLR · seccomp · MAC · TPM2 · Landlock · SPIFFE │
│  AVC cache (security/)                                            │
├──────────────────────────────────────────────────────────────────┤
│  KERNEL CORE (kernel/core/)                                       │
│  Scheduler · MM · Syscalls · Process · Panic · cgroup · Namespace│
│  IPC (bus/pipe/shm) · Timer · IRQ                                 │
├──────────────────────────────────────────────────────────────────┤
│  MEMORY (kernel/memory/)                                          │
│  VMM (4-level paging) · OOM killer · Huge pages                  │
├──────────────────────────────────────────────────────────────────┤
│  CRYPTO (kernel/crypto/ + crypto/)                                │
│  SHA-256 · Kyber-1024 · Dilithium-5 (SPARK proven)               │
├──────────────────────────────────────────────────────────────────┤
│  DRIVERS (drivers/)                                               │
│  e1000 · VirtIO-net · Wi-Fi · NVMe · AHCI · VirtIO-GPU           │
│  VESA · HID · xHCI · HDA · PCI+MSI-X · ACPI power               │
├──────────────────────────────────────────────────────────────────┤
│  BOOT (sigma-boot/)                                               │
│  sigma-boot.zig (UEFI EFI stub) → kernel_main                    │
└──────────────────────────────────────────────────────────────────┘
```

---

## Component Count

| Layer | Components | Languages |
|---|---|---|
| Applications | 8+ (planned v1.0) | Rust/Nim |
| Desktop | 9 components | Rust |
| AI & Runtime | 3 | Rust |
| Tools & Daemons | 12 | Rust + Nim |
| Shell & Packages | 4 | Nim |
| Network Tools | 4 | Nim |
| Kernel Network | 11 stacks | Rust |
| Kernel Filesystems | 6 | Rust |
| Kernel Security | 8 | Rust + SPARK |
| Kernel Core | 13 | Rust + Zig |
| Memory | 3 | Zig + SPARK |
| Crypto | 3 | Rust + SPARK |
| Drivers | 11 | Rust + Zig |
| Boot | 1 | Zig |
| **Total** | **100+** | Rust/Zig/Nim/SPARK |

---

## Data Flow: App → Kernel → Hardware

```
App (Rust/Nim)
  │  navigator.sigmaos.* API or direct syscall
  ▼
Syscall Gate (sigma_syscall_dispatch.rs)
  │  sigma_pledge check → sigma_unveil check → seccomp filter
  ▼
Kernel Core (scheduler/mm/ipc/fs)
  │  VFS layer → SigmaFS / Ext4 / Tmpfs
  │  Net layer → TCP/UDP → TLS → WireGuard
  ▼
SDF Driver (e1000 / NVMe / HDA / HID)
  │  PCI MSI-X → DMA → MMIO
  ▼
Hardware
```

---

## Package Flow: Developer → User

```
Developer writes app
  │
  ▼
PKGBUILD recipe (sigma-sdk)
  │
  ▼
sigma-pkg build --target sigpkg
  │  Dilithium-5 sign → SHA-256 verify
  ▼
pkg.sigmaos.app registry
  │
  ▼
User: sigma-pkg install myapp
  │  Verify sig → extract → register
  ▼
Launched via sigma-launcher or sigma-sh
```

---

*See also: [wiki/Subsystem-Map](Subsystem-Map) · [ROADMAP.md](../ROADMAP.md) · [docs/Adoption_Strategy.md](Adoption_Strategy.md)*
