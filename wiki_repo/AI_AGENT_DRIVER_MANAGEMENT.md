# AI Agent Driver Management in SigmaOS

## Overview
SigmaOS incorporates an autonomous, AI Agent-driven hardware driver management system designed to detect, probe, optimize, rebuild, and secure hardware device drivers across bare-metal systems, microVMs, and containerized workloads.

AI agents (such as **Bolt** ⚡, **Sentinel** 🛡️, and **Palette** 🎨) interact directly with the kernel's Driver Management Subsystem (`src/driver/`), DKMS engine (`DkmsAbiRebuildEngine`), hardware compatibility layer (`src/hardware/compatibility.rs`), and Dilithium-5 Post-Quantum Cryptography (PQC) signature verifiers.

---

## 1. Core Architecture & Driver Subsystems

### 1.1 Driver Abstraction Hierarchy
Drivers in SigmaOS implement the `SigmaDriver` Rust trait or interface with native PCIe/USB/MMIO host controllers:
* **Storage Drivers**: `NvmePCIeHostController` (NVMe 64-byte submission/16-byte completion queues), `AhciSataController`, `VirtioBlkDriver`.
* **Network Drivers**: `IntelE1000eNicDriver` (MMIO ring descriptors), `VirtioNetDriverSimulator`, `IwlwifiDriver`.
* **Display Drivers**: `GopLinearFramebufferDriver` (VESA/UEFI GOP double-buffered blitting), `NvidiaPrimeDriver`.
* **Input & USB Drivers**: `XhciHostControllerDriver` (Extensible Host Controller), `HidInputDevice`.
* **Compatibility Wrappers**: `UbuntuCommonDriverEngine`, `DkmsAbiRebuildEngine`, `FreeBsdVtConsoleDriver`, `NetBsdRumpDriverKernelWrapper`.

---

## 2. AI Agent Operational Directives & Workflows

AI Agents operate under strict protocol boundaries when managing drivers:

### 2.1 Hardware Discovery & Telemetry Inspection
1. **PCIe & USB Bus Enumeration**:
   Agents query `/sys/bus/pci/devices/` or invoke `DeviceManager::enumerate_pci()` to retrieve `vendor_id`, `device_id`, subsystem IDs, and BAR registers.
2. **Hardware Compatibility Index**:
   Agents match detected hardware against `HardwareCompatibilityDatabase` (`src/hardware/compatibility.rs`), assigning compatibility tiers (Tier 1 Gold, Tier 2 Silver, Tier 3 Legacy/Degraded).

### 2.2 DKMS Dynamic ABI Rebuilding
When kernel updates occur or out-of-tree proprietary/open-source modules are ingested (e.g. NVIDIA, ZFS, Broadcom, Realtek):
1. **ABI Hash Check**: Agent calculates the target kernel ABI checksum using `DkmsAbiRebuildEngine`.
2. **Sandbox Compilation**: Agent triggers `SbuildChrootSandboxEngine` or `PoudriereBulkBuildEngine` to compile DKMS driver source in an isolated eBPF/Landlock sandbox.
3. **PQC Dilithium-5 Signing**: Agent verifies the compiled `.ko` driver payload with `Dilithium5KernelSignatureVerifier`. Unsigned or tampered modules are strictly rejected.

### 2.3 Automated Hotplugging & Driver Switching
* **GPU Switching (NVIDIA PRIME / Hybrids)**:
  AI Agents monitor workload latency classes (`WorkloadLatencyClass`). Under heavy 3D/AI workloads, agents invoke `NvidiaPrimeDriver::switch_mode(GpuSwitchMode::Nvidia)`; during power-saving idle states, agents revert to `GpuSwitchMode::Integrated`.
* **Fallback Safety Gates**:
  If a primary driver encounters kernel panics or ring buffer stalls, the agent triggers `SovereignDriverRecovery`:
  - Step 1: Unbind failing PCI driver (`/sys/bus/pci/drivers/.../unbind`).
  - Step 2: Fallback to generic safe driver (e.g. `GopLinearFramebufferDriver` for graphics or `e1000e` generic mode for network).
  - Step 3: Log telemetry report to `SystemDiagnosticReport`.

---

## 3. Security & Compliance Protocols for AI Agents

1. **Memory Safety & Bounds Checks**:
   AI agents must verify that queue pointers (`head`, `tail`, `doorbell`) in MMIO rings do not overflow driver memory bounds.
2. **Least Privilege Enforcement**:
   Driver agents run under Landlock V5 sandboxing constraints (`SovereignLandlockV5Guard`), preventing drivers from accessing unauthorized filesystem paths outside `/dev/` and `/sys/`.
3. **Audit Trails**:
   All driver probe, load, unload, and rebuild events are recorded in `ChainedAuditTrailLedger` for post-quantum audit compliance (ISO 27001).

---

## 4. Sample Agent Commands & CLI Interactions

```bash
# Query hardware telemetry and driver match status
sigma-driver status --json

# Trigger AI-agent DKMS rebuild for new kernel ABI
sigma-driver dkms-rebuild --kernel-version 6.10.0-sigma --module nvidia-open

# Verify Dilithium-5 PQC signature on driver module
sigma-driver verify-pqc /lib/modules/6.10.0-sigma/extra/nvme_target.ko

# Hot-switch GPU power profile via agent
sigma-driver gpu-switch --mode performance
```
