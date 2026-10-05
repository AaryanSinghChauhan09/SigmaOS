# Desktop Environment Manager

## Overview

The Desktop Environment Manager provides comprehensive DE management inspired by Linux Mint's desktop environment settings and Omarchy's DE utilities. It supports multiple desktop environments with different session types (Wayland, X11, TTY) and installation tracking.

## Features

- **Desktop Environments**: Zenith (SigmaOS native), GNOME, KDE Plasma, XFCE, MATE, Cinnamon, LXQt, Pantheon, TTY
- **Session Types**: Wayland, X11, TTY
- **DE Configuration**: Command, display server, availability tracking
- **Default DE**: Set and track default desktop environment
- **Installation Tracking**: Mark DEs as installed/uninstalled
- **DE Filtering**: List DEs by type or session type
- **Availability**: Track which DEs are available
- **Default Configurations**: Pre-configured Zenith, GNOME, KDE, XFCE, Cinnamon
- **Statistics**: Track total, installed, available, and session type counts

## Components

### DesktopEnvironment

```rust
pub enum DesktopEnvironment {
    Zenith,     // SigmaOS native Wayland compositor
    GNOME,      // GNOME
    KDE,        // KDE Plasma
    XFCE,       // XFCE
    MATE,       // MATE
    Cinnamon,   // Cinnamon (Linux Mint)
    LXQt,       // LXQt
    Pantheon,   // Pantheon (elementary OS)
    TTY,        // Text terminal
}
```

### DesktopDESessionType

```rust
pub enum DesktopDESessionType {
    Wayland,  // Wayland compositor
    X11,      // X11 display server
    TTY,      // Text terminal
}
```

### DEConfig

Desktop environment configuration with:
- DE type and session type
- Command to start
- Display server
- Availability status
- Default flag

### DEManager

Main management interface with:
- DE configuration registration
- Default DE management
- Installation tracking
- Availability management
- DE filtering by type and session
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DEManager;

let mut manager = DEManager::new();

// Get default DE
if let Some(de) = manager.get_default_de() {
    println!("Default DE: {}", de.de_type.as_str());
    println!("Session: {}", de.session_type.as_str());
}
```

### DE Configuration

```rust
// Add a new DE configuration
let config = DEConfig::new(
    DesktopEnvironment::MATE,
    DesktopDESessionType::X11,
    "mate-session".to_string(),
    "Xorg".to_string(),
);
manager.add_config("mate-x11".to_string(), config);

// Get configuration by ID
if let Some(config) = manager.get_config("mate-x11") {
    println!("DE: {}", config.de_type.as_str());
    println!("Command: {}", config.command);
}

// Remove configuration
manager.remove_config("mate-x11");
```

### Default DE

```rust
// Set default DE
manager.set_default_de("gnome-wayland");

// Get default DE
if let Some(de) = manager.get_default_de() {
    println!("Default: {}", de.de_type.as_str());
}
```

### Installation Tracking

```rust
// Mark DE as installed
manager.mark_installed("gnome-x11");

// Mark DE as uninstalled
manager.mark_uninstalled("gnome-x11");

// Get installed DEs
let installed = manager.get_installed_des();
println!("Installed DEs: {}", installed.len());
```

### Availability

```rust
// Set DE availability
manager.set_de_available("gnome-wayland", false);

// Get available DEs
let available = manager.get_available_des();
println!("Available DEs: {}", available.len());
```

### DE Filtering

```rust
// Get DEs by type
let gnome_configs = manager.get_configs_by_de(DesktopEnvironment::GNOME);
println!("GNOME configs: {}", gnome_configs.len());

// Get DEs by session type
let wayland_configs = manager.get_configs_by_session_type(DesktopDESessionType::Wayland);
println!("Wayland configs: {}", wayland_configs.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total configs: {}", stats.total_configs);
println!("Installed: {}", stats.installed_count);
println!("Available: {}", stats.available_count);
println!("Wayland: {}", stats.wayland_count);
println!("X11: {}", stats.x11_count);
```

## Default Configuration

The DE Manager includes default configurations:

- **Zenith Wayland**: Default DE (SigmaOS native)
- **GNOME Wayland**: Installed
- **GNOME X11**: Available
- **KDE Plasma Wayland**: Installed
- **XFCE X11**: Installed
- **Cinnamon X11**: Installed

Default DE: Zenith Wayland

## AI Agent Maintenance Instructions

When maintaining the DE Manager:

1. **DE Validation**: Ensure DE types and session types are valid
2. **Default Protection**: Prevent removal of default DE without replacement
3. **Command Validation**: Ensure commands exist before marking available
4. **Installation Detection**: Integrate with package manager for installation detection
5. **Session Detection**: Detect available session types per DE
6. **Display Server**: Integrate with actual display server detection
7. **Backend Integration**: Integrate with actual login manager (GDM, SDDM, LightDM)

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::de_manager
```

## Future Enhancements

- Integration with actual login manager (GDM, SDDM, LightDM)
- Automatic DE detection from package manager
- Per-user DE preferences
- DE switching without logout
- DE-specific configuration management
- Session configuration per DE
- Display manager integration
- Auto-login configuration
- DE installation/uninstallation
- DE theme synchronization
