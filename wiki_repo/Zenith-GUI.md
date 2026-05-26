# 🖥️ Zenith GUI Compositor

> "A Glassmorphic, Sovereign UI rendered directly to the VGA Framebuffer."

SigmaOS eschews X11 and Wayland entirely. The UI is integrated deep into the kernel using a Painter's Algorithm.

## 1. The Compositor (`sigma_compositor`)
The Compositor is responsible for communicating with the Linear Framebuffer (LFB). It draws raw pixels.
It exposes basic primitives:
- `sigma_compositor_draw_rect()`
- `sigma_compositor_flip()` (Double Buffering)

## 2. The Window Manager (`sigma_wm`)
The Window Manager tracks structural windows. Each window has an X, Y, Width, Height, and Z-index.

When `sigma_wm_render_all()` is called, it iterates through the window stack and calls the compositor primitives.

## 3. Future Goals
- Implement hardware-accelerated Vulkan SPIR-V shader routing (bypassing the Vulkan SDK).
- Add alpha-blending for glassmorphic (blur) effects.
- Map the PS/2 mouse cursor to the GUI.
