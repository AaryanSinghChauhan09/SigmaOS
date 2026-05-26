# 🖥️ Zenith GUI — Sovereign Desktop Environment

> **"No X11. No Wayland. No GTK. Pure sovereign pixels rendered directly to silicon."**

Zenith is SigmaOS's built-from-scratch desktop environment — our answer to elementary OS and Zorin OS. It combines glassmorphic modern aesthetics with a zero-dependency architecture drawn directly to the VGA/GOP framebuffer.

---

## 🆚 Comparison with Linux Desktop Environments

| Feature | GNOME / KDE | elementary OS | Zenith (SigmaOS) |
|:--|:--|:--|:--|
| Display server | X11 / Wayland | X11 / Wayland | **Direct LFB / GOP** |
| Toolkit | GTK / Qt | Pantheon/GTK | **Sovereign compositor** |
| Memory footprint | ~500MB+ | ~400MB | **<10MB (kernel-integrated)** |
| Attack surface | X11 protocol CVEs | GTK library CVEs | **Zero — no protocol stack** |
| Compositing | Mutter/KWin | Mutter | **`sigma_compositor` painter's algo** |

---

## 1. Architecture Stack

```
Zenith Desktop (zenith.html — dev preview layer)
        │
        ▼
sigma_wm (Window Manager — Z-index, focus, drag)
        │
        ▼
sigma_compositor (Framebuffer painter)
        │
        ▼
GOP / VGA Linear Framebuffer (direct hardware write)
```

---

## 2. The Compositor (`sigma_compositor`)

The compositor writes pixels directly to the Linear Framebuffer (LFB):
- `sigma_compositor_draw_rect()` — Fills rectangles with ARGB color
- `sigma_compositor_blit_text()` — Renders PSF/bitmap fonts
- `sigma_compositor_flip()` — Double-buffer swap (eliminates tearing)
- `sigma_compositor_blur()` — Boxcar blur passes for glassmorphism (planned)

---

## 3. The Window Manager (`sigma_wm`)

Tracks all open windows as a Z-ordered stack:
- Each window has: `x, y, width, height, z_index, title, flags`
- Supports: drag-to-move, minimize, close, focus-on-click
- Window decorations rendered by the compositor (no client-side decoration)

---

## 4. Desktop Shell Components

### 🚀 The Dock (macOS-inspired)
- Glassmorphic floating dock at screen bottom
- Scale-on-hover animation with cubic-bezier easing
- Icons: OmniShell, VFS Browser, Settings, App Store, Emergency Lattice Sync
- Clicking the rocket 🚀 opens the **App Launcher**

### 🗂️ App Launcher (Zorin-inspired)
- Full-screen or panel overlay with categorized apps
- Fuzzy search across installed `.spk` packages
- Categories: Development, Security & Forensics, AI & Cloud, System

### 📊 Top Bar (Status Island)
- Live telemetry: CPU%, RAM, Frame timing
- Silicon Attestation status (Kyber-1024 / Dilithium-5)
- Real-time clock with date

---

## 5. Theme Engine

Zenith supports dynamic theming without restarts via CSS custom properties:

| Theme | Accent | Mood |
|:--|:--|:--|
| **Cyan** (default) | `hsl(185, 100%, 50%)` | Sovereign Lattice |
| **Gold** | `hsl(42, 100%, 55%)` | Executive Edition |
| **Solar** | `hsl(28, 90%, 55%)` | HPC Performance |
| **Crimson** | `hsl(0, 80%, 55%)` | Security / Forensics |

---

## 6. Accessibility Engine

- **High Contrast Mode** — flips to WCAG AAA contrast ratios
- **Screen Reader / TTS** — eSpeak-NG integration via Web Speech API
- **Keyboard Navigation** — full keyboard-only support for all UI elements

---

## 7. Roadmap

| Phase | Milestone |
|:--|:--|
| **Now** | Framebuffer compositor + WM + Dock + App Launcher (✅ Done) |
| **Q2** | GPU-accelerated compositing (SovereignVulkan SPIR-V) |
| **Q3** | PS/2 + USB HID mouse cursor rendering |
| **Q4** | Wayland compatibility mode for porting desktop Linux apps |
