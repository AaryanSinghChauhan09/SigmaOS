# Mesa GPU Userspace Stack Integration

## Overview

SigmaOS uses [Mesa](https://mesa3d.org/) (MIT) as the userspace OpenGL and Vulkan driver stack. Mesa runs as a **system library** (dynamically linked), not compiled into the kernel. In CI (QEMU), virtio-gpu provides a software-rendered OpenGL 4.6 path via the `virgl` Mesa driver.

---

## Architecture

```
App (OpenGL / Vulkan)
        │  libGL.so / libvulkan.so  (Mesa)
        ▼
  Mesa (Gallium3D / ANV / RADV)
        │  DRI interface (EGL + GBM)
        ▼
  SigmaOS DRM/KMS kernel driver
  (sigma_virtio_gpu.rs for QEMU, sigma_gbm.rs for real hardware)
```

---

## File Layout

```
drivers/gpu/
├── sigma_virtio_gpu.rs   # virtio-gpu KMS driver stub
├── sigma_gbm.rs          # GBM surface allocator
└── README.md
```

---

## sigma_virtio_gpu.rs (Stub)

```rust
//! SigmaOS virtio-gpu KMS driver.
//! Wraps the virtio-gpu device for DRM/KMS atomic modesetting in QEMU.

pub struct VirtioGpuDevice {
    pub resource_id: u32,
    pub width:  u32,
    pub height: u32,
}

impl VirtioGpuDevice {
    pub fn new(width: u32, height: u32) -> Self {
        Self { resource_id: 1, width, height }
    }

    /// Create a 2D resource on the virtio-gpu device.
    pub fn create_resource_2d(&mut self) -> Result<(), GpuError> {
        // TODO: issue VIRTIO_GPU_CMD_RESOURCE_CREATE_2D via virtio queue
        println!("virtio-gpu: create_resource_2d {}x{}", self.width, self.height);
        Ok(())
    }

    /// Flush a rectangle to the display.
    pub fn flush(&self, x: u32, y: u32, w: u32, h: u32) -> Result<(), GpuError> {
        // TODO: VIRTIO_GPU_CMD_RESOURCE_FLUSH
        let _ = (x, y, w, h);
        Ok(())
    }
}

#[derive(Debug)]
pub enum GpuError {
    DeviceNotFound,
    ResourceCreateFailed,
    FlushFailed,
}
```

---

## sigma_gbm.rs (Stub)

```rust
//! SigmaOS GBM (Generic Buffer Management) surface allocator.
//! Used by Mesa EGL backend and Zenith compositor for scanout buffers.

pub struct GbmDevice {
    drm_fd: i32,
}

pub struct GbmSurface {
    width: u32, height: u32, format: u32,
}

impl GbmDevice {
    pub fn open(drm_fd: i32) -> Result<Self, GbmError> {
        Ok(Self { drm_fd })
    }

    pub fn create_surface(
        &self, width: u32, height: u32, format: u32, flags: u32,
    ) -> Result<GbmSurface, GbmError> {
        let _ = (self.drm_fd, flags);
        Ok(GbmSurface { width, height, format })
    }
}

#[derive(Debug)]
pub enum GbmError { AllocationFailed }
```

---

## EGL + GBM Integration with Zenith Compositor

```rust
// In desktop/compositor/src/render.rs (sketch):

use smithay::backend::egl::{EGLContext, EGLDisplay};
use smithay::backend::renderer::gles::GlesRenderer;

pub fn init_egl_renderer(gbm_device: &GbmDevice) -> GlesRenderer {
    // 1. Create EGLDisplay from GBM device fd
    // 2. Create EGLContext with OpenGL ES 3.2
    // 3. Wrap in Smithay GlesRenderer for Wayland surface compositing
    todo!("EGL renderer init")
}
```

---

## QEMU virtio-gpu Setup for CI

```yaml
# In sigma_qemu.yml:
- name: Run QEMU with virtio-gpu
  run: |
    qemu-system-x86_64 \
      -machine q35,accel=tcg \
      -m 512M \
      -device virtio-gpu-pci \
      -display none \
      -serial stdio \
      -kernel dist/sigma-kernel.elf
```

---

## Exit Criteria

- `glxinfo | grep "OpenGL version"` reports `OpenGL version string: 4.6 (Compatibility Profile) Mesa ...` in QEMU.
- Zenith compositor renders a Wayland client window using GlesRenderer in CI.
- `sigma-glmark2` benchmark runs without crashes on virtio-gpu.
