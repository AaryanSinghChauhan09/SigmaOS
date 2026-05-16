# ðŸš€ SigmaOS: Zenith v15.0 Release Manifest

> **The sovereign microkernel, distributed across every possible silicon format.**

SigmaOS is available in multiple professional formats to ensure total industrial parity, portability, and performance across heterogeneous hardware environments.

---

## ðŸ›ï¸ Deployment Formats Status

| Format | Status | Primary Branch | Key Shards |
| :--- | :--- | :--- | :--- |

| **Standalone (Bare Metal)** | ðŸŸ¢ [STABLE] | `release/standalone` | `SovereignBoot`, `S-GPU`, `S-NVMe` |

| **Dual Boot (Interop)** | ðŸŸ¢ [STABLE] | `release/dual-boot` | `S-Compatibility`, `S-Partition`, `sigma-grub` |

| **App-Based (Runtime)** | ðŸŸ¡ [BETA] | `release/app` | `S-Wine`, `S-ARC`, `S-WASM` |

| **Browser-Based (Web)** | ðŸŸ¡ [BETA] | `release/browser` | `ZenithWebUI`, `S-Sandboxing` |

| **Core (Headless/Server)** | ðŸŸ¢ [STABLE] | `release/microkernel` | `S-SSH`, `S-Orchestrator`, `S-MM` |

| **Distributed (Mesh)** | ðŸŸ  [DEV] | `release/distributed` | `LatticeNet`, `S-Consensus` |

---

## ðŸ› ï¸ Build and Deployment Instructions

To build a specific edition of SigmaOS, switch to the corresponding branch and execute the industrial build command:

```bash

# 1. Switch to your desired format branch

git checkout release/standalone

# 2. Build the edition-specific ISO/Image

make zenith-standalone-iso
```

### ðŸ“¦ Unified Package Management

Every format supports the `sigma-pkg` utility for cross-format shard synchronization:

```bash

# Update local lattice to match global repository

sigma-pkg sync

# Install format-specific industrial layers

sigma-pkg layer standalone
```

---

## ðŸ›¤ï¸ Release Philosophy

SigmaOS follows a **Strict Parity Protocol**. While the deployment format varies (Bare metal vs. WASM), the underlying **Sovereign Kernel** and **Lattice Algorithms** remain identical. This ensures that a security audit performed on the Standalone version is mathematically valid for the App version.

---

*SigmaOS â€” One Kernel. Every Hardware. Absolute Sovereignty.*
