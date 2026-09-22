# 🌐 SigmaOS Native Web Application Container Subsystem (`web_apps`) Strategic Development Plan

## Executive Summary & Design Vision

Modern desktop productivity relies heavily on cloud-native web applications (e.g., **HEY Email/Calendar**, **Basecamp**, **ChatGPT**, **Grok**, **WhatsApp**, **Google Apps**, **X**, **YouTube**, **Zoom**, **Discord**). However, running these applications in standard browser tabs clutters browser window state, mixes application audio streams, and lacks native desktop window management integration.

Drawing direct architectural inspiration from **Omarchy Web Apps**, Chromium/Chromium-based SSB (Site-Specific Browser) flags (`--app=https://...`), GNOME Web / Epiphany Web Apps, and Wayland frameless window rules, the **SigmaOS Web Application Subsystem** (`SigmaWebAppManager`, `OmarchyWebAppLauncher`, `ZenithFramelessWindowRule`) provides an isolated, frameless, hotkey-bindable web app container framework built natively in Safe Rust.

---

## 1. Web Application Management Architecture

```text
┌───────────────────────────────────────────────────────────────────────────┐
│              Omarchy Menu & Application Launcher (`Super + Space`)         │
│               - Install Web App (`app_name`, `app_url`, `icon_url`)      │
│               - Remove Web App (`delete_app`)                            │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                 `SigmaWebAppManager` Configuration Engine                 │
│     - Manifest Storage: `~/.config/sigma/web_apps/<app_id>.json`          │
│     - Icon Fetcher: Favicon auto-retrieval or DashboardIcons PNG fetcher   │
│     - Desktop Entry Generator: `~/.local/share/applications/sigma-<app>.desktop`│
│     - Hotkey Mapping Generator: `~/.config/hypr/bindings.lua`             │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│               Zenith Frameless Window & Keyboard Shortcut Router          │
│     - Frameless Window Decoration: Titlebar hidden, custom border radius   │
│     - URL Copy Hotkey (`Shift + Alt + L` -> Copy active URL to clipboard)  │
│     - Pre-logged-in Cookie Isolation & Password Manager Compatibility      │
└───────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Pre-Configured Default Web Applications & Global Shortcuts

SigmaOS ships out-of-the-box with an opinionated assortment of pre-configured web applications and global desktop keybindings:

| Web Application | Service Description | Direct Launcher Keybinding | Deep-Link / Composer Hotkey |
| :--- | :--- | :--- | :--- |
| **HEY Email** | Privacy-focused email service by 37signals | `Super + Shift + E` | `Super + Shift + Alt + E` (Compose Email) |
| **HEY Calendar** | Circadian calendar service by 37signals | `Super + Shift + C` | — |
| **Basecamp** | Integrated project management by 37signals | Launcher (`Super + Space`) | — |
| **ChatGPT** | OpenAI AI conversational assistant | `Super + Shift + A` | — |
| **Grok** | xAI conversational assistant | `Super + Shift + Alt + A` | — |
| **WhatsApp** | Encrypted instant messaging web client | `Super + Shift + Alt + G` | — |
| **Google Messages** | SMS / RCS web messaging client | `Super + Shift + Ctrl + G` | — |
| **Google Photos** | Cloud photo gallery & media backup | `Super + Shift + P` | — |
| **Google Maps** | Navigation & geographic search | `Super + Shift + S` | — |
| **Google Contacts** | Contact directory & address book | Launcher (`Super + Space`) | — |
| **X (Twitter)** | Breaking news & social feed | `Super + Shift + X` | `Super + Shift + Alt + X` (New Post) |
| **YouTube** | Video streaming platform | `Super + Shift + Y` | — |
| **Zoom** | Video conferencing web client wrapper | Launcher (`Super + Space`) | Auto-opens `zoom.us/j/*` links |
| **Discord** | Community chat & voice communication | Launcher (`Super + Space`) | — |

---

## 3. Core Architectural Subsystems

### 3.1 Web App Installation & Removal Wizard (`SigmaWebAppManager`)
- **Installation Workflow**: Triggered via *Install > Web App* in the Omarchy menu (`Super + Space`). Prompts the user for:
  1. `app_name`: Human-readable app display title (e.g., `Basecamp`).
  2. `app_url`: Validated HTTPS target URL (e.g., `https://3.basecamp.com`).
  3. `icon_url` (optional): PNG icon URL (fallback to Favicon scraping or [Dashboard Icons](https://dashboardicons.com)).
- **Removal Workflow**: Triggered via *Remove > Web App* in Omarchy menu, removing `.desktop` launchers, config JSON files, and keybinding registrations.

### 3.2 Frameless Window Isolation & URL Copying (`Shift + Alt + L`)
- **Frameless Window Rules**: Zenith Wayland compositor applies `windowrulev2 = float, class:^(sigma-webapp-.*)$` and removes window titlebars for a clean, borderless native application aesthetic.
- **Active URL Clipboard Copy**: Pressing `Shift + Alt + L` inside any active web application window extracts the current URL and writes it directly to the system clipboard via Wayland portal (`wl-clipboard` / OSC 52).

---

## 4. Phased Development Roadmap

### Phase 1: Native Web App Manifest Engine & Installer Wizard (Q4 2026)
- Build `SigmaWebAppManager` JSON manifest storage and desktop entry generator.
- Implement Favicon auto-fetcher with DashboardIcons fallback support.
- Create CLI installer builtin (`sigma-webapp install/remove/list`).

### Phase 2: Zenith Frameless Window Rules & Keyboard Hotkeys (Q1 2027)
- Configure Zenith Wayland compositor frameless window rules for web app instances.
- Implement global Lua keybinding mappings (`bindings.lua`) for HEY, ChatGPT, Grok, WhatsApp, X, YouTube, and Google Apps.
- Add `Shift + Alt + L` active URL clipboard copying shortcut.

### Phase 3: Zoom Deep-Link Routing & Pre-Configured Shortcuts (Q2 2027)
- Deploy automatic URL scheme handler mapping `zoom.us/j/*` meeting links directly to the Zoom Web App container.
- Embed 14 pre-configured default web application desktop entries in `/usr/share/applications/`.
- Optimize cookie store sharing with default web browser for seamless 1Password/SSO login persistence.

### Phase 4: Performance Benchmarks & CI Testing (Q3 2027+)
- Measure Web App window launch latency (target < 200ms).
- Integrate Web App manifest tests into `./scripts/verify.sh` and continuous integration runner.

---

## 5. Verification & Testing Standards

All Web App subsystem components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/desktop/` (Zenith frameless window rules & web app keybindings)
- `src/shell/` (Omarchy web app launcher & hotkey router)
- `src/security/input_validation.rs` (web app URL & icon path sanitization)
