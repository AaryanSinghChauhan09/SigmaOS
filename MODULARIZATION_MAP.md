# SigmaOS Sovereign Lattice — Modularization Map

This page is the **single source of truth** for how SigmaOS code is organized into its 7-layer Sovereign Lattice. Every shard must belong to exactly one layer and communicate with other layers only through the defined interfaces.

---

## 🗺️ Architecture Overview

```text
┌──────────────────────────────────────────────────────────────────┐
│  L6 ·· Zenith UI / Display Server                                │
│         zenith.html  SovereignZenithDesktop  SovereignGUI        │
├──────────────────────────────────────────────────────────────────┤
│  L5 ·· Industrial Ecosystem                                      │
│         SovereignOrbManager  SovereignUnifiedPkg  SovereignSnap  │
├──────────────────────────────────────────────────────────────────┤
│  L4 ·· AI & Automation                                           │
│         SovereignClawGateway  SovereignAgentCore  OpenClawHub    │
├──────────────────────────────────────────────────────────────────┤
│  L3 ·· Security Fabric                                           │
│         SovereignPQC  SovereignQKD  SovereignSandbox  SovereignTPM│
├──────────────────────────────────────────────────────────────────┤
│  L2 ·· System Services                                           │
│         VFS  SovereignIPC  SovereignMonitor  SovereignDiag       │
├──────────────────────────────────────────────────────────────────┤
│  L1 ·· Kernel Primitives                                         │
│         Scheduler  MemoryManager  SovereignLibC  SovereignHAL    │
├──────────────────────────────────────────────────────────────────┤
│  L0 ·· Silicon / Boot                                            │
│         SovereignInit  SovereignCores  sigma_types  sigma_hal    │
└──────────────────────────────────────────────────────────────────┘
```

---

## 📦 Canonical Include Map (`include/core/sigma_modmap.h`)

Rather than using fragile relative paths, every shard `#include`s the canonical modmap first:

```cpp
#include "core/sigma_modmap.h"

// Then use the defined aliases, e.g.:
#include SIGMA_INC_PQC      // resolves to "security/sigma_pqc.h"
#include SIGMA_INC_HAL      // resolves to "hal/sigma_hal.h"
```

| Alias | Resolves To | Layer |
|:---|:---|:---|
| `SIGMA_INC_TYPES` | `core/sigma_types.h` | L0 |
| `SIGMA_INC_HAL` | `hal/sigma_hal.h` | L0 |
| `SIGMA_INC_MEM` | `sigma_mem.h` | L1 |
| `SIGMA_INC_LOG` | `sigma_log.h` | L1 |
| `SIGMA_INC_IPC` | `ipc/sigma_ipc.h` | L2 |
| `SIGMA_INC_MONITOR` | `observability/sigma_monitor.h` | L2 |
| `SIGMA_INC_DIAG` | `observability/sigma_diag.h` | L2 |
| `SIGMA_INC_VFS` | `vfs.h` | L2 |
| `SIGMA_INC_NET` | `sigma_net.h` | L2 |
| `SIGMA_INC_FIREWALL` | `network/sigma_aether_firewall.h` | L2 |
| `SIGMA_INC_PQC` | `security/sigma_pqc.h` | L3 |
| `SIGMA_INC_SANDBOX` | `security/sigma_sandbox.h` | L3 |
| `SIGMA_INC_QKD` | `security/SovereignQKD.hpp` | L3 |
| `SIGMA_INC_CLAW` | `ai/sigma_claw.h` | L4 |
| `SIGMA_INC_NEURAL` | `ai/sigma_neural.h` | L4 |
| `SIGMA_INC_WORKFLOW` | `ai/sigma_workflow.h` | L4 |
| `SIGMA_INC_PKG` | `sigma_pkg.h` | L5 |
| `SIGMA_INC_SNAP` | `ui/sigma_snap.h` | L5 |
| `SIGMA_INC_DISPLAY` | `sigma_displayserver.h` | L6 |

---

## 📂 Directory to Layer Mapping

| Directory | Layer | Description |
|:---|:---:|:---|
| `kernel/core/boot/` | L0 | Bootloader, CPU init, early hardware bringup |
| `kernel/core/hal/` | L0–L1 | Hardware Abstraction Layer + driver registry |
| `kernel/core/memory/` | L1 | PMM, VMM, slab allocator |
| `kernel/core/system/` | L1 | Scheduler, process model, syscall gate |
| `kernel/core/ipc/` | L2 | Sovereign IPC bus, EventBus |
| `kernel/core/network/` | L2 | Network stack, Aether Firewall |
| `kernel/core/observability/` | L2 | eBPF monitor, telemetry |
| `kernel/core/security/` | L3 | PQC, QKD, Sandbox, TPM |
| `kernel/core/automation/` | L4 | Workflow Engine, task automation |
| `kernel/shards/ai/` | L4 | Agent core, OmniTool, Neural Nexus |
| `kernel/core/industrial/` | L5 | Orb Manager, unified packaging |
| `kernel/core/ui/` | L6 | Zenith compositor, display server |
| `drivers/` | L1 | GPU, NVMe, NIC, USB, Wi-Fi shards |
| `userland/` | L6+ | Spotlight, OmniShell, user apps |

---

## 🔒 Inter-Layer Communication Rules

> **A lower-layer shard MUST NOT include headers from a higher layer.**

| Allowed | Forbidden |
|:---|:---|
| L3 includes L0/L1/L2 headers | L1 including `security/` headers |
| L5 calls L3 via C Bridge | L3 calling into L5/L6 directly |
| L4 AI service uses IPC to reach L2 | L4 writing to kernel memory directly |

---

## 🛠 Key Stabilization Fixes Applied

| Shard | Problem | Fix |
|:---|:---|:---|
| `SovereignOrbManager.cpp` | `SovereignQKD.hpp` not found; `Security` undeclared | Fixed include path to `security/SovereignQKD.hpp`; used fully-qualified namespace |
| `SovereignOrbManager.cpp` | Two adjacent `const char*` params easily swapped | Added strong-type wrappers `OrbName{}` / `OrbSig{}` |
| `SovereignOrbManager.cpp` | C-style vararg logging (`sigma_printf`) | Replaced with `sigma_log_info()` macros throughout |
| `SovereignOrbManager.cpp` | `listOrbs()` not `const` | Marked method `const` |
| `SovereignOrbManager.cpp` | `m_installed_orbs` uninitialized | Default member initializer `{0u}` |
| `SovereignHAL.cpp` | Drivers not required at boot | Added mandatory `gpu_init`, `nvme_init`, `nic_init`, `usb_init`, `wifi_init` calls |
| `SovereignClawGateway.cpp` | AI parameter swap risk | Implemented strong-type wrappers `WorkflowID` / `AgentType` |
| `SovereignPQC.cpp` | C-style casts & magic numbers | Migrated to `static_cast` and lattice constants |
| `SovereignSandbox.cpp` | Missing aria-labels & labels | Hardened security policy and added Zenith UI accessibility markers |
| `SovereignMonitor.cpp` | Magic numbers in telemetry | Replaced with constexpr industrial constants |
| `SovereignSnap.cpp` | Pure C-style implementation | Refactored to `SigmaObject` singleton in UI namespace |
| `zenith.html` | Duplicate IDs & a11y violations | Renamed `command-input` and added `aria-label` to all inputs |

---

*This page is auto-maintained. For contribution guidelines, see [CONTRIBUTING.md](../CONTRIBUTING.md).*
