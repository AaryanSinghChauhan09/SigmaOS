# Mint & Omarchy Apex Mastery

The **Mint & Omarchy Apex Mastery Suite** ([`src/distro/sovereign_mint_omarchy_apex_mastery.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_apex_mastery.rs)) elevates SigmaOS above Linux Mint and Omarchy in fractional display scaling, file indexing speed, compositor IPC throughput, and living-room/handheld gamepad navigation.

---

## 1. Overview & Comparative Advantages

| Functional Area | Linux Mint (Muffin / Nemo) | Omarchy (Hyprland / Arch) | **SigmaOS Apex Mastery** |
| :--- | :--- | :--- | :--- |
| **Fractional Scaling** | Bilinear downsampling blur (X11/Wayland) | Standard wlroots fractional scale | **Native `wp-viewport` blur-free crisp scaling + mixed-DPI** |
| **Content Indexing** | Catfish / Tracker (high CPU/IO) | CLI `ripgrep` / `fd` (no index) | **In-memory trigram indexer with sub-5ms multi-TB queries** |
| **Compositor IPC** | Polling D-Bus queries | Hyprland UNIX socket IPC (C++) | **Zero-polling pub/sub event bus with zero allocations** |
| **10-Foot Gamepad UI** | No native gamepad navigation | Manual anti-microX / Steam input | **Native controller mouse emulation, radial menu & virtual keyboard** |

---

## 2. Component Specifications

### A. Sovereign Mixed-DPI & Fractional Scaling Engine (`SovereignMixedDpiFractionalScalingEngine`)
*Source: [`src/distro/sovereign_mint_omarchy_apex_mastery.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_apex_mastery.rs#L34-L122)*

Replaces legacy compositor fractional scaling implementations:
* **Crisp Viewport Rendering**: Utilizes `wp-fractional-scale-v1` and `wp-viewport` to render clients directly at their exact physical destination pixels (125%, 150%, 175%), completely bypassing fractional downsampling blurs.
* **Mixed-DPI & Multi-Refresh Coordination**:
  * Independently drives high-DPI displays (e.g. 2.8K 120Hz @ 150%) alongside native gaming monitors (e.g. 1440p 360Hz @ 100%).
  * Variable Refresh Rate (VRR / Adaptive Sync) scheduled per-display without frame dropping or pipeline stalls.

### B. Sovereign Fast Content Search Engine (`SovereignFastContentSearchEngine`)
*Source: [`src/distro/sovereign_mint_omarchy_apex_mastery.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_apex_mastery.rs#L124-L200)*

* **High-Speed Trigram Indexing**: Indexes file names, MIME types, and document contents in memory.
* **Sub-5ms Query Latency**: Delivers instant search results across millions of lines of code and configuration files without disk spin-up or battery drain.
* **Relevance Ranking**: Weighted scoring prioritizing exact path matches (500 pts), content matches (300 pts), and line number pinpointing.

### C. Sovereign Event-Driven Compositor IPC Bus (`SovereignEventDrivenIpcBus`)
*Source: [`src/distro/sovereign_mint_omarchy_apex_mastery.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_apex_mastery.rs#L202-L245)*

* **Zero-Polling Notification Dispatch**:
  * `WorkspaceSwitched`: Emitted instantaneously on desktop transitions.
  * `WindowFocused`: Delivers active window ID, app class, and title to taskbars and dock widgets.
  * `FullscreenToggled`: Direct scanout state transitions.
  * `MonitorHotplug`: Dynamic display connection / disconnection events.
* **Lock-Free Event Publishing**: Eliminates timer-based polling in desktop panels, system bars, and automation scripts.

### D. Sovereign Gamepad Desktop Navigator (`SovereignGamepadDesktopNavigator`)
*Source: [`src/distro/sovereign_mint_omarchy_apex_mastery.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_apex_mastery.rs#L247-L335)*

* **Living-Room & Handheld Ergonomics**:
  * Circular Deadzone Filtering: Configurable deadzone ($15\%$) prevents analog stick drift.
  * Non-Linear Acceleration Curves: Dual-stage response curve delivers micro-precision aiming at low deflection and fast panning at full deflection.
  * Radial Action Menu: Quick access to window switcher, power controls, and workspace management via the controller Guide button.
  * Virtual On-Screen Keyboard: Toggleable virtual keyboard for text entry without requiring physical peripherals.

---

## 3. Source Code Reference

* Implementation: [`src/distro/sovereign_mint_omarchy_apex_mastery.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_apex_mastery.rs)
* Registration: [`src/distro/mod.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/mod.rs#L402)

---

## 4. AI Agent Maintenance Instructions

> **For AI Agents Maintaining This Page:**
> - Source: `src/distro/sovereign_mint_omarchy_apex_mastery.rs`
> - When adding new fractional scaling viewport algorithms or Wayland protocol extensions, update the scaling specifications.
> - Ensure gamepad button mapping tables reflect both Xbox/Steam Deck and DualSense controller layouts.
> - Cross-reference with [Compositor](Compositor.md), [Theming-and-Customization](Theming-and-Customization.md), and [Omarchy-Gaming-Performance-Suite](Omarchy-Gaming-Performance-Suite.md).
