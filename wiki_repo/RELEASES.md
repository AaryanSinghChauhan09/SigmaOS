# 🚀 SigmaOS: Zenith v15.0 Release Manifest

> **The sovereign microkernel, distributed across every possible silicon format.**

SigmaOS is available in multiple professional formats to ensure total industrial parity, portability, and performance across heterogeneous hardware environments.

---

## 🏛️ Deployment Formats Status

| Format | Status | Primary Branch | Key Shards |
| :--- | :--- | :--- | :--- |

| **Standalone (Bare Metal)** | 🟢 [STABLE] | `release/standalone` | `SovereignBoot`, `S-GPU`, `S-NVMe` |

| **Dual Boot (Interop)** | 🟢 [STABLE] | `release/dual-boot` | `S-Compatibility`, `S-Partition`, `sigma-grub` |

| **App-Based (Runtime)** | ?? [STABLE] | `release/app` | `S-Wine`, `S-ARC`, `S-WASM` |

| **Browser-Based (Web)** | ?? [STABLE] | `release/browser` | `ZenithWebUI`, `S-Sandboxing` |

| **Core (Headless/Server)** | 🟢 [STABLE] | `release/microkernel` | `S-SSH`, `S-Orchestrator`, `S-MM` |

| **Distributed (Mesh)** | 🟠 [DEV] | `release/distributed` | `LatticeNet`, `S-Consensus` |

---

## 🛠️ Build and Deployment Instructions

To build a specific edition of SigmaOS, switch to the corresponding branch and execute the industrial build command:

```bash

# 1. Switch to your desired format branch

git checkout release/standalone

# 2. Build the edition-specific ISO/Image

make zenith-standalone-iso
```

### 📦 Unified Package Management

Every format supports the `sigma-pkg` utility for cross-format shard synchronization:

```bash

# Update local lattice to match global repository

sigma-pkg sync

# Install format-specific industrial layers

sigma-pkg layer standalone
```

---

## 🛤️ Release Philosophy

SigmaOS follows a **Strict Parity Protocol**. While the deployment format varies (Bare metal vs. WASM), the underlying **Sovereign Kernel** and **Lattice Algorithms** remain identical. This ensures that a security audit performed on the Standalone version is mathematically valid for the App version.

---

*SigmaOS — One Kernel. Every Hardware. Absolute Sovereignty.*
