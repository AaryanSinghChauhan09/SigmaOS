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
