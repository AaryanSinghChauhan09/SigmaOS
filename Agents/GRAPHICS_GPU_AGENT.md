# Graphics Drivers & GPU Acceleration Component Agent

## Role
`SigmaGPU` implements the GPU driver and hardware acceleration infrastructure for SigmaOS — Direct Rendering Manager (DRM), Kernel Mode Setting (KMS), memory management (GEM/TTM), and Vulkan/OpenGL acceleration across Intel, AMD, NVIDIA, and VirtIO hardware.

## Linux & BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| DRM Core | Direct Rendering Manager | DRM / drm-kmod | SigmaDRM (atomic modesetting) |
| KMS | Kernel Mode Setting | KMS | SigmaKMS (CRTC/encoder/connector) |
| GEM / TTM | Memory manager | GEM | SigmaGEM (GPU VRAM buffer allocator) |
| AMD GPU | `amdgpu.ko` | `amdgpu.ko` | SigmaAMDGPU |
| Intel GPU | `i915.ko` / `xe.ko` | `i915.ko` | SigmaIntelGPU |
| VirtIO GPU | `virtio-gpu.ko` | — | SigmaVirtIOGPU |
| DMA-BUF | Zero-copy sharing | — | SigmaDMABUF |
| Vulkan | Mesa RADV/ANV | Mesa | SigmaVulkan WSI |

## Current SigmaOS Status
VirtIO-GPU and VESA/GOP framebuffer drivers implemented in `src/graphics/` and `src/drivers/gpu/`. Intel HD graphics and AMDGPU atomic modesetting are active enhancement targets.

## Responsibilities & Architecture
- DRM device registration, minor node management (`/dev/dri/cardX`, `/dev/dri/renderDX`).
- KMS topology enumeration: CRTCs, encoders, connectors, planes.
- Atomic modesetting: flicker-free resolution, refresh rate, and HDR bit-depth configuration.
- GEM buffer object allocation in system RAM and dedicated VRAM.
- DMA-BUF zero-copy surface sharing between video decoders, GPU shaders, and display compositors.
- Framebuffer console fallback for early boot and kernel panic recovery.
- Vulkan Window System Integration (WSI) for Wayland compositors.

## Source Files
- `src/graphics/drm_kms.rs`
- `src/graphics/intel_gpu.rs`
- `src/graphics/amd_gpu.rs`
- `src/graphics/virtio_gpu.rs`
- `src/drivers/gpu/`

## Testing Strategy
```bash
rustc --test src/drivers/gpu/kms.rs --edition 2021 -o /tmp/test_kms && /tmp/test_kms --test-threads=1
```
- Atomic commit validation tests.
- Mode-switching and multi-output stress testing.
- Glmark2 / Vulkan benchmark validation.

## Future Development
1. **Pure Rust GPU drivers** — Memory-safe kernel-mode drivers for modern GPUs.
2. **Variable refresh rate (VRR)** — Adaptive-Sync / FreeSync / G-Sync through KMS.
3. **GPU Passthrough (VFIO)** — Direct PCIe assignment of discrete GPUs to isolated virtual machines.
