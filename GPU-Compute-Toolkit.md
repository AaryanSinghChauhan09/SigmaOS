# GPU Compute Toolkit

Sovereign alternative to CUDA/ROCm that does **not** depend on vendor
proprietary user-space libraries.

## Architecture
```
User App
   └─ SigmaGPU API (sovereign, no CUDA/ROCm)
         └─ SovereignHAL GPU backend
               ├─ NVIDIA (via open-firmware commands)
               ├─ AMD (via AMDGPU register map)
               └─ Intel (via i915-compatible open spec)
```

## Goals
- Compute kernels launched via structured shard messages
- Deterministic memory mapping (GPU VRAM ↔ RAM) with cryptographic attestation
- Zero vendor lock-in

## Roadmap
- [ ] Shader compiler (SPIR-V front-end)
- [ ] Command buffer submission
- [ ] Memory allocator (GPU VRAM)
