# Omarchy Linux Inspiration & Synthesis Architecture

## Overview
Inspired by the **Omarchy Linux** ecosystem, SigmaOS incorporates next-generation aesthetic, declarative desktop workflows, and developer ergonomics designed to outperform standard Linux, BSD, and Windows desktop environments.

---

## 1. Core Architectural Pillars

### A. Sovereign QuickShell HUD & Widgets (`OmarchyQuickShellBridge`)
- **Direct Wayland Layer-Shell Integration**: Native composited top bars, notifications, workspace switchers, and control centers without heavy X11/Electron runtimes.
- **Hardware-Accelerated Blur & Vibrancy**: Direct KMS/DRM rendering pipelines with configurable opacity, blur radius, and corner styling.

### B. Declarative Dynamic Theme Switcher (`OmarchyThemeLiveEngine`)
- **Zero-Restart Live Reloading**: Simultaneously updates GTK4/libadwaita, Qt6, Alacritty, Ghostty, Neovim, and the Zenith Wayland compositor via native IPC signals.
- **Curated High-Contrast Palettes**: Out-of-the-box support for Tokyo-Night, Catppuccin Mocha, Gruvbox, Nord, Everforest, Kanagawa, and Rose Pine.

### C. Web2App Sandboxed Application Isolation (`OmarchyAppSandbox`)
- **Micro-Sandboxing**: Bubblewrap and Landlock-backed application boundaries restricting access to read-only SSL certs, dedicated write paths, and granular GPU/audio permissions.
- **PWA Integration**: Turn modern web applications into isolated, native desktop citizens.

### D. Herdr Autonomous Desktop Agent (`OmarchyHerdrAiScheduler`)
- **Agentic Desktop Automation**: On-device AI task scheduler routing tasks, orchestrating window management, handling workspace context switching, and optimizing power usage during sustained high-load workloads.

---

## 2. Integration Mapping

| Omarchy Capability | SigmaOS Native Module | Performance Target |
|---|---|---|
| Hyprland / Layer-Shell Widgets | `src/distro/omarchy_master_synthesis.rs` | Sub-millisecond rendering |
| Systemwide Theme Synchronization | `src/distro/omarchy_master_synthesis.rs` | 0s latency (signal-driven) |
| Web2App Sandboxing | `src/distro/omarchy_master_synthesis.rs` | Zero privilege leakage |
| Autonomous Workspace & Task AI | `src/distro/omarchy_master_synthesis.rs` | Zero user intervention |
