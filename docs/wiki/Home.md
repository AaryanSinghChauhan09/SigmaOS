# Welcome to the SigmaOS Wiki

SigmaOS is a sovereign, zero-dependency operating system built on a **Lattice Architecture**. It prioritizes extreme simplicity, explicit zero-trust security, and native orchestration.

> *"Fuse Whonix's paranoia, NixOS's reproducibility, Elementary's polish, and Flatcar's immutability into a single sovereign identity."*

---

## 🚀 Core Subsystems

| Area | Wiki Page | Status |
|---|---|---|
| Sovereign Orchestrator | [Container-Orchestrator](Container-Orchestrator.md) | ✅ Stable |
| Zenith Desktop & SDK | [Zenith-Desktop-SDK](Zenith-Desktop-SDK.md) | ✅ Stable |
| Sovereign System Profiles | [Sovereign-System-Profiles](Sovereign-System-Profiles.md) | ✅ Stable |
| Packaging & Immutability | [Sovereign-Packaging-and-Immutability](Sovereign-Packaging-and-Immutability.md) | ✅ Stable |
| Resilience & Control Center | [Resilience-and-Control-Center](Resilience-and-Control-Center.md) | ✅ Stable |
| Driver Support & Registry | [Driver-Support](Driver-Support.md) | ✅ Phase 5 |
| Sovereign LibC | [Sovereign-LibC-and-Dependencies](Sovereign-LibC-and-Dependencies.md) | ✅ Phase 5 |
| Sovereignty Architecture | [Sovereignty-Architecture](Sovereignty-Architecture.md) | ✅ Phase 7 |

---

## 🏗️ Architecture at a Glance

```
┌─────────────────────────────────────────────────────────┐
│                    ZENITH DESKTOP (Phase 4)             │
│  Compositor · Onboarding · App Store · Tiling WM       │
├─────────────────────────────────────────────────────────┤
│              SOVEREIGN ORCHESTRATOR (Phase 3)           │
│  Container Shards · Sandbox Bridge · IPC Bus            │
├──────────────────────────┬──────────────────────────────┤
│   KERNEL SUBSYSTEMS      │   USERLAND TOOLS             │
│  Driver Manager          │  sigma-coreutils (libc-free) │
│  Driver Registry (DKMS)  │  sigma-shell                 │
│  Immutable Update Daemon │  sigma-pod-cli               │
│  HW Test Suite           │  sigma-drv                   │
│  Sovereign LibC          │  Proton Bridge               │
├──────────────────────────┴──────────────────────────────┤
│                  SOVEREIGN LIBC (Phase 5)               │
│  sigma_malloc · sigma_memcpy · sigma_strcmp             │
│  sys_print (raw syscall, no glibc) · sigma_itoa         │
└─────────────────────────────────────────────────────────┘
```

---

## 📦 Key Design Principles

### 🔒 Zero External Dependencies (Phase 5)
Kernel-space code **never** links against glibc or musl. All memory, string, and I/O primitives are provided by the Sovereign LibC (`kernel/libc/sigma_libc_impl.c`). See [Sovereign-LibC-and-Dependencies](Sovereign-LibC-and-Dependencies.md).

### 🔧 Profile-Aware Driver Loading (Phase 5)
The Driver Manager loads only the modules relevant to the active hardware profile. Community drivers are sourced from the Sovereign Driver Registry as signed `.srecipe` build scripts — no pre-built binaries. See [Driver-Support](Driver-Support.md).

### 🔄 Immutable A/B Updates with DKMS (Phase 6)
The Update Daemon performs atomic A/B partition swaps. After every kernel swap it automatically triggers a DKMS rebuild of all tracked community drivers. If rebuild fails, it rolls back to the previous slot and notifies the user. See [Sovereign-Packaging-and-Immutability](Sovereign-Packaging-and-Immutability.md).

### 🧪 Boot-Time Hardware Tests (Phase 6)
The Hardware Test Suite (`kernel/tests/sigma_hw_test.cpp`) validates GPU, NIC, audio, and storage for the active hardware profile before the desktop launches. Critical failures boot into VGA safe mode (Rescuezilla model). See [Driver-Support](Driver-Support.md).

---

## 🛡️ Security Layers

*   **Zero-Trust VFS**: Explicit RBAC enforced at the filesystem level.
*   **Whonix-style Network Isolation**: GUI apps are sandboxed from the network by default.
*   **CAINE Forensic Mode**: All block devices mounted read-only; validated by HW test suite at boot.
*   **Cryptographic Updates**: Kernel images verified against a sovereign root key before slot swap.
*   **Heap Corruption Detection**: Every `sigma_malloc` block carries a `0xSIGMA5A5` magic cookie.

---

## 🛠️ Contributing

We enforce a strict quality gateway. Please ensure you run tests locally before submitting a PR.
*   `make test` — unit test suite.
*   `make hw_test PROFILE=standard` — hardware test suite (safe to run in QEMU).
*   `make valgrind_check` — memory leak detection.
*   The CI/CD pipeline builds all variants and executes static analysis.

See [CONTRIBUTING.md](../CONTRIBUTING.md) and the [Sovereign Build Registry](../SOVEREIGN_REGISTRY.md).

---

## 📚 Resources

*   [Developer Guide](../DEVELOPER_GUIDE.md)
*   [Architecture Blueprint](../Architecture.md)
*   [Issue Tracker](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
