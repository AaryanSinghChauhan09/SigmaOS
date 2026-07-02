# 👥 Community, Documentation & Contributor Guide

> **"A sovereign OS is only as strong as the community that sustains it."**

Linux distros thrive because of their communities — Arch has its legendary Wiki, Debian has thousands of maintainers, EndeavourOS has welcoming forums. SigmaOS is building its community infrastructure with the same intentionality we bring to kernel engineering.

---

## 🆚 Community Infrastructure Comparison

| Feature | Arch Linux | Debian | EndeavourOS | SigmaOS |
|:--|:--|:--|:--|:--|
| Wiki | Legendary (80K+ articles) | Comprehensive | Good | **GitHub Wiki (growing)** |
| Forums | BBS (active) | Mailing lists | Discourse | **GitHub Discussions** |
| Package contributions | AUR (user repo) | Mentored uploads | AUR-based | **OmniPkg templates** |
| IRC / Chat | IRC + Matrix | IRC | Telegram | **Discord + Matrix** |
| Bounty program | None | None | None | **Sovereign Contributor Bounty** |

---

## 1. Documentation Strategy

### Wiki Structure
The SigmaOS GitHub Wiki is organized into 7 sections:

| Section | Purpose |
|:--|:--|
| 🗺️ Overview | Vision, strategy, roadmap, competitive positioning |
| ⚙️ Subsystem Docs | Kernel internals, scheduler, memory, drivers, networking |
| 🔒 Security | Crypto primitives, MAC, IDS, sandboxing, PQC |
| ⚡ Performance | Optimizer, benchmarks, GPU, daemons |
| 🏗️ Architecture | Declarative config, containers, modularity, ZFS |
| 🛠️ Developer Guide | Build system, toolchain, contribution guidelines |
| 💾 Recovery & Tools | Snapshots, rollback, forensic audit |

### Interactive Tutorials
The Zenith GUI will include **built-in interactive tutorials** that teach:
- Writing your first SigmaOS shard
- Understanding the 256-slot syscall dispatcher
- Building a driver using `SovereignDriverTemplate.c`
- Packaging an app as `.spk` for OmniPkg

---

## 2. Contributor Pipeline

### How to Contribute

```
1. Fork the repo → git clone
2. Choose a branch (see Branch-Guide.md)
3. Write code following Contributor-Guidelines
4. Submit PR with test evidence
5. Code review by maintainers
6. Merge → auto-deploy to wiki + branches
```

### Contribution Areas

| Area | Difficulty | Impact |
|:--|:--|:--|
| Wiki documentation | 🟢 Easy | High — helps onboarding |
| Driver shards | 🟡 Medium | High — hardware support |
| OmniPkg packages | 🟡 Medium | High — software ecosystem |
| Kernel subsystems | 🔴 Advanced | Critical — core OS |
| Security auditing | 🔴 Advanced | Critical — trust |

---

## 3. Sovereign Contributor Bounty Program

To accelerate ecosystem growth, SigmaOS offers bounties for high-impact contributions:

| Bounty Category | Reward | Example |
|:--|:--|:--|
| Port a critical app to `.spk` | ⭐⭐⭐ | nginx, PostgreSQL, Python runtime |
| Write a new hardware driver | ⭐⭐⭐ | USB 3.0, NVMe, Wi-Fi chipset |
| Security vulnerability report | ⭐⭐⭐⭐ | Kernel exploit, sandbox escape |
| Documentation improvement | ⭐ | Wiki page, tutorial, translation |
| Performance benchmark | ⭐⭐ | Comparative benchmark vs Linux |

---

## 4. Driver Development Quick-Start

Using `ecosystem/templates/SovereignDriverTemplate.c`:

```c
#include "sigma_kernel_types.h"
#include "hal/sigma_hal.h"

// 1. Probe: Detect hardware
sigma_status_t my_driver_probe(sigma_device_t* dev) {
    // Check PCI vendor/device ID
    return SIGMA_OK;
}

// 2. Init: Configure hardware
sigma_status_t my_driver_init(sigma_device_t* dev) {
    // Map MMIO registers, set up interrupts
    return SIGMA_OK;
}

// 3. Register with kernel
SIGMA_DRIVER_REGISTER("my_driver", my_driver_probe, my_driver_init);
```

---

## 5. Communication Channels

| Channel | Purpose | Link |
|:--|:--|:--|
| **GitHub Discussions** | Q&A, proposals, showcases | github.com/AaryanSinghChauhan09/SigmaOS |
| **GitHub Wiki** | Official documentation | Wiki tab on repo |
| **Discord** | Real-time chat, support | Invite link in README |
| **Matrix** | Federated chat (bridged to Discord) | #sigmaos:matrix.org |

---

## 6. Code of Conduct

SigmaOS follows the **Contributor Covenant v2.1**:
- Be respectful and inclusive
- Focus on constructive feedback
- Assume good intent
- Maintainers enforce standards fairly
