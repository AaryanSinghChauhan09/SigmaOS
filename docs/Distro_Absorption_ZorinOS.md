# Distro Absorption: Zorin OS

> **Status**: 📋 Planned | **Source Paradigm**: Zorin OS | **Target Shard**: `SigmaOS Adaptive UI Stylesheet Layer`

---

## 1. Executive Summary

Zorin OS is famous for its visual layout switcher, allowing users to make the desktop environment look and feel exactly like Windows, macOS, or standard Gnome/XFCE layouts at the click of a button.

SigmaOS absorbs Zorin OS's **Desktop Adaptive Layout engine** natively into the Zenith compositor, permitting users to switch UI configurations dynamically without needing desktop restarts.

---

## 2. Key Features to Absorb

### 2.1 Native Adaptive Layouts (`sigma-layout`)

Zenith provides a system-level stylesheet API that changes window decorations, panel positions, taskbar widgets, and launch menus on-the-fly.

```bash
$ sigma ui layout set macOS
Σ [ZENITH] Swapping layout stylesheet...
  Taskbar moved to top
  Dock enabled at bottom
  Window control buttons moved to top-left
```

Since the Zenith compositor is built entirely on unified Rust/Javascript rendering pipelines, swapping the style schema is instantaneous.

---

## 3. References & Standards

- Zorin OS — `zorin.com`
- Zenith Compositor CSS theming rules
