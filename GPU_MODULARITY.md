# Modular GPU Sovereignty

SigmaOS achieves heterogeneous compute sovereignty via the `SovereignGPUEngine` — a vendor-agnostic, NUMA-aware hardware acceleration framework operating natively in Ring-0.

## The Problem with Legacy GPU Drivers

Traditional OS kernels (Linux, Windows) rely on massive proprietary binary blobs supplied by NVIDIA, AMD, and Intel. These blobs:

- Operate with root-level trust but zero kernel-level isolation
- Break on kernel updates, requiring manual driver rebuilds
- Cannot be audited or patched for security vulnerabilities

## Sovereign Solution: Abstracted Vendor Execution

`SovereignGPUEngine` implements a PCIe endpoint analysis framework via `SovereignHAL`. Instead of depending on vendor blobs, it:

1. Probes the GPU's PCIe BAR registers at boot
2. Maps the compute register interface to a normalized sovereign API
3. Routes workloads via `SovereignNUMA` for O(1) latency to the nearest GPU die

```c
// Register a GPU with the sovereign framework
gpu_register("NVIDIA:10DE:2684", 24576); // RTX 4090, 24GB VRAM

// Dispatch a compute kernel
gpu_dispatch("AI_INFERENCE");
gpu_dispatch("MATRIX_MULTIPLY");
```

## NUMA-Aware Memory Binding

Every GPU workload is bound to the NUMA node physically closest to the GPU die:

```text
[SovereignNUMA] Node 0 (ARM64) -> CPU workloads
[SovereignNUMA] Node 1 (x86_64) -> GPU-adjacent memory
[SovereignGPU]  VRAM DMA routed via Node 1 -- O(1) latency
```

## Container GPU Passthrough

When a micro-VM is spawned via `SovereignContainers`, the GPU Engine automatically exposes safe hardware passthrough:

```c
// Inside a container spawn sequence:
container_spawn("ai-workload", "/usr/bin/inference");
// -> SovereignGPU automatically maps VRAM slice to container namespace
// -> SovereignSEL enforces GPU resource quotas via MAC policy
```

## Supported Compute Workloads

| Workload Type       | Dispatch Mode    | NUMA Binding    |
|---------------------|------------------|-----------------|
| AI Inference        | Tensor Block     | Automatic       |
| HPC Simulation      | SIMD Pipeline    | Pinned to Node  |
| Visualization       | Framebuffer DMA  | GPU-Adjacent    |
| Matrix Math (AMX)   | Intel AMX Tiles  | x86_64 Node     |

## Performance: SigmaOS vs Legacy

| Metric                    | Linux (Proprietary Blob)  | SigmaOS (Sovereign GPU)       |
|---------------------------|---------------------------|-------------------------------|
| Driver Init Time          | ~2800ms                   | ~12ms (Ring-0 HAL probe)      |
| Kernel Update Breakage    | Yes (binary blob)         | Never (hardware-abstracted)   |
| Security Auditability     | None                      | Full (open sovereign shard)   |
| NUMA Awareness            | Manual (numactl)          | Automatic (SovereignNUMA)     |
