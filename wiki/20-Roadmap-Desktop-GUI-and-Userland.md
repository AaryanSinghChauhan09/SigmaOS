# SigmaOS Future Development Roadmap: Desktop GUI, Zenith Compositor & Userland Tooling

This roadmap details the future evolution of the bare-metal Zenith Desktop Environment, Wayland protocol adapters, Linux Mint Mint-Y/X theme parity engines, and Omarchy developer power toolchains in SigmaOS.

---

## 1. Executive Summary & Core Desktop Philosophy

SigmaOS features the **Zenith Desktop Environment**, a direct bare-metal GPU compositor written in Safe Rust that renders directly to display framebuffers without legacy X11 server overhead or Wayland client IPC bottlenecks. To deliver user experience supremacy, Zenith incorporates desktop innovations from Linux Mint Cinnamon, Hyprland, System76 COSMIC, SerenityOS LibGUI, and Omarchy Linux.

```
+----------------------------------------------------------------------------------------------------+
|                         SIGMAOS ZENITH DESKTOP & USERLAND ROADMAP                                  |
+----------------------------------------------------------------------------------------------------+
|  [Zenith Bare-Metal GPU Compositor] |  [Hyprland Dynamic Tiling Engine] |  [Mint-Y & Mint-X CSS Theme] |
+----------------------------------------------------------------------------------------------------+
|  [Omarchy Developer Toolchain Engine] |  [SerenityOS LibGUI Protocol]   |  [Accessibility WCAG 2.1 AAA]|
+----------------------------------------------------------------------------------------------------+
|                   SIGMAOS DIRECT FRAMEBUFFER & DRM/KMS Bare-Metal Rendering                        |
+----------------------------------------------------------------------------------------------------+
```

---

## 2. Bare-Metal Zenith Compositor & Window Management

### 2.1 Direct Framebuffer & Hyprland Tiling Extensions
- **Inspiration**: Hyprland wlroots compositor and System76 COSMIC desktop.
- **Target Architecture**:
  - Direct KMS/DRM double-buffered swapchain rendering with sub-1ms vblank synchronization.
  - Dynamic auto-tiling layout algorithm (dwindle, master-stack, interactive floating grid) with smooth sub-pixel animations.
- **Milestones**:
  - **Phase 1**: Direct DRM/KMS hardware surface swapchain allocation in Rust `#![no_std]`.
  - **Phase 2**: Hyprland-inspired dynamic workspace workspace switching with smooth gesture swipes.

---

## 3. Linux Mint Theme Engine Parity

### 3.1 Mint-Y & Mint-X Accent Color Theme Engine
- **Inspiration**: Linux Mint Cinnamon Mint-Y, Mint-X, Mint-L, and Mint-Z themes and `xapp` libraries.
- **Target Architecture**:
  - Fully customizable GTK CSS theme generator supporting 11 accent colors (Aqua, Blue, Brown, Teal, Orange, Pink, Purple, Red, Sand, Yaru, Mint Green).
  - Native dark/light mode toggle with automatic time-of-day solar shifting.

---

## 4. Omarchy Developer Toolchain Integration

### 4.1 Omarchy Dev Tool Presets & Polyglot Runtime Engine
- **Inspiration**: Omarchy Linux developer environment toolsets (`mise`, `lazy.nvim`, `ghostty`, `zellij`, `lazygit`, `helix`, `aider`).
- **Target Architecture**:
  - `MiseToolchainManager`: Zero-dependency polyglot runtime manager (Rust, Node.js, Python, Go, Zig).
  - `NeovimLazyVimConfig`: Pre-configured modal editing environment with LSP bridging to `GhosttyTerminalGrid`.
  - `UniversalOpenSourceToolsMasterGateway`: Safe Rust zero-dependency CLI power tools (`git-delta`, `just`, `du-dust`, `bottom`, `procs`, `tokei`, `hyperfine`, `gping`).

---

## 5. Desktop & Userland Parity Matrix

| Feature | Inspired By | SigmaOS Component | Performance Target | Status |
| :--- | :--- | :--- | :--- | :--- |
| **Zenith Compositor** | Custom / Wayland | `src/desktop/zenith.rs` | Direct DRM/KMS 144Hz | Fully Implemented |
| **Hyprland Tiling** | Hyprland | `src/desktop/tiling.rs` | Sub-1ms Animation | Active Development |
| **Mint-Y Themes** | Linux Mint | `src/customization/mint_themes.rs` | Instant Accent Swap | Fully Implemented |
| **Omarchy Dev Tools** | Omarchy Linux | `src/dev/omarchy_dev_tools.rs` | Zero-Config Setup | Fully Implemented |
| **CLI Power Tools** | Modern Rust CLI | `src/tools/universal_tools.rs` | Native Bare-Metal Exec | Fully Implemented |

---

## 6. Verification Protocol

1. **Visual & Rendering Verification**: Verified using direct framebuffer snapshot captures.
2. **Unit Tests**: Full coverage via `omarchy_dynamic_workspace_suite.rs` and `mint_themes.rs` in `./run_sigma_tests.sh`.
