# CORE IMPROVEMENTS

> **Status**: ACTIVE | **Priority**: Critical | **Tracks**: Kernel, Packages, Desktop, Security

This document tracks the core improvement initiatives required to achieve feature parity with major Linux distributions and then surpass them with SigmaOS-exclusive capabilities.

---

## 1. Package Management Improvements

SigmaOS already integrates Pacman, Flatpak, and Nix-style builds. To match and exceed Linux distros:

### 1.1 sigpkg Enhancements

| Improvement | Description | Status |
|---|---|---|
| Content-addressed store | Nix-style `/sigma/store/` with hash-based paths | ✅ Done |
| Delta updates | Binary diff patches (bsdiff) for package upgrades | 📋 Planned |
| Parallel downloads | Concurrent package fetches (up to 8 streams) | 🔄 Active |
| Mirror auto-selection | Geo-aware mirror ranking via latency probing | 📋 Planned |
| Dependency resolution | SAT-solver based conflict resolution | ✅ Done |
| Rollback support | Atomic transaction rollback on install failure | ✅ Done |
| PQC signatures | Dilithium5 package verification | ✅ Done |
| Dry-run safety | `--dry-run` guaranteed no-mutation (ANOMALY-0004 fix) | ✅ Done |

### 1.2 Package Ecosystem Growth

```
Current:   ~2,000 native packages
Target Q4: ~10,000 native packages
Target Y2: ~50,000 (via absorption pipeline + community recipes)
```

**Strategy**:
- Automated PKGBUILD → sigpkg recipe translator
- AUR-compatible `sigma-recipes` community repository
- Flatpak hub integration for desktop applications
- AppImage compatibility layer for portable apps

---

## 2. Kernel Core Improvements

### 2.1 Scheduler Evolution

```rust
// Current: EEVDF with static slices
// Improvement: AI-autotuned EEVDF with workload classification

pub struct ImprovedEEVDF {
    base_slice_ns: u64,        // configurable base time-slice
    ai_tuner: AISchedulerTuner, // neural prediction of optimal slice
    workload_classifier: WorkloadType, // CPU-bound, IO-bound, RT, interactive
}
```

| Improvement | Benchmark Target | Status |
|---|---|---|
| Workload-aware classification | +18% compile throughput | ✅ Done (ANOMALY-0008) |
| Cache-pressure-aware slicing | -12% L3 miss rate | 🔄 Active |
| NUMA-aware task migration | +25% NUMA-local memory hits | ✅ Done |
| AI predictive pre-scheduling | +8% interactive latency | 📋 Planned |

### 2.2 Memory Management

| Improvement | Description | Status |
|---|---|---|
| Slab allocator optimization | Per-CPU slab caches, magazine layer | ✅ Done |
| Transparent huge pages | 2MB/1GB THP support with defrag | 🔄 Active |
| Memory compaction | Online defragmentation daemon | 📋 Planned |
| Balloon driver | Dynamic memory sizing for VMs/containers | 🔄 Active |
| DAMON integration | Data-access monitoring for cold page reclaim | 📋 Planned |

### 2.3 I/O Subsystem

| Improvement | Description | Status |
|---|---|---|
| io_uring support | Async I/O ring for userspace | 🔄 Active |
| NVMe multipath | Active-active NVMe path management | 📋 Planned |
| XFS/ext4 parity | Feature-complete filesystem drivers | 🔄 Active |
| bcachefs exploration | Next-gen COW filesystem evaluation | 📋 Planned |

---

## 3. Desktop Environment Improvements

### 3.1 Zenith Desktop

| Improvement | Description | Status |
|---|---|---|
| Wayland-native compositor | No X11 dependency, DRM/KMS direct | 🔄 Active |
| Fractional scaling | Per-monitor fractional DPI (125%, 150%, 175%) | 📋 Planned |
| Variable refresh rate | Freesync/G-Sync in compositor | 📋 Planned |
| Window tiling | Auto-tiling with keyboard shortcuts (i3-style) | 🔄 Active |
| Notification center | Grouped, actionable notifications | 📋 Planned |
| Settings hub redesign | Unified settings with search | 🔄 Active |
| Theme engine | GTK/Qt unified theming via sigma-theme | 📋 Planned |

### 3.2 Shell & Terminal

| Improvement | Description | Status |
|---|---|---|
| sigma-shell | POSIX-compatible with AI completions | 🔄 Active |
| Rich prompt | Git status, Python venv, exit code display | ✅ Done |
| Syntax highlighting | Real-time command syntax coloring | 📋 Planned |
| History search | Fuzzy search with AI-ranked suggestions | 📋 Planned |

---

## 4. Security Improvements

| Improvement | Description | Status |
|---|---|---|
| MAC policy engine | SELinux/AppArmor-equivalent in native MAC | 🔄 Active |
| Integrity Measurement | IMA/EVM for runtime file integrity | ✅ Done |
| Secure boot chain | UEFI → bootloader → kernel → shards | ✅ Done |
| Kernel CFI | Control Flow Integrity for kernel code | 🔄 Active |
| Stack protector | `-fstack-protector-strong` on all kernel builds | ✅ Done |
| KASLR | Kernel Address Space Layout Randomization | ✅ Done |
| Audit framework | syscall auditing + anomaly resolution log | ✅ Done |

---

## 5. Networking Improvements

| Improvement | Description | Status |
|---|---|---|
| TCP BBR congestion | Modern congestion control algorithm | 🔄 Active |
| WireGuard native | WireGuard VPN as kernel shard | 📋 Planned |
| eBPF networking | XDP-based packet processing | 🔄 Active |
| IPv6 full support | Dual-stack + privacy extensions | ✅ Done (ANOMALY-0001 fix) |
| mDNS/DNS-SD | Zero-conf networking for LAN discovery | 📋 Planned |
| QUIC transport | HTTP/3 support in network stack | 📋 Planned |

---

## 6. Developer Experience Improvements

| Improvement | Description | Status |
|---|---|---|
| sigma-sdk | Shard scaffolding, build, test tools | 🔄 Active |
| GDB kernel debugging | QEMU + GDB integration for kernel dev | ✅ Done |
| eBPF tracing | `sigma-trace` for system-wide tracing | 🔄 Active |
| Documentation | Wiki fully implemented, API docs generated | 🔄 Active |
| CI/CD pipeline | GitHub Actions + QEMU hardware matrix | ✅ Done |

---

## Progress Dashboard

```
Core Improvements Progress:
├── Package Management:  ████████░░  80%
├── Kernel Core:         ██████░░░░  60%
├── Desktop/UX:          ████░░░░░░  40%
├── Security:            ████████░░  80%
├── Networking:          █████░░░░░  50%
└── Developer Experience:██████░░░░  60%

Overall: ██████░░░░  62%
```

---

*Improvements tracked quarterly. Next review: Q4 2025.*
