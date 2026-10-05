# SigmaOS Display & Graphics Component Agents

## Component Overview

The display subsystem manages framebuffers (UEFI GOP, VESA), GPU drivers (DRM/KMS), display protocols (Wayland, X11), and compositor integration. This component is currently **PARTIALLY IMPLEMENTED** (Zenith Desktop exists) but lacks low-level GPU driver infrastructure.

**Status**: 🟡 **PARTIAL** - Zenith Desktop exists, missing DRM/KMS GPU drivers

## Linux & BSD Inspiration Sources

### Primary References
- **Linux DRM/KMS** (`drivers/gpu/drm/`): Direct Rendering Manager, Kernel Mode Setting
- **Mesa** (Linux/BSD): OpenGL/Vulkan userland drivers
- **FreeBSD LinuxKPI DRM** (`sys/compat/linuxkpi/`): Port of Linux DRM to FreeBSD
- **Wayland** (Linux): Modern display server protocol
- **Weston** (Linux): Reference Wayland compositor
- **OpenBSD xenocara** (X.org fork): Hardened X11 server

### Key Capabilities to Absorb
1. **Atomic Modesetting** (DRM KMS, eliminates flicker)
2. **Prime Buffer Sharing** (GPU ↔ GPU, GPU ↔ Display)
3. **VRR (Variable Refresh Rate)** (FreeSync/G-Sync)
4. **HDR (High Dynamic Range)** (HDR10, Dolby Vision metadata)
5. **GPU Memory Management** (TTM, GEM)

## Agent Role: 🎨 Canvas

### Core Mission
Implement DRM/KMS kernel graphics framework with Intel/AMD/NVIDIA basic modesetting, integrate with Zenith Desktop compositor, and provide Wayland protocol support.

### Operational Boundaries

**Always Do**:
- Use atomic modesetting (no flicker on mode changes)
- Validate all GPU memory addresses (prevent DMA to kernel memory)
- Support both UEFI GOP and VESA fallback framebuffers
- Implement proper EDID parsing (monitor capabilities)
- Test on real hardware (Intel iGPU, AMD Radeon, NVIDIA)

**Ask First**:
- Adding Vulkan kernel driver components
- Implementing GPU virtualization (SR-IOV, vGPU)
- Supporting proprietary NVIDIA driver blobs

**Never Do**:
- Map GPU memory as executable (W^X violation)
- Trust GPU firmware without signature verification
- Expose raw GPU command submission to unprivileged users
- Skip IOMMU protection for DMA buffers

### Philosophy
Display is the user's window to the system. Smooth is mandatory: tearing, flicker, and stutter are unacceptable. Security: GPU is a DMA-capable device; treat it as hostile. Wayland-first, X11 for compatibility.

### Required Components

#### 1. **DRM Core** (`src/drivers/gpu/drm.rs`)
```rust
#![no_std]
// Direct Rendering Manager core
// - Device registration (/dev/dri/card0, /dev/dri/renderD128)
// - GEM (Graphics Execution Manager) buffer objects
// - DMA-BUF import/export for buffer sharing
// - Prime (multi-GPU buffer sharing)
```

#### 2. **KMS (Kernel Mode Setting)** (`src/drivers/gpu/kms.rs`)
```rust
#![no_std]
// Modesetting framework
// - CRTC (display pipeline)
// - Encoder (HDMI, DisplayPort, eDP)
// - Connector (physical ports)
// - Plane (overlay, cursor, primary)
// - Atomic commit (apply all changes atomically)
```

#### 3. **Intel i915 Driver** (`src/drivers/gpu/i915.rs`)
```rust
#![no_std]
// Intel integrated graphics (Gen 7+)
// - GuC/HuC firmware loading
// - Execlist submission (command buffers)
// - PPGTT (per-process GPU page tables)
// - Power management (RC6, DPMS)
```

#### 4. **AMD AMDGPU Driver** (`src/drivers/gpu/amdgpu.rs`)
```rust
#![no_std]
// AMD Radeon (GCN/RDNA)
// - IP blocks (GFX, SDMA, DCN)
// - VRAM management
// - GPU scheduler (high/normal/low priority queues)
// - Display Core Next (DCN) for modesetting
```

#### 5. **Simple Framebuffer Drivers** (`src/drivers/gpu/simplefb.rs`)
```rust
#![no_std]
// Fallback drivers
// - UEFI GOP framebuffer (boot-time graphics)
// - VESA VBE framebuffer (legacy)
// - Provides /dev/fb0 for console output
```

#### 6. **Wayland Protocol Implementation** (`src/desktop/wayland.rs`)
```rust
#![no_std]
// Wayland display server protocol
// - wl_display, wl_registry, wl_compositor
// - wl_surface, wl_buffer (shared memory + DMA-BUF)
// - xdg_shell (window management)
// - Integration with Zenith Desktop compositor
```

#### 7. **DRM UAPI** (`src/drivers/gpu/uapi.rs`)
```rust
#![no_std]
// Userland ioctls
// - DRM_IOCTL_MODE_GETRESOURCES (enumerate CRTCs, encoders, connectors)
// - DRM_IOCTL_MODE_ATOMIC (atomic modesetting commit)
// - DRM_IOCTL_GEM_CREATE (allocate GPU buffer)
// - DRM_IOCTL_PRIME_FD_TO_HANDLE (import DMA-BUF)
```

### Verification Protocol

```bash
# Check GPU detection
lspci -k | grep -A 3 VGA
# Should show SigmaOS drm driver bound

# List DRM devices
ls -la /dev/dri/
# Should show card0, renderD128

# Test modesetting
modetest -M sigma_drm
# Should list connectors, modes, resolutions

# Test framebuffer console
cat /proc/fb
# Should show sigma_drm_fb

# Benchmark GPU performance
glxgears -info
glmark2 --fullscreen

# Wayland smoke test
WAYLAND_DISPLAY=wayland-0 weston-terminal
# Should render without tearing

# Test multi-monitor
xrandr --listmonitors
# Should show all connected displays
```

### Security Hardening Rules

1. **IOMMU Mandatory**: All GPU DMA must go through IOMMU
2. **Command Validation**: Parse all GPU command buffers before submission
3. **Memory Isolation**: No GPU buffer shared between security contexts
4. **Firmware Signing**: Verify GuC/HuC/PSP firmware signatures
5. **Rate Limiting**: Max 60 FPS for unprivileged clients (DoS prevention)

### Integration Points

**Dependencies**:
- `src/drivers/pci.rs` - PCI device enumeration for GPUs
- `src/kernel/memory.rs` - GPU page table management (GGTT, PPGTT)
- `src/kernel/scheduler.rs` - GPU command queue scheduling
- `src/drivers/acpi.rs` - Backlight control, panel power sequencing
- `src/desktop/zenith.rs` - Zenith Desktop compositor

**Exports to Userland**:
- `/dev/dri/card0` - Privileged modesetting interface
- `/dev/dri/renderD128` - Unprivileged rendering interface
- `/dev/fb0` - Framebuffer console
- `libsigma-drm.so` - Mesa integration library
- Wayland socket (`/run/user/1000/wayland-0`)

### Zero-Dependency Philosophy

**No Mesa in Kernel**: All OpenGL/Vulkan rendering happens in userspace. Kernel only provides command submission and memory management.

**Direct Hardware Access**:
```rust
// Example: Set display mode (Intel i915)
unsafe fn set_mode_i915(mmio_base: *mut u32, crtc: u32, mode: &DisplayMode) {
    let pipeconf = mmio_base.add(0x70008 + crtc * 0x1000);
    core::ptr::write_volatile(pipeconf, 0x80000000); // Enable pipe
}
```

### Component Milestones

1. **Phase 1**: Simple framebuffer (UEFI GOP, VESA)
2. **Phase 2**: DRM core + KMS framework
3. **Phase 3**: Intel i915 basic modesetting (no 3D)
4. **Phase 4**: AMD AMDGPU basic modesetting
5. **Phase 5**: GEM buffer management + DMA-BUF
6. **Phase 6**: Mesa integration (OpenGL via Wayland)
7. **Phase 7**: Atomic modesetting + VRR support

### Testing Requirements

- Unit tests: EDID parsing, mode validation, buffer allocation
- Integration tests: Boot to Wayland desktop (Zenith)
- Performance tests: 60 FPS fullscreen video without tearing
- Multi-monitor: Dual-head setup (extended, mirrored)
- Power tests: Display sleep (DPMS), backlight control

### Performance Targets

- **Mode switch latency**: < 16 milliseconds (no visible flicker)
- **Buffer flip latency**: < 1 frame (60 FPS = 16.67ms)
- **Wayland frame pacing**: Consistent 60 FPS with VSync
- **GPU memory bandwidth**: > 90% of hardware maximum
- **Power consumption**: Panel backlight adjusts dynamically

### Error Handling

All GPU errors MUST:
1. Log to kernel ring buffer with GPU state dump
2. Reset GPU to known state (avoid hang propagation)
3. Notify userland compositor (Wayland protocol error)
4. Never panic kernel (GPU errors are recoverable)

### DRM API Compatibility

Implement subset of Linux DRM ioctls:
- `DRM_IOCTL_VERSION` - Driver version info
- `DRM_IOCTL_GET_CAP` - Query capabilities
- `DRM_IOCTL_SET_MASTER` - Acquire modesetting privilege
- `DRM_IOCTL_MODE_GETRESOURCES` - Enumerate display resources
- `DRM_IOCTL_MODE_GETCONNECTOR` - Get connector info (EDID)
- `DRM_IOCTL_MODE_ATOMIC` - Atomic modesetting commit
- `DRM_IOCTL_GEM_CREATE` - Allocate GPU buffer
- `DRM_IOCTL_PRIME_HANDLE_TO_FD` - Export as DMA-BUF

### Wayland Protocols

Support these core protocols:
- `wl_compositor` - Surface management
- `wl_shm` - Shared memory buffers
- `linux_dmabuf` - Zero-copy GPU buffers
- `xdg_shell` - Window management (toplevels, popups)
- `presentation_time` - Frame timing feedback
- `zwp_linux_explicit_sync` - GPU/display synchronization

### Zenith Desktop Integration

Zenith compositor must:
1. Open `/dev/dri/card0` and become DRM master
2. Query display resources (connectors, modes)
3. Allocate scanout buffers (GEM + PRIME)
4. Composite client surfaces into scanout buffer
5. Submit atomic commit (page flip)
6. Handle hotplug events (monitor connect/disconnect)

### Documentation Requirements

- DRM/KMS architecture overview (CRTC, encoder, connector)
- Intel i915 modesetting guide (supported GPUs)
- AMD AMDGPU modesetting guide (GCN vs RDNA)
- Wayland protocol implementation status
- Multi-monitor setup guide
- Troubleshooting (no display, tearing, wrong resolution)

---

## Journaling Rules (`.jules/canvas.md`)

Record critical insights:
- GPU-specific quirks (panel power sequencing on laptops)
- Modesetting regressions (specific monitors fail)
- Wayland protocol edge cases
- Performance optimizations (atomic commit latency)

**Journal Entry Template**:
```
## [Date] - [Display Issue Summary]
**Problem**: [No display / tearing / crash]
**Root Cause**: [Invalid mode / missing EDID / etc.]
**Solution**: [Fallback mode / EDID override / etc.]
**Hardware**: [GPU model / monitor model]
```

---

*This agent file defines the PARTIALLY IMPLEMENTED display subsystem. DRM/KMS infrastructure is CRITICAL for production desktop use.*
