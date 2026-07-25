# Σ SigmaOS v15.0 Zenith: Standalone Edition

## 🚀 The Sovereign Foundation

The **Standalone Edition** is the purest form of SigmaOS. It is designed to run directly on bare metal, assuming total control of the hardware lattice. It is the recommended choice for dedicated workstations and mission-critical industrial nodes.

### 🛠️ Key Features

- **Silicon-Direct Kernel**: Zero-overhead execution with no hypervisor layer.

- **Unified HAL**: Native drivers for NVMe, GPU (Vulcan-S), and PQC-hardened Wi-Fi.

- **Asynchronous Shard Ignition (ASI)**: Boot times under 1.2 seconds on NVMe.

- **Full Entropy Security**: Hardware-attested PQC key generation.

### 📥 Installation Guide (Bare Metal)

1. **Prepare Media**: Flash the `SigmaOS-v15.0-Zenith-Standalone.iso` to a USB 3.0+ drive using `sigma-pkg boot-create`.

2. **BIOS/UEFI**: Disable Secure Boot (SigmaOS uses its own PQC-attestation). Set SATA/NVMe mode to AHCI.

3. **Ignition**: Boot from USB. Select "Full Lattice Deployment".

4. **Partitioning**: The Sovereign Installer will automatically create the Shard Partition Table (SPT).

5. **Finalize**: Once the lattice is seeded, reboot and remove media.

### 💎 Exclusive Functions

- `lattice-hard-lock`: Physically isolate the node from the network via the S-WIFI shard.

- `silicon-direct-render`: Bypass the compositor for ultra-low latency compute.

---
[Return to Global Home](Home)
