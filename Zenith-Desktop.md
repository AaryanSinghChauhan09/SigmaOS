# Zenith Desktop Environment

The Zenith Desktop is SigmaOS's flagship UI — a sovereign, compositor-first desktop environment with AI-driven personalisation, spatial audio, and full accessibility.

---

## Architecture

```
Zenith Desktop (zenith_desktop/)
    │
    ├── Compositor (C++ native — Phase G)
    │     └── ZenithDesktopEnvironment.cpp
    ├── Window Manager
    │     └── zenith_desktop/wm/
    ├── Neural UI (AVX-512 accelerated)
    │     └── zenith_desktop/neural/sigma_neural_ui.cpp
    ├── Theme Engine
    │     └── zenith_desktop/theme/
    ├── Personalisation
    │     └── zenith_desktop/personalization/
    ├── App Store
    │     └── zenith_desktop/appstore/
    ├── Notifications
    │     └── zenith_desktop/notifications/
    ├── Settings
    │     └── zenith_desktop/settings/
    └── Accessibility
          └── zenith_desktop/a11y/
```

---

## Current State

| Component | Status |
|-----------|--------|
| JS prototype (browser) | ✅ Working in browser profile |
| C++ compositor | 🔄 In progress — Phase G |
| Auto-tiling WM | 🔄 Implemented, needs input integration |
| Theme engine | ✅ Implemented |
| Neural UI (AVX-512) | ✅ Implemented |
| Accessibility (SSR) | 🔄 Partial |
| Indian IME | ⬜ Phase H |
| sigma-ai LLM daemon | ⬜ Phase H |
| App store UI | 🔄 Demo in `release/app` |

---

## Window Management (`zenith_desktop/wm/`)

Zenith uses a **hybrid tiling + floating** window manager:

```
Workspaces: 1–9 (switch with Super+1 through Super+9)
Tiling modes:
  - Master/stack (default)
  - Grid (Super+G)
  - Fullscreen (Super+F)
  - Floating (Super+Shift+Space)

Key bindings (default):
  Super+Enter     New terminal
  Super+D         App launcher (sigma-spotlight)
  Super+Q         Close window
  Super+H/L       Resize master pane
  Super+Shift+R   Reload config
```

---

## Theme System (`zenith_desktop/theme/`)

```bash
# List available themes
sigma-theme list

# Apply a theme
sigma-theme apply midnight-sovereign

# Create a custom theme
sigma-theme new my-theme --base midnight-sovereign
```

**Built-in themes:**
- `midnight-sovereign` — dark blue/purple
- `solar-zenith` — warm amber + white
- `forest-minimal` — muted green
- `arctic-pure` — clean white/grey

Theme structure: CSS-like variables → compositor applies to all windows.

---

## Neural UI (`zenith_desktop/neural/sigma_neural_ui.cpp`)

AI-driven adaptive UI using AVX-512 SIMD acceleration:

- **Predictive pre-loading**: predicts next app the user will open based on context
- **Adaptive layouts**: rearranges widgets based on usage patterns
- **Smart notifications**: batches low-priority notifications, surfaces critical ones immediately
- **Energy-aware rendering**: reduces refresh rate when battery low

---

## Accessibility (`zenith_desktop/a11y/`)

- **Sovereign Screen Reader (SSR)**: reads UI elements aloud via sigma-voice
- **High-contrast themes**: automatic inversion for visual impairments
- **Keyboard-only navigation**: full WCAG 2.1 AA compliance target
- **Switch access**: single-switch scanning for motor impairments
- **Font scaling**: 75%–300% without layout break
- **Braille display**: planned via sigma-braille daemon (Phase H)

---

## Zenith Widgets (`zenith_desktop/modules/`)

Pre-installed widgets on the desktop panel:

| Widget | File | Function |
|--------|------|---------|
| System telemetry | `sigma_widget_sys_telemetry.cpp` | CPU/RAM/net graphs |
| AI monitor | `sigma_widget_ai_monitor.cpp` | LLM inference usage |
| Crypto shield | `sigma_widget_crypto_shield.cpp` | Active PQC connections |
| App launcher | `sigma_widget_app_launcher.cpp` | Quick-launch dock |
| Quick settings | `sigma_widget_quick_settings.cpp` | Wi-Fi, volume, brightness |

---

## Desktop Build & Run

```bash
# Build Zenith desktop
make PROFILE=standalone all -j$(nproc)

# Run JS prototype in browser (current demo)
open sigma-web/index.html

# Run native compositor in QEMU (Phase G)
make PROFILE=standalone qemu
```

---

*See also: [Architecture-Overview](Architecture-Overview) · [Release-Profiles](Release-Profiles) · [Sigma-Desktop-Environment](Sigma-Desktop-Environment)*
