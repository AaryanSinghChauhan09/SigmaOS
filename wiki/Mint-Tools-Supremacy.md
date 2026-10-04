# Mint Tools Supremacy

The **Mint Tools Supremacy** layer is SigmaOS's native reimplementation and enhancement of Linux Mint's most beloved user-facing tools. Implemented in `src/compatibility/mint_tools_supremacy.rs`, it provides MintStick, Bulky, MintReport, Nemo Actions, and XApps — all rewritten in pure Rust with superior safety, performance, and integration.

---

## Overview

Linux Mint (`github.com/linuxmint`) built a reputation on user-friendly tools layered on top of Ubuntu. SigmaOS takes each Mint tool, reimplements it in Rust, and adds features Mint doesn't have:

| Mint Tool | SigmaOS Equivalent | Advantage |
|-----------|-------------------|-----------|
| MintStick | `SovereignMintStickEngine` | Verify-after-write, compression support |
| Bulky | `SovereignBulkyEngine` | Undo log, regex rename, preview |
| MintReport | `SovereignMintReportEngine` | AI-enhanced, health score 0–100 |
| Nemo Actions | `SovereignNemoActionsEngine` | MIME-typed, batch execution |
| XApp library | `SovereignXAppsManager` | Pure Rust, HiDPI-native |

---

## MintStick (`SovereignMintStickEngine`)

### Purpose
USB drive image writer — safer and faster than Mint's MintStick.

### Features
- Writes ISO, IMG, and compressed (`.gz`, `.xz`, `.zst`) images
- **Verify-after-write**: reads back and SHA256-compares every block
- Block-level progress reporting (bytes/s, ETA)
- Safe device enumeration: only shows removable drives
- Unmounts all partitions before writing

### Usage
```bash
sigma-stick write /path/to/sigmaos.iso /dev/sdb
sigma-stick verify /dev/sdb --checksum sha256:abc123
sigma-stick list                  # show removable drives
```

### Key Types
| Type | Description |
|------|-------------|
| `SovereignMintStickEngine` | Main engine struct |
| `UsbDriveInfo` | Drive metadata (size, model, removable) |
| `WriteProgress` | Live progress (bytes written, speed, ETA) |
| `VerifyResult` | Post-write verification outcome |

---

## Bulky (`SovereignBulkyEngine`)

### Purpose
Batch file renamer with preview and undo — superior to Mint's Bulky.

### Rename Modes

| Mode | Example Input | Example Output |
|------|--------------|----------------|
| `Regex` | `s/IMG_(\d+)/photo_$1/` | `IMG_001.jpg` → `photo_001.jpg` |
| `CaseChange` | `LowerCase` | `DOCUMENT.PDF` → `document.pdf` |
| `Sequence` | `prefix=track_ start=1 pad=3` | `_.mp3` → `track_001.mp3` |
| `MetadataDate` | `{year}-{month}-{day}` | `DSC_0001.jpg` → `2025-10-04.jpg` |

### Undo Log
All renames are logged to `~/.local/share/sigma/bulky/undo.log`:
```
2025-10-04T10:00:00Z  /home/user/IMG_001.jpg → /home/user/photo_001.jpg
```
Undo any rename:
```bash
sigma-bulky undo --last 5
sigma-bulky undo --all
```

### Preview Before Rename
```bash
sigma-bulky preview --mode regex 's/old/new/' ~/Pictures/*.jpg
# Shows: old_name.jpg → new_name.jpg (without applying)
sigma-bulky apply  # confirms and applies
```

---

## MintReport (`SovereignMintReportEngine`)

### Purpose
System health report generator with AI-enhanced analysis.

### Health Score
- Score: **0–100** (higher = healthier)
- Calculated by: `100 - Σ(severity_penalties)`
  - `Critical` finding: -25 points
  - `Warning` finding: -10 points
  - `Info` finding: -2 points
- Minimum score: 10 (system never scores 0 — always improvable)

### Report Categories
| Category | Checks |
|---------|--------|
| Hardware | SMART disk health, RAM errors, temperature |
| Kernel | Oops count, RCU stalls, hung tasks |
| Software | Broken packages, failed services |
| Network | DNS resolution, MTU, packet loss |
| Security | Open ports, world-writable files, CVEs |

### Output Formats
```bash
sigma-report --format text      # terminal-friendly
sigma-report --format html      # browser report
sigma-report --format json      # machine-parseable
sigma-report --format pdf       # printable report
```

### Example Output
```
SigmaOS System Health Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Score: 87/100

[CRITICAL] SMART: /dev/sda reallocated sectors = 42     (-25)
[WARNING]  Service: sigma-netd restarted 3 times today  (-10)
[INFO]     Update: 3 packages have available updates     (-2)
[OK]       Kernel: no oops in last 7 days
[OK]       Network: DNS resolving correctly
```

---

## Nemo Actions (`SovereignNemoActionsEngine`)

### Purpose
Right-click context menu actions for the file manager — extensible and MIME-aware.

### Action Definition
```toml
[[action]]
name = "Convert to WebP"
icon = "image-convert"
mime_types = ["image/jpeg", "image/png"]
command = "sigma-img convert --format webp {files}"
batch = true        # operates on multiple selected files at once
```

### Built-in Actions
| Action | MIME Types | Description |
|--------|-----------|-------------|
| Open Terminal Here | `inode/directory` | Opens shell in selected folder |
| Set as Wallpaper | `image/*` | Sets desktop background |
| Extract Here | `application/zip`, etc. | Extract archive in place |
| Share via Warpinator | Any | P2P file transfer |
| Convert Image | `image/*` | Format conversion |
| Shred File | Any | Secure deletion |

### Batch Execution
```bash
sigma-nemo-action run "Convert to WebP" ~/Pictures/*.jpg
# Converts all selected files with progress bar
```

---

## XApps Manager (`SovereignXAppsManager`)

### Purpose
Cross-desktop application theming and HiDPI management — replaces Mint's `libxapp`.

### Features
- Unified GTK3/GTK4 theme injection
- HiDPI: per-app scale factor override
- Dark/light mode switch propagation to all running apps
- Window decoration override (client-side vs server-side)
- Status icon compatibility for system tray

### Configuration
```toml
[xapps]
global_theme = "sigma-dark"
hiDPI_scale = "auto"           # auto | 1.0 | 1.5 | 2.0
prefer_dark_mode = true
force_csd = false              # false = use compositor decorations
icon_theme = "sigma-icons"
font = "Inter 11"
monospace_font = "JetBrains Mono 10"
```

### App-Specific Overrides
```toml
[[xapps.override]]
app_id = "org.gnome.gedit"
scale = 1.5
theme = "sigma-light"
```

---

## WebApp Manager (`SovereignWebAppManager`)

### Purpose
Isolated desktop web applications from URLs — superior to Linux Mint's `webapp-manager`.

### Features
- Spawns isolated browser profiles with isolated cookies, local storage, and caches
- Suppresses browser navigation bars and tabs for native app look-and-feel
- Generates desktop entries and application drawer shortcuts
- Pre-configured profiles: YouTube Music, Discord, GitHub Enterprise

---

## Keyboard Shortcut Remapper (`SovereignKeyboardShortcutRemapper`)

### Purpose
System-wide hotkey management and hardware modifier remapping.

### Features
- Native CapsLock-to-Control swap for developers
- Custom hotkey actions (e.g. Super+Enter for `sigma-term`, Super+B for `sigma-browser`)
- Gaming mode suppression: automatically suppresses desktop hotkeys during fullscreen games

---

## Comparison: Mint Tools vs SigmaOS Equivalents

| Feature | Linux Mint | **SigmaOS** |
|---------|-----------|-------------|
| Language | Python/C | **Rust** |
| MintStick verify | ❌ | ✅ |
| Bulky undo | ❌ | ✅ |
| Report AI analysis | ❌ | ✅ |
| Nemo batch actions | ✅ limited | ✅ full |
| XApps HiDPI | ✅ | ✅ native |
| WebApp isolation | ✅ | ✅ sandbox profile |
| Keyboard gaming suppression | ❌ | ✅ |
| Report health score | ❌ | ✅ 0–100 |

---

## Source File

- [`src/compatibility/mint_tools_supremacy.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/compatibility/mint_tools_supremacy.rs)

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/compatibility/mint_tools_supremacy.rs`
> - Update action table when new built-in Nemo actions are added
> - Update health score penalty table if scoring algorithm changes
> - Keep comparison table current vs Linux Mint releases on GitHub
> - Cross-reference with [Mint-and-Omarchy-Supremacy](Mint-and-Omarchy-Supremacy.md)
