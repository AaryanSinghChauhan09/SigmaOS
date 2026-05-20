# GPU MODULARITY


SigmaOS achieves heterogeneous compute sovereignty via the `SovereignGPUEngine` � a vendor-agnostic, NUMA-aware hardware acceleration framework operating natively in Ring-0.


Traditional OS kernels (Linux, Windows) rely on massive proprietary binary blobs supplied by NVIDIA, AMD, and Intel. These blobs:



`SovereignGPUEngine` implements a PCIe endpoint analysis framework via `SovereignHAL`. Instead of depending on vendor blobs, it:

1. Probes the GPU's PCIe BAR registers at boot


// Register a GPU with the sovereign framework
gpu_register("NVIDIA:10DE:2684", 24576); // RTX 4090, 24GB VRAM

// Dispatch a compute kernel
gpu_dispatch("AI_INFERENCE");
gpu_dispatch("MATRIX_MULTIPLY");



Every GPU workload is bound to the NUMA node physically closest to the GPU die:


[SovereignNUMA] Node 0 (ARM64) -> CPU workloads
[SovereignNUMA] Node 1 (x86_64) -> GPU-adjacent memory
[SovereignGPU]  VRAM DMA routed via Node 1 -- O(1) latency



When a micro-VM is spawned via `SovereignContainers`, the GPU Engine automatically exposes safe hardware passthrough:


// Inside a container spawn sequence:
container_spawn("ai-workload", "/usr/bin/inference");
// -> SovereignGPU automatically maps VRAM slice to container namespace
// -> SovereignSEL enforces GPU resource quotas via MAC policy


