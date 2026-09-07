# HTML Language Dependency Reduction Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                       SigmaOS Zenith Wayland Compositor                         |
|     (GPU-Accelerated DRM/KMS Framebuffer, Vulkan / OpenGL ES Surface Render)    |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|               Native GTK / Qt / Cinnamon Desktop Environment Engine             |
|       (CinnamonThemeEngine, Gnome46MutterEngine, KdePlasma6Engine, Xfce418)     |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
| Plymouth GTK Bootsplash|  | Zero-Alloc Terminal TUI|  | Native Desktop Portals|
| (GtkPlymouthBootsplash|   | (ANSI Powerline REPL) |   | (Capability-Gated IPC)|
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                         SigmaOS Kernel & VFS Drivers                            |
|             (Pure Rust Kernel, Zero-Dependency klib, Multi-Arch HAL)            |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **Zenith Wayland Compositor**:
   - Replaces HTML web views with direct GPU-accelerated Wayland surface rendering on DRM/KMS framebuffers.

2. **Native GTK, Cinnamon & Desktop Engines**:
   - Native GTK and Cinnamon theme engines replace HTML control panels and web settings dashboards.

3. **ANSI Terminal TUI Engine**:
   - High-performance, zero-dependency ANSI terminal TUI buffers (`src/shell/zsh_bash_parity.rs`) replace web-based admin dashboards.

4. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
