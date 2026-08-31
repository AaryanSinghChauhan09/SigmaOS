# NVIDIA PRIME Hybrid Graphics & Power Management Specification

## Architecture Overview
The NVIDIA PRIME Hybrid Graphics Subsystem in SigmaOS (`NvidiaPrimeEngine`) provides graphics switching, GPU offloading, D3cold power management, and DRM PRIME DMA-BUF buffer sharing for laptops and multi-GPU systems.

## Key Features Inspired by Linux & BSD Distros

### 1. Operating Profiles (`PrimeProfile`)
Inspired by Pop!_OS `system76-power`, Fedora `optimus-manager`, and Ubuntu `nvidia-prime`:
- `Offload`: On-demand render offload (`__NV_PRIME_RENDER_OFFLOAD=1`).
- `OnDemand`: Dynamic GPU sleep/wake switching based on active applications.
- `DiscreteNvidia`: High-performance mode running exclusively on NVIDIA discrete GPU.
- `IntegratedOnly`: Maximum power saver powering down discrete GPU into D3cold state.
- `ReversePrime`: Discrete GPU renders secondary displays and routes buffers to integrated display.

### 2. Runtime D3cold Power Management (`GpuPowerState`)
Inspired by FreeBSD `bbswitch` and Linux kernel PCIe power management (`/sys/bus/pci/devices/.../power/control`):
- `D0Active`: GPU fully active.
- `D3hot`: Low-power sleep state.
- `D3coldDynamicOff`: Dynamic power-off state suspending PCIe power when dGPU is idle.

### 3. Offload Environment Variables (`NvidiaPrimeOffloadConfig`)
Inspired by Arch Linux `env __NV_PRIME_RENDER_OFFLOAD=1`:
```bash
__NV_PRIME_RENDER_OFFLOAD=1
__GLX_VENDOR_LIBRARY_NAME=nvidia
__VK_LAYER_NV_optimus=NVIDIA_only
__EGL_VENDOR_LIBRARY_FILENAMES=/usr/share/glvnd/egl_vendor.d/10_nvidia.json
```

### 4. DRM PRIME DMA-BUF Buffer Sharing (`PrimeDmaBufShare`)
Cross-GPU zero-copy frame buffer export from NVIDIA discrete GPU rendering engine to integrated display server scanout pipeline.
