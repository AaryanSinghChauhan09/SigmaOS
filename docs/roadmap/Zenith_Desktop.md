# Zenith Desktop Roadmap

## Wayland Compositor
Zenith is the native display stack for SigmaOS, bypassing legacy X11 bloat. It interfaces directly with the DRM/KMS subsystem to provide zero-latency compositing.

## Compatibility
- **XWayland Layer:** Ensures seamless execution of legacy X11 applications within a sandboxed frame.

## Accessibility Suite
- Native screen reader integration (TTS hooks).
- Hardware-accelerated high contrast, color inversion, and screen magnification built directly into the compositor render loop.

## Customization Hub
A unified `sigma-control-center` provides declarative styling (colors, typography, layouts) without relying on fragile GTK/Qt theme hacks.
