# Hardware Acceleration & GPU Stack in SigmaOS

## Overview

SigmaOS features a zero-dependency, direct-to-metal graphics and compute subsystem designed to deliver high-frame-rate desktop rendering and hardware-accelerated compute without legacy X11/Mesa dependencies.

---

## Key Modules

- [`src/drivers/gpu.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/drivers/gpu.rs): Direct GPU abstraction, command ring buffer, and framebuffer management.
- [`src/distro/linux_bsd_inspirations.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/distro/linux_bsd_inspirations.rs) — [`DrmModeInfo`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/distro/linux_bsd_inspirations.rs#L1100-L1158): Atomic DRM/KMS modesetting and video timing boundaries.
- [`src/graphics/compositor.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/graphics/compositor.rs): High-efficiency hardware-accelerated window compositor.

---

## Capabilities

| Feature | SigmaOS Native Implementation | Advantages |
|---------|------------------------------|------------|
| **Atomic Modesetting** | Kernel KMS mode validation via `DrmModeInfo` | Zero screen tearing, sub-millisecond mode switching |
| **Vulkan/Direct3D Emulation** | Sovereign Shader Compiler (`src/toolchain/`) | Compiles SPIR-V directly to target GPU assembly |
| **Lock-Free Command Submission** | `SovereignRingBuffer` command stream | $O(1)$ lock-free submission from userspace to GPU rings |
| **Memory Isolation** | GPU Page Tables (GTT) with Landlock validation | Prevents rogue shaders from reading arbitrary VRAM |

---

## Architecture

```
Userspace App (Zenith Desktop / WebUI / Kuroko)
       │
       ▼ (Direct syscall or shared memory ring)
[Sovereign GPU Command Ring] ──> Lock-free SPSC queue
       │
       ▼
[Kernel GPU Driver / GTT] ────> Hardware MMIO / DMA batching
       │
       ▼
[Physical Display / Monitor] ──> VSync aligned scanning
```
