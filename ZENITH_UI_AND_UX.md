
# Zenith UI & Personalisation


SigmaOS does not use generic display servers like X11 or Wayland. Instead, it embeds the **Zenith UI Compositor** directly as an isolated module, allowing zero-latency, GPU-accelerated bare-metal graphics.

Located in `modules/ui/zenith_compositor.c` and `modules/ui/user_profile.c`.


## Competitive Advantages (USPs)



### 1. Bare-Metal Glassmorphism

- **Standard Linux**: Getting smooth, alpha-blended glassmorphism usually requires heavy compositor daemons (like Picom) layered over X11.
- **SigmaOS USP**: The `zenith_compositor.c` maps directly to the UEFI Framebuffer and implements native alpha-blending in C. Windows can natively request `bg_color.a` values to become transparent, instantly rendering a premium, frosted-glass interface without any high-level JavaScript or Python bloat.


### 2. Declarative Automations (NixOS Parity)

- **Standard Linux**: You manage settings via scattered `.dotfiles` in `/home`.
- **SigmaOS USP**: SigmaOS features a declarative `user_profile_t`. When the system boots, `automation_apply_profile()` reads the user's declarative preferences and configures the *entire kernel*. 
  - If `performance_bias == 2`, it hot-swaps the kernel scheduler to Priority/Realtime mode natively.
  - It automatically spawns required daemons via capability tokens.


### 3. Focus Mode UX

- Built directly into the kernel's automation loop. If Focus Mode is enabled via the user profile, the Zenith UI compositor automatically drops rendering for non-critical notification overlays, and the scheduler deprioritizes background networking tasks.


### 4. Kernel-Native Widget Engine

- **Dashboard Overlays (`widget_engine.c`)**: Unlike macOS widgets or Conky which run as heavy user-space apps polling system files, SigmaOS widgets run inside the kernel UI compositor. They have direct access to memory structs, providing 0-latency telemetry (CPU, Sovereign Tokens, AI Accelerators) with near-zero overhead.


### 5. Adaptive Typography

- **Environmental Scaling (`adaptive_typography.c`)**: Fonts don't just scale by screen DPI. The OS reads physical ambient light sensors. In direct sunlight, the kernel dynamically boosts font weight and contrast. In dark rooms, it thins fonts to reduce eye strain.


### 6. Tiling Window Manager & Micro-Animations

- **Window Manager (`window_manager.c`)**: Supports declarative workspaces with hot-swappable layouts (Floating, BSP Tiling, Monocle). 
- **Spring Physics (`animations.c`)**: Moving and resizing windows doesn't use static linear interpolation. SigmaOS natively implements Hooke's Law spring physics (tension & friction) in C. Windows bounce, snap, and scale with 120Hz fluidity.


### 7. Global Hotkey Automations

- **Input Routing (`hotkeys.c`)**: Hotkeys aren't handled by a user-space daemon (like `sxhkd`). They are intercepted directly at the HID driver level. Pressing `SUPER + ENTER` to launch a terminal is processed in Ring 0, executing instantly without context-switch latency.
