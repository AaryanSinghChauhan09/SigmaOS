# 🤖 AI Agent Roadmap: Desktop, Userland & Graphical Subsystems

This document outlines future development specifications and guidelines for autonomous AI agents working on the SigmaOS Desktop, Userland, and Graphical Environment Subsystems.

---

## 🎯 Graphical Environment & Userland Parity Matrix

### 1. Wayland Composite & Direct KMS Scanout
- **Inspiration**: Hyprland, Sway, SerenityOS LibGUI, PipeWire.
- **AI Agent Directive**:
  - Implement sub-millisecond direct KMS/DRM scanout pipeline with 3D LUT HDR color transformations.
  - Route SPA audio/video buffer graphs dynamically using PipeWire node port link routing.

### 2. Linux Mint & Cinnamon XApp Suite Parity
- **Inspiration**: Linux Mint Cinnamon Desktop, XApps (`thingy`, `mintupdate`, `mintinstall`, `warpinator`, `hypnotix`).
- **AI Agent Directive**:
  - Maintain `thingy` XApp document library parity with reading progress tracking, ratings, and tags.
  - Integrate `mint-x-icons` theme engine with 12 color variants and symbolic fallback icon lookup.

### 3. Omarchy Dotfiles & Theme Engine
- **Inspiration**: Omarchy Linux `.files` dotfile manager & dynamic theme switcher.
- **AI Agent Directive**:
  - Automate local dotfile drift detection and Zstd backup archiving.
  - Support dynamic Wallust color palette extraction across Wayland outputs for wallpaper changes.

---

## 🛠️ Verification Command Protocol

```bash
# Verify desktop & userland tests
mkdir -p build && rustc --test src/desktop/omarchy_theme.rs --edition=2021 -o build/test_omarchy_theme && ./build/test_omarchy_theme

# Execute complete SigmaOS test suite
./run_sigma_tests.sh
```
