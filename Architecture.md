# SigmaOS Architecture

> Canonical architecture reference. For the privilege/isolation boundary doc, see [Architecture.md](Architecture.md).

---

## System Layers

```
┌─────────────────────────────────────────────────────────────────┐
│  USER SPACE (Ring 3 / EL0)                                      │
│  PWAs · Zenith Desktop · profession apps · sigma-ai LLM         │
├─────────────────────────────────────────────────────────────────┤
│  BROWSER SHELL (optional profile)                               │
│  Custom Chromium + navigator.sigmaos.* API bridge               │
├─────────────────────────────────────────────────────────────────┤
│  SYSTEM DAEMONS (Ring 3, capability-restricted)                 │
│  sigmad-health · sigmad-pkg · sigmad-netd · sigmad-vault        │
│  sigmad-watchdog · sigmad-metrics · sigmad-cloudsync            │
├─────────────────────────────────────────────────────────────────┤
│  SYSCALL INTERFACE                                              │
│  sigma_pledge (allowlist) + sigma_unveil (path restriction)     │
│  seccomp-BPF filter · AVC O(1) MAC cache                        │
├─────────────────────────────────────────────────────────────────┤
│  KERNEL (Ring 0 / EL1 / S-Mode) — freestanding, no glibc       │
│  ┌──────────┬──────────┬──────────┬──────────┬───────────────┐  │
│  │ Scheduler│ Memory   │ Security │ Network  │ Filesystem    │  │
│  │ MLFQ+EDF │ Buddy+   │ pledge/  │ TLS1.3+  │ VFS + SigmaFS │  │
│  │ + CFS    │ Slab+    │ unveil+  │ Kyber+   │ Ext4 + Tmpfs  │  │
│  │ + RT     │ 4-level  │ AVC+     │ DNS/DoH+ │ + dm-verity   │  │
│  │ + AI     │ paging   │ ZeroTrust│ DHCP+    │ + OSTree A/B  │  │
│  │          │ + ASLR   │ + TPM2   │ WPA3+    │               │  │
│  │          │ + W^X    │ + PQC    │ Firewall │               │  │
│  └──────────┴──────────┴──────────┴──────────┴───────────────┘  │
│  IPC · IRQ/APIC · cgroups · namespaces · eBPF · kprobes         │
├─────────────────────────────────────────────────────────────────┤
│  HARDWARE ABSTRACTION (SovereignHAL)                            │
│  x86_64 · ARM64 · RISC-V RV64GC                                 │
├─────────────────────────────────────────────────────────────────┤
│  HARDWARE                                                       │
│  CPU · NVMe · GPU · NIC · USB · TPM2 · UEFI                     │
└─────────────────────────────────────────────────────────────────┘
```

---

## Core Subsystems

### Scheduler (`kernel/sched/`)

- **MLFQ**: 4 queues with aging — interactive tasks stay responsive

- **CFS clone**: vruntime + red-black tree for fair CPU sharing

- **EDF**: Earliest-Deadline-First for `release/rtos` hard real-time tasks

- **SCHED_SOVEREIGN RT**: bounded IRQ latency < 10 µs target

- **sigma-ai predictive**: TinyLlama pre-warming for hot code paths (Phase H)

- **NUMA-aware**: reads ACPI SRAT table for memory locality placement

### Memory Manager (`kernel/memory/`, `kernel/mm/`)

- **Buddy allocator**: 2^n page-frame management, O(log n) alloc/free

- **Slab allocator**: kmalloc via object caches, minimises fragmentation

- **4-level paging** (x86_64 PML4): per-process virtual address spaces

- **ASLR**: 42-bit entropy per VMA region

- **W^X enforcement**: no page is simultaneously writable and executable

### Security (`security/`, `kernel/security/`)

- **sigma_pledge**: process declares capabilities at exec; kernel enforces allowlist

- **sigma_unveil**: process declares filesystem paths; all others denied

- **AVC**: O(1) SELinux-inspired access vector cache for MAC decisions

- **Zero-trust**: SPIFFE workload identities, per-syscall cryptographic attestation

- **PQC**: Kyber-1024 KEM + Dilithium-5 signatures baked into TLS, packages, boot

- **TPM2**: seals CryptFS key derivation; remote attestation via sigma-trustd

### Networking (`net/`, `kernel/net/`)

- **Stack**: IPv4/IPv6 · TCP · UDP · ICMP · ARP

- **TLS 1.3**: X25519/Kyber-1024 hybrid key exchange

- **DNS**: UDP/TCP/DoH + DNSSEC + LRU cache

- **DHCP**: full RFC 2131/2132 state machine

- **WPA3/SAE**: dragonfly key exchange (P-256)

- **Firewall**: stateful + NAT + conntrack

- **Mesh**: CRDT offline-first sync, ZeroNet (release/distributed)

### Filesystem (`fs/`, `kernel/fs/`)

- **VFS**: generic inode/dentry/file layer

- **SigmaFS**: native CoW journaling filesystem (Phase G)

- **Ext4**: read/write with JBD2 ordered journaling

- **FAT32**: for EFI system partitions

- **Tmpfs**: RAM-backed ephemeral storage

- **dm-verity**: block-level integrity verification (release/cloud)

- **OSTree A/B**: atomic updates (release/cloud, release/standalone)

### HAL (`hal/`, `arch/`)

- Multi-arch: x86_64, ARM64, RISC-V RV64GC

- PCI/PCIe enumeration + MSI-X interrupt routing

- ACPI tables (MADT/SRAT/DSDT) parsing

- UEFI runtime services via sigma-boot.efi (Phase G)

### Sovereign Driver Framework (SDF) (`drivers/`)

- Each driver: `probe()` → `init()` → `shutdown()` lifecycle

- Ring-3 driver launch for fault isolation (Phase G)

- Auto-registered via `SIGMA_SDF_REGISTER_DRIVER` macro

---

## Shard System

SigmaOS code is organised into **600+ shards** — atomic, independently-testable modules. Shards are identified by `S<N>_<Name>` and live in `suites/`.

Key shard groups:

- **S01–S14**: Genesis, Silicon, ZenithUI, HAL, Memory, Storage, Network, Security, Intelligence, Registry, Virtualisation, Ecosystem, LuaBridge, Transcendence

- **S034–S500+**: Extended capability shards (IPC, crypto, observability, AI, etc.)

---

## Deployment Profiles

8 profiles compiled from one codebase via CMake feature flags:

| Profile | Key Difference |
|---------|---------------|
| `standalone` | Full Zenith DE, profession apps, sigma-ai |
| `browser` | `navigator.sigmaos.*` API, Chromium shell |
| `microkernel` | < 512 KB kernel, < 8 MB RAM, sigma-bus IPC |
| `mobile` | ARM64 GIC, touch UI, NEON Kyber |
| `rtos` | EDF scheduler, < 10 µs IRQ latency |
| `dual-boot` | EFI boot entry, NTFS read, GRUB chainload |
| `cloud` | Immutable root, A/B partitions, sigma-pod |
| `distributed` | CRDT mesh, SovereignCloudFS, ZeroNet |

---

## Directory Map

| Directory | Purpose |
|-----------|---------|
| `kernel/` | Core microkernel |
| `arch/` | CPU-specific code (x86_64, arm64, riscv64) |
| `drivers/` | SDF hardware drivers |
| `hal/` | Hardware abstraction layer |
| `fs/` | Filesystems |
| `net/` | Network stack |
| `security/` | Security subsystems |
| `crypto/` | PQC crypto primitives |
| `memory/` | Physical memory management |
| `scheduling/` | Scheduler implementations |
| `ui/` | Desktop compositor + UI toolkit |
| `userland/` | Shell, coreutils, system daemons |
| `suites/` | 600+ capability shards |
| `include/` | All headers |
| `docs/` | Extended documentation |
| `wiki_repo/` | GitHub Wiki source |
| `tests/` | Unit, integration, regression, fuzz tests |
| `scripts/` | Build, CI, release automation |

---

*See also: [Wiki: Architecture Overview](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture-Overview) · [DEVELOPMENT_ROADMAP.md](DEVELOPMENT_ROADMAP.md)*
