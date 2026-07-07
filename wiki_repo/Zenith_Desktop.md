# Zenith Desktop Roadmap

## 1. Wayland Compositor Architecture
Zenith is the native display manager for SigmaOS, completely replacing legacy X11 graphics paths.
- **DRM/KMS Adaptation**: Renders directly to framebuffers via the kernel interface (`sigma_virtio_gpu.rs`).
- **Memory-Safe Composition**: Written entirely in Rust, preventing heap-exploit vector risks.
- **XWayland Sandbox**: Legacy X11 apps are isolated in dedicated virtual buffers, protecting the host system from input interception.

## 2. Accessibility Suite
Accessibility is integrated directly into the compositor pipeline:
- **TTS screen reader**: Hooked directly to client UI trees via system IPC.
- **Visual Filters**: GPU-level high-contrast scaling, color-blind correction filters, and magnification rendering.
- **Input Customization**: Integrated gesture detection and virtual on-screen inputs.

## 3. Customization Hub & UI Design
Zenith features a configuration panel for native system tuning:
- Unified theme configurations (CSS variables mapping).
- Layout configurations for tiling, stacking, and workspace views.
- Deep multilingual support supporting multiple active input methods (IMEs).

## 4. Roadmap Phases
- **Phase 1 (0–3m)**: VirtIO-GPU framing support and basic keyboard input routing.
- **Phase 2 (3–6m)**: DRM/KMS support for AMD/Intel graphics, client surface mapping, and basic panel layouts.
- **Phase 3 (6–9m)**: Accessibility screen-reader engine and XWayland integration.
- **Phase 4 (9–12m)**: Customization controls, theme manager, and multilingual input system.

## 5. Contributor Guidelines
- Follow memory safety rules, avoiding unsafe code in client interface composition.
- All interface additions must support high-contrast theme variations.
