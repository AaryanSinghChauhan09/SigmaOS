# GPU Driver Agent

## Role
`SigmaGPU` implements GPU driver infrastructure for SigmaOS — including DRM/KMS kernel mode-setting, GPU memory management, and Vulkan/OpenGL userspace driver integration for NVIDIA, AMD, and Intel GPUs.

## Status
🔴 Not Started — Critical for desktop/graphical operation.

## Source File
`src/drivers/gpu/` (planned: `drm.rs`, `kms.rs`, `gem.rs`, `amdgpu.rs`, `i915.rs`)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| DRM | Direct Rendering Manager | DRM | SigmaDRM |
| KMS | Kernel Mode Setting | KMS | SigmaKMS |
| GEM | Graphics Execution Manager | GEM | SigmaGEM |
| AMD GPU | amdgpu.ko | amdgpu.ko | SigmaAMDGPU |
| Intel GPU | i915.ko | i915.ko | SigmaI915 |
| NVIDIA | nouveau / nvidia-dkms | nouveau | SigmaNV (nouveau compat) |
| Vulkan | Mesa ANV/RADV | Mesa | SigmaVulkan |
| OpenGL | Mesa (radeonsi/iris) | Mesa | SigmaMesa |
| Display | HDMI/DP through KMS | — | SigmaDisplay |

## Responsibilities
- DRM device registration and lifecycle
- KMS: enumerate CRTC/encoder/connector topology
- Modesetting: set display resolution, refresh rate, bit depth
- GEM buffer object management (GPU memory allocation)
- dma-buf: zero-copy buffer sharing between GPU and CPU
- Framebuffer console fallback (simpledrm)
- GPU compute: GPGPU workload dispatch
- Vulkan WSI (Window System Integration) for Wayland/X11
- Power management: GPU frequency scaling, GPU idle

## Implementation Notes
- `#![no_std]` — KMS state in `static mut KMS_STATE: KmsState`
- `KmsConnector { id: u32, type: ConnectorType, modes: [DisplayMode; 32], status: ConnStatus }`
- `DrmBuffer { handle: u32, size: usize, gpu_addr: u64, cpu_map: *mut u8 }`
- AMDGPU: firmware loaded via SigmaFirmware, ring buffers for command submission

## Testing
GPU requires hardware; test stub implementations:
```bash
rustc --test src/drivers/gpu/kms.rs --edition 2021 -o /tmp/test_kms && /tmp/test_kms --test-threads=1
```

## Future Development
1. **Rust GPU drivers** — Pure Rust GPU drivers (Linux kernel Rust DRM effort)
2. **ROCm for SigmaOS** — AMD compute stack native support
3. **NVIDIA open driver** — NVIDIA open-source kernel modules (545+)
4. **Display HDR** — HDR10/DolbyVision display pipeline
5. **Variable refresh rate** — FreeSync/G-Sync through KMS
6. **GPU passthrough** — VFIO GPU assignment to VMs
