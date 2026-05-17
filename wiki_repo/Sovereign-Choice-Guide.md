# Sovereign Choice Guide

Welcome to the SigmaOS Sovereign Choice architecture. True sovereignty means *you* decide exactly what stays on your device.

## Universal OS Format Profiles

SigmaOS can adapt to any hardware scale through Modular Format Profiles. Choose your foundation during installation:

### 1. Legacy Profile

- **Target:** Old Hardware (≤512MB RAM, HDDs).

- **Included:** Lightweight Kernel, Minimal Drivers (VGA/IDE), Text-Only Console.

- **Excluded:** GPU Acceleration, Wi-Fi 6, Heavy PQC Crypto.

### 2. Modern Profile

- **Target:** High-End Desktops & Laptops.

- **Included:** Full GPU (Vulkan/DirectX), Wi-Fi 6E, Dilithium-5 PQC Security, Sovereign UI.

### 3. Cloud Profile

- **Target:** Servers & Datacenters.

- **Included:** Containerized Networking (S-NET), Strict Shard Isolation, S-VFS without GUI overhead.

### 4. RTOS Profile

- **Target:** Embedded & Real-Time Systems.

- **Included:** S-SCHED with deterministic deadlines, Minimal Footprint.

## Shard-Based Modularity

All components in SigmaOS are isolated shards. Using the `sigma-pkg` Sovereign Package Manager, you can add or remove subsystems on the fly.

- Want networking? `sigma-pkg install s-net`

- Removing graphics? `sigma-pkg remove s-gpu`

## Security & Privacy Controls (S-ARMOR)

Security is customizable:

- **Strict:** Zero-Trust, per-process memory sealing, full audit logging.

- **Balanced:** Secure defaults for desktop usage.

- **Permissive:** For offline, sandboxed development.

Take control. Welcome to the Lattice.
