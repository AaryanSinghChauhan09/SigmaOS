# Zenith Wayland Compositor & Display Stack

## 1. Wayland-Native by Default
SigmaOS abandons the legacy X11 display server stack entirely in favor of a native Wayland compositor called **Zenith**. 
- Built in pure, memory-safe Rust.
- Interfaces directly with the kernel's DRM/KMS (Direct Rendering Manager / Kernel Mode Setting) subsystems.

## 2. XWayland Compatibility
To ensure seamless transitions for legacy software, Zenith integrates a highly optimized XWayland layer. X11 applications run sandboxed and isolated, tricking them into believing a traditional X server is running, while Zenith composites their output securely.

## 3. Accessibility Suite
Zenith is designed from day one with deep accessibility hooks.
- **Screen Reader Hooks:** Native text-to-speech integration via dbus/IPC bridging.
- **High Contrast & Magnification:** Rendered directly by the GPU compositor for zero-latency screen scaling and color inversion.

## 4. Native UI Toolkit
Instead of forcing the entire GTK or Qt stack into the base system, SigmaOS ships a lightweight native UI toolkit used exclusively for core system apps (Installer, Control Center, Network Manager). This dramatically reduces the TCB (Trusted Computing Base) footprint of the graphical environment.
