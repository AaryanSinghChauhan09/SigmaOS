# Ready to Work & Ready to Launch: Turnkey Operating System Design

SigmaOS is engineered from bare metal to be **immediately ready to work and ready to launch** without tedious post-install configuration, missing driver hunting, or fragmented package installation.

---

## ⚡ Turnkey Out-of-the-Box Experience

### 1. Zero-Configuration Hardware Autodiscovery
- **Automated GPU Acceleration**: Out-of-the-box hardware acceleration for Intel Iris/Xe, AMD Radeon RDNA, NVIDIA, and VirtIO GPUs.
- **Wi-Fi & Bluetooth Auto-Link**: Instant Wi-Fi scanning and peripheral pairing from the initial boot wizard.
- **High-DPI Display Scaling**: Automatic display DPI detection (100%, 150%, 200%) preventing blurry or microscopic UI.

### 2. Pre-Configured Productive Workspaces
SigmaOS ships with four turnkey workspace presets switchable via `Super + 1..4`:
- **💻 Developer Workspace**: Zenith Terminal, Neovim IDE with language servers (Rust, Zig, C, Python), and Git status bar.
- **🎨 Creative Studio**: The complete 20-module Craft Suite with dual-monitor canvas and color palette docks.
- **📊 Business & Office**: WordCraft, GridCraft, DeckCraft, and PDFCraft ready for immediate document authoring.
- **🎮 Entertainment & Media**: Hardware-accelerated media player, gaming HUD, and low-latency audio stack.

### 3. Instant App Dispatcher & Quick Launcher
- **`Super + Space` Launch Palette**: Instant fuzzy search across applications, system settings, open windows, and calculator formulas.
- **Global Keybindings**:
  - `Super + Enter`: Open Zenith Terminal
  - `Super + E`: File Manager
  - `Super + D`: VoiceCraft Speech Dictation
  - `Super + Shift + S`: Screenshot Region Tool
  - `Super + L`: Secure Lock Screen

### 4. Turnkey Installation & Live ISO
- **Instant Live Boot**: Boots to an interactive desktop in under 3 seconds from a live USB drive.
- **One-Click Calamares/Sigma Installer**: Automated disk partitioning, dual-boot detection, Btrfs/SigmaFS formatting, and user account creation.
- **Reproducible Dotfiles Sync**: Declarative configuration imports matching Omarchy and Arch Linux workflows with one command.

```bash
# Verify entire system readiness
sigmaos-doctor --ready-to-work

# Instant workspace switch
sigmaos-workspace switch creative
```
