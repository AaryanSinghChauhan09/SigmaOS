# SigmaOS Ecosystem Map

> Full ecosystem diagram and component catalogue.
> Canonical source: [docs/Ecosystem_Map.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Ecosystem_Map.md)

---

## Layer Overview

```
Applications     → sigma-browser · sigma-edit · sigma-office · sigma-play
Desktop          → compositor · wm · panel · settings · launcher · notifications
AI & Runtime     → sigma-ai (TinyLlama) · WASM runtime · sigma-pod OCI
Tools & Daemons  → monitor · disks · logs · doctor · bench · strace · top
                   sigmad-health · sigmad-netd · sigmad-vault · sigmad-metrics
Shell & Packages → sigma-sh · scripting · coreutils · sigma-pkg
Network Tools    → ssh · curl · vpn · netctl · crdt-sync
Kernel Network   → TCP · ICMP · DHCP · DHCPv6 · NTP · mDNS · DNS · TLS · WG
Kernel Filesys   → VFS · Tmpfs · ProcFS · SysFS · FAT32 · Ext4 · SigmaFS
Kernel Security  → pledge · unveil · ASLR · seccomp · MAC · Landlock · SPIFFE
                   Capability tokens · TPM2 · AVC cache
Kernel Core      → Scheduler · MM · Syscalls · Process · cgroup · Namespace
Memory           → VMM (4-level paging) · OOM · Huge pages (SPARK proven)
Crypto           → SHA-256 · Kyber-1024 · Dilithium-5 (SPARK proven)
Drivers          → e1000 · VirtIO-net · Wi-Fi · NVMe · AHCI · GPU · HDA
Runtime          → ELF loader · WASM parser · OCI container
Boot             → sigma-boot.zig (UEFI stub)
```

---

## Component Count: 100+

| Layer | Count |
|---|---|
| Desktop UI | 9 |
| Userland Tools | 13 |
| Network Tools | 5 |
| Kernel Net | 11 |
| Kernel FS | 6 |
| Kernel Security | 9 |
| Kernel Core | 14 |
| Drivers | 11 |
| Runtime | 3 |
| **Total** | **100+** |

---

## New in Batch 6

| Component | File | Purpose |
|---|---|---|
| Panel / Dynamic Island | `userland/desktop/sigma_panel.rs` | Top status bar with modular items |
| Settings Hub | `userland/desktop/sigma_settings.rs` | Unified settings (appearance/network/privacy) |
| Accessibility | `userland/desktop/sigma_accessibility.rs` | High contrast, screen reader, colour blind, sticky keys |
| Metrics daemon | `userland/daemon/sigmad_metrics.rs` | Prometheus-compatible /metrics endpoint |
| sigma-strace | `userland/tools/sigma_strace.nim` | Syscall tracer (ptrace-based) |
| sigma-top | `userland/tools/sigma_top.nim` | Real-time process monitor (full ncurses-free TUI) |
| CRDT sync | `userland/net/sigma_crdt.rs` | Offline-first sync — vector clocks, LWW, PN counters |
| ELF loader | `runtime/elf/sigma_elf_loader.rs` | ELF64 parser + PT_LOAD mapper (no_std) |
| Capability tokens | `kernel/security/sigma_capability.rs` | seL4-inspired capability system with derivation + revocation |
| SysFS | `kernel/fs/sigma_sysfs.rs` | /sys virtual filesystem with callback nodes |

---

*Full map: [docs/Ecosystem_Map.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Ecosystem_Map.md)*
