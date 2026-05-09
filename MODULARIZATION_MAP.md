# SigmaOS Sovereign Lattice — Modularization Map

This page is the **single source of truth** for how SigmaOS code is organized into its 7-layer Sovereign Lattice. Every shard must belong to exactly one layer and communicate with other layers only through the defined interfaces.

---

## 🗺️ Architecture Overview

```text
┌──────────────────────────────────────────────────────────────────┐
│  L6 ·· Zenith UI / Display Server                                │
│         zenith.html  SovereignZenithDesktop  ZenithAccessibility │
├──────────────────────────────────────────────────────────────────┤
│  L5 ·· Industrial Ecosystem / Deployment                         │
│         SovereignOrbManager  SovereignCloudImage  SovereignLTS    │
│         SovereignDependencyGraph  SovereignKubelet               │
│         SovereignMicroEdition  SovereignForensicLattice          │
├──────────────────────────────────────────────────────────────────┤
│  L4 ·· AI & Automation                                           │
│         SovereignClawGateway  SovereignAgentQuotasExtended       │
│         SovereignAgentGovernance                                 │
├──────────────────────────────────────────────────────────────────┤
│  L3 ·· Security Fabric                                           │
│         SovereignPQC  SovereignFIPS  SovereignAnonymity          │
│         SovereignSandbox  SovereignTPM                           │
├──────────────────────────────────────────────────────────────────┤
│  L2 ·· System Services / Reliability                             │
│         VFS  SovereignEmergencySync  SovereignMonitor            │
│         SovereignContainerOrchestrator                           │
├──────────────────────────────────────────────────────────────────┤
│  L1 ·· Kernel Primitives / Drivers                               │
│         Scheduler  MemoryManager  SovereignLibC  SovereignHAL    │
│         SovereignARM64  NVMeCore  AMDGPUGraphics                 │
├──────────────────────────────────────────────────────────────────┤
│  L0 ·· Silicon / Boot                                            │
│         SovereignInit  SovereignCores  sigma_types  sigma_hal    │
└──────────────────────────────────────────────────────────────────┘
```

---

## 🚀 Competitive Absorption Matrix (The Crushers)

| Distro Target | SigmaOS Feature Shard | Mission Result |
|:---|:---|:---|
| **Whonix** | `SovereignAnonymity` | Hardened P2P network cloaking & anonymity. |
| **elementary** | `ZenithAccessibility` | Polished accessibility layers & high-contrast elegance. |
| **Clear Linux** | `SovereignCloudImage` | Automated AMI/GCP/Azure production sharding. |
| **Gentoo/Alpine** | `SovereignMicroEdition` | Zero-bloat minimal lattice config (<16MB RAM). |
| **RancherOS** | `SovereignContainerOrch` | Integrated OS-level container orchestration. |
| **CAINE** | `SovereignForensicLattice` | Read-only investigation and integrity auditing. |
| **AlmaLinux** | `SovereignFIPS` / `LTS` | FIPS-140 compliance and 10-year release support. |

---

## 🏗️ Layer 5: Industrial Evolution (2026-2027)

| Shard | Purpose | Status |
|:---|:---|:---|
| `SovereignCloudImage` | Automated AMI/GCP/Azure production image generation. | **OPERATIONAL** |
| `SovereignLTS` | 10-year stable release orchestration and ABI stability. | **OPERATIONAL** |
| `SovereignForensic` | Read-only forensic mount and integrity auditing. | **OPERATIONAL** |
| `SovereignMicro` | Extreme minimalism (Alpine/Gentoo parity). | **OPERATIONAL** |

## 🛡️ Layer 3: Security & Compliance

| Shard | Purpose | Status |
|:---|:---|:---|
| `SovereignAnonymity` | Network isolation and PQC-Tor circuit building. | **OPERATIONAL** |
| `SovereignFIPS` | Post-quantum cryptographic self-audit (FIPS-140-3). | **OPERATIONAL** |

## 🔌 Layer 1: Hardware Sovereignty

| Shard | Purpose | Status |
|:---|:---|:---|
| `SovereignARM64` | Deep tuning for RPi5 (BCM2712) in Sovereign Mode. | **OPERATIONAL** |
| `NvidiaTensorCore` | Silicon-direct offloading for neural inference. | **OPERATIONAL** |
