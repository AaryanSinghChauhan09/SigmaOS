# GPU and Graphics

SigmaOS implements a native GPU abstraction layer, DRM/KMS driver interface, Vulkan/OpenGL compatibility, and a custom 2D compositor backend — all in Rust. GPU support covers NVIDIA (nouveau + proprietary shim), AMD (amdgpu), and Intel (i915/Xe).

---

## Architecture Overview

```
 ┌─────────────────────────────────────────────────────┐
 │              Applications / Games                    │
 │  Vulkan │ OpenGL │ WebGPU │ OpenCL / CUDA            │
 └──────────────────────┬──────────────────────────────┘
                        │ Mesa / NVML userspace (compat)
 ┌──────────────────────▼──────────────────────────────┐
 │           SigmaOS GPU Abstraction Layer (GAL)        │
 │  src/gpu/ │ src/graphics/                            │
 │  CommandBuffer │ MemoryHeap │ Pipeline │ Swapchain   │
 └──────────────────────┬──────────────────────────────┘
                        │ DRM/KMS
 ┌──────────────────────▼──────────────────────────────┐
 │               GPU Kernel Driver                      │
 │  AMDGPU (GFX10/11) │ i915/Xe │ nouveau              │
 └──────────────────────┬──────────────────────────────┘
                        │ PCI BAR / MMIO
 ┌──────────────────────▼──────────────────────────────┐
 │              Physical GPU Hardware                   │
 └─────────────────────────────────────────────────────┘
```

---

## DRM/KMS Layer (`src/gpu/`)

### Atomic Modesetting
- Plane / CRTC / encoder / connector abstraction
- Atomic commit: all changes applied atomically (no tearing during modeset)
- Universal planes: primary, cursor, overlay
- High dynamic range (HDR) metadata via `hdr_output_metadata` property

### Direct Rendering
- GEM (Graphics Execution Manager) buffer objects
- PRIME DMA-BUF sharing between GPU and CPU
- Sync objects (dma_fence) for explicit GPU/CPU synchronization
- DRM lease: VR compositors get exclusive display access

---

## GPU Memory Management

### Memory Heaps
| Heap | Location | Bandwidth | Use |
|------|----------|-----------|-----|
| VRAM | GPU local | 1–2 TB/s | Textures, framebuffers, RT data |
| GTT | System RAM (pinned) | 50–100 GB/s | Staging, readback |
| WC | Write-combined | ~20 GB/s | CPU → GPU upload |

### VRAM Eviction
- LRU-based eviction when VRAM pressure exceeds 90%
- Priority tiers: `critical` (framebuffer), `normal`, `evictable`
- Idle BO eviction: moves least-recently-used BOs to GTT

---

## Vulkan Support

SigmaOS provides a Vulkan ICD (Installable Client Driver):

### Supported Extensions (subset)
| Extension | Purpose |
|-----------|---------|
| `VK_KHR_swapchain` | Present to display |
| `VK_KHR_ray_tracing_pipeline` | Hardware ray tracing |
| `VK_KHR_dynamic_rendering` | Render pass simplification |
| `VK_EXT_mesh_shader` | Mesh shading pipeline |
| `VK_KHR_timeline_semaphore` | GPU/CPU sync |
| `VK_KHR_buffer_device_address` | GPU-side pointers |

---

## OpenGL / GLES Compatibility

Via Mesa compatibility layer:
- OpenGL 4.6 core profile
- OpenGL ES 3.2
- EGL for Wayland surface integration
- GLX (XWayland compatibility)

---

## GPU Compute

### OpenCL 3.0
- Kernel compilation via LLVM SPIRV backend
- Unified memory model: CPU + GPU share address space (iGPU)

### CUDA Compatibility (NVIDIA)
- Partial CUDA 12.x compatibility via `sigma-cuda-compat`
- Translates CUDA API calls to Vulkan Compute / ROCm

### ROCm / HIP (AMD)
- Full HIP runtime for AMD GPU compute
- PyTorch, JAX, TensorFlow via ROCm backend

---

## Display Features

### High Refresh Rate
- 144Hz, 165Hz, 240Hz, 360Hz support
- Variable Refresh Rate: FreeSync / G-Sync (Adaptive Sync)
- Low framerate compensation (LFC) for VRR

### Color Management
- Wide color gamut: DCI-P3, Adobe RGB, BT.2020
- HDR10, HDR10+, Dolby Vision metadata passthrough
- ICC profile auto-apply based on display EDID

### Multi-Monitor
- Up to 8 displays simultaneously
- Independent refresh rates per display
- Mixed DPI: 1x + 2x HiDPI in same session

---

## Framebuffer Driver (`src/drivers/framebuffer.rs`)

Early boot / fallback framebuffer:
- VESA/UEFI GOP linear framebuffer
- Pixel formats: RGB888, RGBA8888, BGR888
- Character rendering for pre-GPU console
- Handoff to DRM KMS after GPU driver loads

---

## AI GPU Management

SigmaOS AI runtime monitors GPU health and usage:
- **Thermal throttle prediction**: warns 5s before GPU throttles
- **VRAM pressure relief**: evicts stale textures before OOM
- **Workload routing**: offloads ML inference to GPU when idle
- **Power tuning**: adjusts GPU P-state based on load forecast

---

## Comparison vs Competitors

| Feature | Mint (Mesa/X11) | Omarchy (Hyprland/Vulkan) | **SigmaOS** |
|---------|----------------|--------------------------|-------------|
| Wayland-native | ✅ | ✅ | ✅ |
| Vulkan ICD | Mesa | Mesa | ✅ Native + Mesa |
| HDR | ❌ | ✅ | ✅ |
| VRR/G-Sync | ✅ | ✅ | ✅ |
| DRM lease (VR) | ❌ | ✅ | ✅ |
| AI GPU tuning | ❌ | ❌ | ✅ |
| Safe memory mgmt | ❌ | ❌ | ✅ Rust |

---

## Source Files

| File | Description |
|------|-------------|
| `src/gpu/` | GPU abstraction layer |
| `src/graphics/` | Rendering pipeline |
| `src/drivers/framebuffer.rs` | Early framebuffer |
| `src/compositor/` | Compositor GPU backend |
| `src/audio/` | Audio/video sync |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/gpu/`, `src/graphics/`, `src/drivers/framebuffer.rs`
> - Update Vulkan extension table when new extensions are implemented
> - Keep GPU model support list current (new AMD/NVIDIA/Intel architectures)
> - Update HDR/VRR feature table as compositor gains features
