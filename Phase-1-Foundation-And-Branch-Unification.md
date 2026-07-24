# Phase 1: Foundation & Branch Unification

## Overview

Phase 1 establishes the bedrock of SigmaOS. All divergent experimental branches are absorbed into a single canonical `main` branch. The kernel gains strict modular boundaries and a unified driver interface.

---

## ✅ Completed

| Task | Status | Notes |
|------|--------|-------|
| Single `main` branch | ✅ Done | All 20+ branches merged & deleted |
| Modular driver dirs | ✅ Done | `drivers/linux/`, `drivers/sigma/`, `drivers/bsd/` |
| Root `CMakeLists.txt` | ✅ Done | `-DSIGMA_TARGET_OS=sigma\|ubuntu\|bsd` |
| Root `Makefile` update | ✅ Done | `TARGET_OS=sigma\|ubuntu\|bsd` |
| OS Profiles (YAML) | ✅ Done | `config/sigma.yaml`, `ubuntu.yaml`, `bsd.yaml` |
| CI matrix | ✅ Done | 3× `target_os` × 3× `profile` build jobs |
| 109 unit tests passing | ✅ Done | Vitest 10 suites, 109 tests |

---

## 🔧 Driver Framework

The Sovereign Driver Interface enforces a clean ABI boundary: OS-specific drivers live in their own directories and are compiled *only* when that `TARGET_OS` is selected.

```
drivers/
├── linux/          # Ubuntu/Debian ABI wrappers (TARGET_OS=ubuntu)
│   ├── ubuntu_compat.cpp
│   └── ubuntu_e1000.cpp
├── sigma/          # Native Sovereign drivers (TARGET_OS=sigma)
│   ├── sigma_nvme.cpp
│   ├── sigma_wifi.cpp
│   └── sigma_usb.cpp
└── bsd/            # FreeBSD newbus wrappers (TARGET_OS=bsd)
    ├── bsd_compat.cpp
    └── bsd_em.cpp
```

Each driver exposes a minimal C bridge (`extern "C"`) so the shared HAL boot code can call `sigma_nvme_init()` / `ubuntu_compat_init()` / `bsd_compat_init()` without knowing which target is active.

---

## 📐 Build Commands

```bash
# Native SigmaOS (default)
cmake -B build -DSIGMA_TARGET_OS=sigma && ninja -C build

# Ubuntu compat
make TARGET_OS=ubuntu -j$(nproc)

# BSD compat
cmake -B build -DSIGMA_TARGET_OS=bsd && ninja -C build
```

---

## 🔗 Related Pages

- [Phase 2: Modularization & Profiles](Phase-2-Modularization-And-Profiles)
- [Build Guide](BuildGuide)
- [Driver Development](Driver-Development)
- [CI Pipeline](CI-Pipeline)
