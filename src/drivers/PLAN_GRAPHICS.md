# 🖥️ SigmaOS OOP Display & GPU Subsystem Development Plan

This document maps out the development plan for the **SigmaOS Graphics & Display Subsystem**. Inspired by the high-performance **DRM/KMS (Direct Rendering Manager / Kernel Mode Setting)** architecture and compositing systems of **Fedora** and **Arch Linux**, this plan outlines how SigmaOS handles accelerated pixel drawing, multiple viewport layers, and low-level display modes.

---

## 🏗️ 1. Graphics Subsystem Architecture

All graphics devices conform to a unified polymorphic output trait, exposing raw framebuffers or hardware accelerators while hiding complex registers or bus widths.

```
          +-------------------------------------------+
          |         Zenith Desktop Compositor         |
          +-------------------------------------------+
                                |
             +------------------+------------------+
             |                                     |
             v                                     v
+------------------------+             +------------------------+
|      DisplayDevice     |             |   GpuAccelerationCard  | (OOP Traits)
+------------------------+             +------------------------+
| - CGA / VGA Text       |             | - Intel Xe Graphics    |
| - VESA Framebuffer      |             | - PCIe discrete GPU    |
+------------------------+             +------------------------+
```

### 1.1 The Core Trait (`DisplayDevice`)
Every output driver must implement this abstract interface:

```rust
pub trait DisplayDevice: PeripheralDevice {
    /// Queries supported video resolution modes
    fn list_video_modes(&self) -> &[VideoModeInfo];

    /// Activates a specific resolution mode
    fn set_video_mode(&mut self, mode_id: u32) -> Result<(), &'static str>;

    /// Returns a direct memory-mapped pointer to the Linear Framebuffer (LFB)
    fn framebuffer_mut(&mut self) -> &mut [u32];
}
```

### 1.2 The GPU Accelerator Trait (`GpuAccelerationCard`)
Exposes modern hardware acceleration commands:

```rust
pub trait GpuAccelerationCard {
    /// Copies memory with hardware-accelerated Blit commands
    fn copy_rect_accelerated(&mut self, src_x: u32, src_y: u32, dst_x: u32, dst_y: u32, w: u32, h: u32);

    /// Flushes command ring buffers to the GPU processing pipeline
    fn flush_command_ring(&mut self);
}
```

---

## 🔌 2. Dual-Generation Graphics Matrix

We implement multiple classes of display drivers to support hardware from any generation:

### 2.1 Ancient: CgaGraphicsDriver & VgaTextModeDriver
- **CGA Graphics**: Operates on a tiny 16KB framebuffer mapped at memory address `0xB8000`. Supports retro 320x200 4-color palettes or 640x200 monochrome displays.
- **VGA Text**: Uses 80x25 characters with attribute bytes mapped at `0xB8000`, enabling classic command-line shells with near-instant rendering speeds.

### 2.2 Modern: VesaDriver & IntelXeGpuDriver
- **VESA Framebuffer**: Queries BIOS VBE 3.0 records to activate 32-bit linear framebuffers at high resolutions (e.g. 1920x1080).
- **Intel Xe Gpu**: Operates via PCIe bus configuration, maps ring buffers in high-memory BAR spaces, and processes accelerated 2D blits and composition commands with minimal CPU utilization.

---

## ⚡ 3. UDF Custom Shader Sandbox

For legacy system UI filtering, window animations, or real-time color corrections:
- Users register short **UDF shader blocks**.
- These run on incoming pixel streams inside the compositor drawing loops, performing safe, sandboxed transformations (e.g., color inversion, contrast stretching, or brightness control) directly on local memory without using any heap allocation.

---

## 📈 4. Roadmap and Milestones

1. **Phase 1: Basic Display Framework**
   - Define `DisplayDevice` trait and the generic `VideoModeInfo` structures in `src/drivers/graphics/mod.rs`.
2. **Phase 2: VGA Text Mode Console**
   - Implement cursor movement, terminal scrolling, and character background formatting directly on standard memory index `0xB8000`.
3. **Phase 3: VESA BIOS Extensions (VBE)**
   - Setup linear framebuffer addressing, write fast row pointer walks, and optimize bulk area clearing (`fill_rect`).
4. **Phase 4: Modern PCIe GPU Driver**
   - Initialize PCIe base address registers, enable command submissions, and allocate rendering rings.
5. **Phase 5: Zenith Compositor Integration**
   - Hook the GPU linear framebuffer output directly into the standard Zenith Compositor and window rendering tree.
