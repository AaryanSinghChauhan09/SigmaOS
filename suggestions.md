# Σ SIGMAOS: SUGGESTIONS & PENDING FEATURES (v1.0)

The following features are conceptualized but require deeper silicon-level sharding to be fully "working as intended."

## 🛠️ PENDING SHARDS
1. **Universal App Container (UAC)**: An OCI-compatible container shard that runs Linux binaries inside SigmaOS without a kernel translation layer. (Currently in simulation).
2. **Cosmic-GPU-Bridge**: Direct memory access (DMA) for NVIDIA/AMD GPUs to offload SLAC legal audits to CUDA cores. (Awaiting hardware-sharding logic).
3. **Sovereign-Boot-UI**: A high-resolution splash screen that allows novices to choose their "Identity" (Lawyer, Scientist, Student) before the kernel initializes.
4. **Holographic Shell**: A 3D terminal interface for VR/AR devices (Apple Vision Pro, Quest 3).

## 🐞 KNOWN BEHAVIORS (Non-Simulated)
- **Mesh-Sync Latency**: In high-congestion local networks, task offloading may trigger an `E_TIMEOUT` shard.
- **Sovereign-LibC Alignment**: Certain AVX-512 instructions require manual 64-byte alignment in `sigma_kernel_types.h`.
