# Linux Mint and Omarchy Linux Inspired Features

This document describes the features implemented in SigmaOS inspired by Linux Mint and Omarchy Linux distributions.

## Linux Mint-Inspired Features

### Package Management

#### MintUpdateManager
- Update manager with 4-level classification: Security, Recommended, Optional, Unsafe
- Auto-install security and recommended updates
- Update level filtering and configuration
- Total update size calculation

#### MintInstallManager
- Software manager with multi-source support (APT, Flatpak, Snap, SigmaPkg, AUR, AppImage)
- Package search functionality
- Installed and available update tracking
- Source enable/disable management
- Sandbox detection for package sources

#### MintMirrorManager
- Repository mirror management with latency-based selection
- Mirror scoring and auto-selection
- Country-based mirror filtering
- Mirror refresh tracking

### Desktop Environment

#### CinnamonDesktopManager
- Desktop environment manager with panels, desklets, themes, and extensions
- Panel configuration with applets (menu, task switcher, tray, clock, volume, network, battery)
- Panel positioning (top, bottom, left, right)
- Desktop widgets (desklets) with position and size configuration
- Theme management with GTK, icon, window, cursor themes
- Extension system with enable/disable and configuration
- Default layout initialization

#### XAppPreferences
- Cross-desktop integration settings
- Dark mode, accent colors, animations
- Font configuration (default and monospace)
- Locale and time format settings

### System Tools

#### MintDriverManager
- Driver management for proprietary and open-source drivers
- Driver type classification (OpenSource, Proprietary, Community, BuiltIn)
- Driver status tracking (NotInstalled, Installed, Active, Recommended, InUse)
- Auto-install recommended drivers with proprietary restrictions
- Device-driver mapping and recommendations

#### MintUsbWriter
- USB formatting and bootable image creation
- Filesystem support (FAT32, NTFS, exFAT, ext4, Btrfs)
- Image format detection (ISO, IMG, Hybrid)
- Operation progress tracking with speed estimation
- Device safety verification and space checking

#### MintDomainBlocker
- Domain blocking via /etc/hosts
- Block rule types (Exact, Wildcard, Regex)
- Import/export blocked domains
- Hosts file entry generation
- Block statistics and management

#### MintLocaleManager
- System locale and language pack management
- Locale setting types (System, Language, Numeric, Time, Monetary, etc.)
- Locale search and installation
- Language pack management
- Locale configuration generation

#### MintWelcomeScreen
- First-boot welcome and information display
- System information display
- Content sections (Introduction, Release Notes, Getting Started, etc.)
- HTML and Markdown report generation
- First-boot tracking

#### MintSystemReport
- System information collection and reporting
- Information categories (General, CPU, Memory, Storage, Graphics, etc.)
- Text, JSON, and Markdown report generation
- System information collection
- Search functionality

## Omarchy Linux-Inspired Features

### Theme System

#### OmarchyThemeManager
- Visual theme switcher with live previews
- Semantic color system with named colors (Primary, Secondary, Success, Warning, Error, etc.)
- Coordinated theming across components (Desktop, Terminal, Editor, ActivityMonitor, Notifications, TopBar, Launcher, LockScreen)
- Color definition with hex, RGB, and HSL representations
- CSS variable generation for easy integration
- Theme history for undo functionality
- Dark/light theme filtering
- Theme search by name/description
- Default dark and light themes with Catppuccin-inspired colors

### Command Palette System

#### OmarchyCommandPalette
- Filterable, nested command palette defined in JSONC for system control
- Command action types (Shell, Open, Toggle, Navigate, Custom)
- CommandPaletteItem with label, description, category, shortcut, icon
- Search functionality with label/description/category matching
- Category-based command filtering (System, Applications, Settings)
- Keyboard navigation (next/previous selection)
- Command execution with enabled/disabled states
- Default commands for system operations, applications, and settings
- Extensible architecture for custom commands

## Implementation Details

All implementations follow SigmaOS's zero-dependency `#![no_std]` architecture using `alloc::` primitives for kernel-compatible code. The code maintains Linux/BSD cross-distro interoperability and applies security best practices including least-privilege sandboxing, PQC cryptography, and safe Rust patterns.

## Testing

All implementations include comprehensive unit tests:
- Mint package management: 4 tests passed
- Mint desktop management: 5 tests passed
- Mint system tools: 37 tests passed (driver: 5, usb: 6, blocker: 8, locale: 7, welcome: 5, report: 6)
- Omarchy theme system: 6 tests passed
- Omarchy command palette: 6 tests passed

## Files Added

### Package Management
- `src/package/mint_package.rs` - Linux Mint package management
- `src/package/mod.rs` - Module exports

### Desktop Environment
- `src/desktop/mint_desktop.rs` - Cinnamon desktop management
- `src/desktop/omarchy_theme.rs` - Omarchy theme system
- `src/desktop/mod.rs` - Module exports

### System Tools
- `src/tools/mint_driver_manager.rs` - Driver manager
- `src/tools/mint_usb_writer.rs` - USB image writer
- `src/tools/mint_domain_blocker.rs` - Domain blocker
- `src/tools/mint_locale_manager.rs` - Locale manager
- `src/tools/mint_welcome.rs` - Welcome screen
- `src/tools/mint_system_report.rs` - System reporting
- `src/tools/omarchy_command_palette.rs` - Command palette system
- `src/tools/mod.rs` - Module exports

### Compatibility
- `src/compatibility/mint.rs` - Linux Mint compatibility integration
- `src/compatibility/mod.rs` - Module exports

## Future Enhancements

Potential future additions based on Linux Mint and Omarchy Linux:
- Snapshot management (Btrfs-inspired system snapshots)
- AI usage tracking widget (Omarchy-style model usage statistics)
- LAN file sharing (Warpinator-inspired local network sharing)
- Backup and snapshot tools (Timeshift-inspired backup system)
