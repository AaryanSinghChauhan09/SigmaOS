# Phase 2: Modularization & Profiles

## Overview

Phase 2 separates SigmaOS into clean, independently-togglable subsystems and introduces declarative OS build profiles — analogous to Ubuntu's Desktop, Server, and Core *flavours* but fully sovereign and POSIX-free.

---

## OS Build Profiles

Three first-class profiles ship with v15.0:

| Profile | Config File | Use Case |
|---------|------------|----------|
| `sigma-core` | `config/sigma.yaml` | Bare-metal, headless, POSIX-free sovereign kernel |
| `sigma-desktop` | `config/sigma.yaml` (Zenith DE ON) | Full Zenith desktop + GPU, Wi-Fi, audio drivers |
| `sigma-cloud` | `config/sigma.yaml` (HV + PQ-Net ON) | Distributed silicon sovereignty, hypervisor, PQC networking |
| `ubuntu-compat` | `config/ubuntu.yaml` | Debian/Ubuntu ABI shim, dual-boot, APT bridge |
| `bsd-compat` | `config/bsd.yaml` | FreeBSD newbus, immutable root, server-hardened |

### Selecting a Profile at Build Time

```bash
# CMake
cmake -B build \
  -DSIGMA_TARGET_OS=sigma \
  -DSIGMA_USE_ZENITH_DE=OFF \          # sigma-core
  -DSIGMA_USE_AI_ENGINE=ON \
  -DSIGMA_IMMUTABLE_ROOT=ON

# Makefile
make TARGET_OS=sigma SIGMA_PROFILE=sigma-core -j$(nproc)
```

---

## Subsystem Map (Rust/Zig rewrite in progress)

```
kernel/
├── core/
│   ├── memory/         # Sovereign VMM, slab allocator, PMM (Rewritten in Rust no_std)
│   ├── sched/          # MLFQ-MCS scheduler (Rewritten in Rust no_std)
│   ├── syscall/        # Sovereign syscall gate (Rewritten in Rust no_std)
│   └── hal/            # Hardware Abstraction Layer (Rewritten in Rust no_std & Zig freestanding)
├── net/                # Sovereign TCP/IP stack (IPv4/IPv6)
├── storage/            # SigmaFS + ZFS journal shim
├── telemetry/          # Zero-trust audit & telemetry
└── virt/               # Hypervisor (optional — SIGMA_USE_HYPERVISOR)
```

### USE-Flag Feature Toggles (CMake)

| Flag | Default | Description |
|------|---------|------------|
| `SIGMA_USE_HYPERVISOR` | `ON` | Enable VMM / hypervisor shard |
| `SIGMA_USE_AI_ENGINE` | `ON` | Neural AI engine for scheduling |
| `SIGMA_USE_ZENITH_DE` | `ON` | Zenith desktop environment |
| `SIGMA_USE_CRYPTFS` | `ON` | Sovereign CryptFS |
| `SIGMA_USE_PQ_NET` | `OFF` | Post-quantum TLS networking |
| `SIGMA_USE_WASM` | `OFF` | WASM/WASI runtime |
| `SIGMA_USE_BLUETOOTH` | `ON` | Bluetooth 5.3 stack |
| `SIGMA_USE_WIFI` | `ON` | 802.11ax Wi-Fi stack |
| `SIGMA_IMMUTABLE_ROOT` | `OFF` | Remount root read-only at boot |

---

## Profile YAML Schema

Each profile YAML (under `config/`) documents:

```yaml
target_os: sigma           # Build target: sigma | ubuntu | bsd
meta:
  description: "..."
  edition: "Zenith"
  version: "15.0"
drivers:
  directory: drivers/sigma
  modules: [sigma_nvme, sigma_wifi, sigma_usb]
kernel:
  scheduler: "MLFQ-MCS"
  security: "Zero-Trust + PQC"
features:
  hypervisor: true
  ai_engine:  true
  zenith_de:  true
  ...
```

---

## 🔗 Related Pages

- [Phase 1: Foundation & Branch Unification](Phase-1-Foundation-And-Branch-Unification)
- [Phase 3: Package & Update System](Phase-3-Package-And-Update-System)
- [OS Formats](OS_FORMATS)
- [Build Guide](BuildGuide)
