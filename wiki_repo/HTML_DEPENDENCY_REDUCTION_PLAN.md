# SigmaOS HTML Programming Language Dependency Reduction Plan

## 1. Executive Overview
SigmaOS aims to minimize dependency on web-based HTML/CSS DOM interfaces by migrating userland desktop components, control panels, installer interfaces, and system monitors to high-performance, native Rust Wayland compositors (`Zenith`), GTK/Qt desktop engines (`CinnamonThemeEngine`, `Gnome46MutterEngine`, `KdePlasma6Engine`), GTK Plymouth bootsplash engines (`GtkPlymouthBootsplashEngine`), and zero-dependency terminal TUIs (`src/shell/zsh_bash_parity.rs`).

## 2. HTML Dependency Assessment
An assessment of HTML files reveals legacy web frontend stubs in:
- `./web_ui/index.html`
- `./index.html`

While HTML/CSS DOM interfaces provided lightweight web browser prototypes during initial phases, relying on HTML web views introduces browser engine memory overhead (Electron/Chromium bloat), DOM text XSS security risks, and slow rendering performance compared to direct GPU-accelerated DRM/KMS Wayland surface rendering.

## 3. Native Rust UI Substitution Roadmap

### Phase 1: Native Wayland & DRM/KMS Display Compositing (`Zenith`)
- **Zenith Wayland Compositor**: Replaces browser-based HTML windows with native Wayland surface compositing (`Zenith`) rendered directly to Direct Rendering Manager (DRM) / Kernel Mode Setting (KMS) framebuffers.
- **Hardware Acceleration**: Executes hardware-accelerated Vulkan and OpenGL ES rendering pipelines without HTML DOM parsing overhead.

### Phase 2: Native GTK, Cinnamon & Desktop Environment Engines
- **Native GTK & Cinnamon Themes**: Replaces HTML control panels with native `CinnamonThemeEngine` (`src/compatibility/mint_linux.rs`), `Gnome46MutterEngine`, and `KdePlasma6Engine` (`src/desktop/ultimate_distro_desktop.rs`).
- **Plymouth GTK Bootsplash**: `GtkPlymouthBootsplashEngine` (`src/boot/plymouth.rs`) renders native boot animations and LUKS password prompts directly to framebuffer consoles without HTML web rendering.

### Phase 3: Zero-Dependency Terminal TUIs
- **Rich Terminal TUIs**: Shell REPL interfaces and system monitors (`btop`, `fastfetch`, `lscpu`, `journalctl`, `nvmeinfo`) render via zero-dependency ANSI terminal TUI buffers (`src/shell/zsh_bash_parity.rs`), eliminating the need for web-based admin dashboards.

---
*Maintained by the SigmaOS Desktop, UI & Zero-Dependency Steering Committee.*
