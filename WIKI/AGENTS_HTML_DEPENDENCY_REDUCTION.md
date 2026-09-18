# AI Agent Guidelines for HTML Language Dependency Reduction in SigmaOS

## 1. Overview
SigmaOS enforces a strict policy reducing external HTML/WebUI dependencies in favor of native Wayland compositing (`Zenith`), GTK/Cinnamon desktop engines, GTK Plymouth bootsplash engines, and zero-dependency ANSI terminal TUIs.

## 2. Guidelines for AI Agents

### 2.1 Native Desktop & Wayland Compositing First
- **Avoid HTML WebViews**: AI agents managing desktop controls or applications must interact with native Wayland display portals (`DesktopPortal`) and GTK/Qt desktop engines (`CinnamonThemeEngine`, `Gnome46MutterEngine`) rather than launching web browser HTML views.
- **Direct DRM/KMS Framebuffers**: System interfaces render directly to DRM/KMS hardware framebuffers (`GtkPlymouthBootsplashEngine`) with zero HTML DOM parsing latency.

### 2.2 Rich Terminal TUI Dashboards
- **ANSI Terminal Buffers**: System monitoring and administrative workflows render using native ANSI TUI components (`PowerlinePromptBuilder`, `btop`, `journalctl`) built with zero-dependency `klib` primitives.

---
*Maintained by the SigmaOS Desktop & Architecture Steering Committee.*
