# SigmaOS: Window Managers & Compositors Roadmap

This document outlines the design and integration of lightweight, dynamic, and tiling window managers within the Zenith UI compositor stack.

## Target Repositories for Absorption

1. **`tiling-window-manager/i3` & `awesomeWM/awesome`**
   - **Goal:** Dynamic keyboard-driven layout tiling.
   - **SigmaOS Integration:** Combine i3's structural tiling and tree model with AwesomeWM's Lua-based customization to allow developers to build dynamic workspace layouts.

2. **`polybar/polybar` & `rofi/rofi`**
   - **Goal:** Status panels and application launchers.
   - **SigmaOS Integration:** Adapt rofi's prompt system to drive the Zenith semantic AI launcher.

3. **`picom/picom`**
   - **Goal:** Transparency, blurring, and window effects.
   - **SigmaOS Integration:** Bring window shadowing, active/inactive opacity decay, and dual-filter KaWase blur algorithms directly into our GPU-accelerated Zenith compositor.

### Last Updated: July 2026
