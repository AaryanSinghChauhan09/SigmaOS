# 🖥️🖼️ SIGMAOS OMARCHY GUI APPLICATIONS & WORKSTATION ECOSYSTEM BLUEPRINT
## Comprehensive Architecture, Keyboard Navigation Matrix, and 4-Phase Execution Roadmap for Omarchy GUI Applications for https://github.com/AaryanSinghChauhan09/SigmaOS

---

## EXECUTIVE SUMMARY & CONCEPTUAL VISION

Desktop operating systems rely on a cohesive, fast, and accessible suite of graphical user interface (GUI) applications. Inspired by **Omarchy Linux** (the opinionated Linux workstation distribution designed by DHH & 37signals based on Arch Linux, Hyprland, and Quickshell), **SigmaOS** implements a unified GUI applications ecosystem (`src/desktop/omarchy_apps.rs`).

This ecosystem seamlessly integrates core productivity tools, dead-simple dedicated utilities, cross-platform local sharing, dynamic wallpaper palette theme extraction, and media creation tools into a single keyboard-driven workstation workflow.

---

## PART 1: OMARCHY GUI APPLICATION SUITE & KEYBINDINGS MATRIX

| **Application Name** | **Keybinding Shortcut** | **Primary Category & Purpose** | **Key Features & Integrations** |
|----------------------|-------------------------|--------------------------------|---------------------------------|
| **Files (Nautilus)** | `Super + Shift + F` | Graphical File Manager | Terminal CWD launch (`Super+Shift+Alt+F`), `Ctrl+L` path bar, `Space` quick preview, USB auto-mount, Disks partition manager launcher (`Super+Space`), default associations (`imv`, `mpv`, Document Viewer, Neovim). |
| **Obsidian** | `Super + Shift + O` | Vault Markdown Notes | Extensible Markdown note-taking, Omarchy theme syncing, commercial mobile sync support. |
| **Omawrite** | `Super + Shift + W` | Dead-Simple Writing App | Vault-free, plugin-free distractionless Markdown editor for pure writing. |
| **Pinta** | `Super + Space` (Launcher) | Image Editor | Cropping, resizing, Magic Wand selection, multi-layer image editing. |
| **Aether** | `Super + Space` (Launcher) | Wallpaper Theme Generator | Extracts color palettes from background wallpapers to auto-generate cohesive desktop themes. |
| **LocalSend** | `Super + Ctrl + S` | Cross-Platform Network Transfer | Share menu for Clipboard, Files, Folders, and Receiving over local Wi-Fi/Ethernet; CLI `omarchy share` bridge; Nautilus right-click context integration; default firewall port exemption. |
| **LibreOffice** | `Super + Space` (Launcher) | Full Office Suite | Word processing, spreadsheets, presentations; full Microsoft Office (DOCX/XLSX/PPTX) compatibility. |
| **Omacalc** | `Super + Ctrl + Q` | Floating Window Calculator | Dead-simple arithmetic and floating window quick calculator. |
| **Signal** | `Super + Shift + G` | E2E Encrypted Messaging | Privacy-first messaging; auto-install prompt on first launch if uninstalled. |
| **mpv** | `Super + Space` / Double-click | Media Player | Fast hardware-accelerated video/audio playback for almost any media container. |
| **OBS Studio** | `Super + Space` (Launcher) | Screen Recorder & Streamer | Multi-input video/audio mixing, webcam overlays, screencasting. |
| **Kdenlive** | `Super + Space` (Launcher) | Non-Linear Video Editor | Advanced multi-track video editing for OBS Studio recordings. |
| **Omacut** | `Super + Space` (Launcher) | Fast Video Trimmer | Dead-simple video clip start/end trimming without heavy video editor overhead. |

---

## PART 2: CORE ARCHITECTURAL PILLARS FOR SIGMAOS

```
                 +-------------------------------------------------+
                 |   SIGMAOS OMARCHY GUI APPLICATIONS ENGINE       |
                 +-------------------------------------------------+
                                          |
      +-------------------+---------------+---------------+-------------------+
      |                   |               |               |                   |
      v                   v               v               v                   v
📂 NAUTILUS FILE     🌐 LOCALSEND CROSS-  🎨 AETHER WALLPAPER ✍️ OMAWRITE &    🎥 OMACUT & MPV
  MANAGER ENGINE       NETWORK SHARE     THEME EXTRACTOR   OMACALC SUITE       MEDIA TRIMMERS
  • Auto-Mount USB    • Super+Ctrl+S     • K-Means Palette • Vault-Free MD     • Fast Start/End Trim
  • Space Preview     • Clipboard/Folder • Auto GTK/Qt/WM   • Floating Calculator• Hardware Acceleration
  • App Associations  • Firewall Unblock • Color Palette   • Keybinding Launch • OBS/Kdenlive Bridge
```

---

## PART 3: 4-PHASE DEVELOPMENT ROADMAP

### PHASE 1: Core Application Keybindings & Desktop Launchers
- Implement `OmarchyGuiAppsEngine` in `src/desktop/omarchy_apps.rs` mapping global hotkeys (`Super+Shift+F`, `Super+Shift+O`, `Super+Shift+W`, `Super+Ctrl+S`, `Super+Ctrl+Q`, `Super+Shift+G`).
- Register default application launchers for Pinta, Aether, LibreOffice, mpv, OBS Studio, Kdenlive, and Omacut.

### PHASE 2: File Association & Storage Auto-Mounting Engine
- Implement default file type handlers mapping image extensions (`.png`, `.jpg`) -> `imv`, video extensions (`.mp4`, `.mkv`) -> `mpv`, documents (`.pdf`) -> Document Viewer, and text (`.txt`, `.md`) -> Neovim.
- Integrate automatic storage device mounting for USB drives and SD cards with sidebar notifications.

### PHASE 3: LocalSend Cross-Platform Network File Transfer & Firewall Rules
- Implement `LocalSendShareManager` supporting `Clipboard`, `File`, `Folder`, and `Receive` share modes.
- Expose `omarchy share` CLI commands and Nautilus right-click context menu options. Ensure LocalSend port (53317) is unblocked by default in OpenBSD `pf` / Netfilter firewall rules.

### PHASE 4: Dynamic Wallpaper Palette Extraction (Aether) & Theme Syncing
- Deploy `AetherThemeExtractor` calculating dominant background colors using k-means color quantization.
- Automatically broadcast generated color palettes across Zenith compositor, GTK, Qt, and Obsidian Omarchy themes.

---

## PART 4: VERIFICATION BENCHMARK & TEST CRITERIA

1. **Keybinding Registry Unit Tests**: Verify that invoking application shortcuts launches the correct binary or internal desktop service.
2. **File Association Unit Tests**: Confirm file extensions resolve to appropriate default applications (`.mp4` -> `mpv`, `.md` -> `omawrite` / Neovim).
3. **LocalSend Sharing Unit Tests**: Validate CLI `omarchy share clipboard` string serialization and share mode state machine.
4. **Aether Palette Extraction Unit Tests**: Ensure wallpaper color palette generation produces valid RGB/HEX color tokens.

---
*End of SigmaOS Omarchy GUI Applications & Workstation Ecosystem Blueprint Specification.*
