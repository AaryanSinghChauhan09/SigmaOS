# Σ SigmaOS Installation Guide

Welcome to the industrial setup for SigmaOS. To ensure stability and trust, follow these high-fidelity steps.

## 🖥️ Hardware Requirements
*   **CPU**: x86_64 (Intel 8th Gen+ / AMD Ryzen) or ARM64 (RPi 4/5).
*   **RAM**: 4GB Minimum (8GB Recommended for AI shards).
*   **Storage**: 20GB NVMe/SSD (Shard lattice attestation required).
*   **GPU**: Vulkan 1.3 compatible (Intel i915+, AMDGPU-SI+, Nouveau).

## 🚀 Step-by-Step Setup
1.  **Flash Shard**: Download the `sigmaos-industrial.iso` and flash to USB via `dd` or BalenaEtcher.
2.  **Secure Boot**: Ensure TPM 2.0 is enabled for `SovereignAttestation`.
3.  **Lattice Partitioning**: Use the automatic partitioner to create a `LatticeFS` encrypted volume.
4.  **Neural Calibration**: Follow the post-install wizard to initialize your integrated AI assistant.

## 🛠️ Troubleshooting
*   **"Lattice Not Verified"**: Re-run `SovereignAttestation` tool from the live environment.
*   **Driver Missing**: Use the `sigma-pkg` to fetch the appropriate `DRV` shard from the P2P index.
