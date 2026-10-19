# SigmaOS Miscellaneous Tools Components

## Overview
SigmaOS miscellaneous tools manager providing drive management and TUI application management, competitive with Omarchy miscellaneous tools (omancy-drive-*, omancy-tui-*, omancy-file-select, omancy-font-*, omancy-games-*, omancy-git-*, omancy-hook, omancy-hw-*, omancy-monitor-*, omancy-refresh-*, omancy-transcode-*, omancy-voxtype-*, omancy-windows-vm).

## Features
- **Drive management** - Add, list, get drive information
- **Drive usage** - Calculate usage percentage
- **TUI app management** - Install and remove TUI applications
- **App listing** - List installed TUI apps

## Structures

| Structure | Description |
|-----------|-------------|
| **MiscToolsManager** | Miscellaneous tools manager |
| **DriveManager** | Drive manager with drive registry |
| **DriveInfo** | Individual drive information |
| **TuiManager** | TUI application manager |

## Usage Examples

```rust
// Create misc tools manager
let mut mgr = MiscToolsManager::new();

// Add a drive
mgr.drive_manager.add_drive(DriveInfo {
    id: "root".to_string(),
    name: "Root".to_string(),
    device: "/dev/sda1".to_string(),
    total_bytes: 500_000_000_000,
    used_bytes: 250_000_000_000,
    available_bytes: 250_000_000_000,
    mount_point: "/".to_string(),
    filesystem: "ext4".to_string(),
});

// Get usage percent
let pct = mgr.drive_manager.get_usage_percent("root").unwrap();

// Install a TUI app
mgr.tui_manager.install("btop".to_string(), "/usr/bin/btop".to_string());

// List TUI apps
let apps = mgr.tui_manager.list_apps();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: System Administration (P1)

## Related Files
- `src/misc/manager.rs` - Misc tools implementation
- `wiki/Misc-Tools.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL, USER_CONTROL, CODE_REUSABILITY,
COMPOSITION_OVER_INHERITANCE, PLUG_IN_DRIVER_MODEL