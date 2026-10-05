# Boot Manager

## Overview

The Boot Manager provides system startup and boot management inspired by Linux Mint's startup applications and Omarchy's boot utilities. It manages boot services, startup programs, and startup sequence simulation.

## Features

- **Startup Types**: Auto, Manual, Disabled
- **Startup Entries**: Configurable startup service definitions
- **Boot Time Tracking**: System boot time recording
- **Startup Delay**: Configurable delay for each entry
- **Enable/Disable**: Enable or disable startup entries
- **Startup Simulation**: Simulate startup sequence
- **Default Services**: Pre-configured essential system services
- **Statistics**: Track startup entry counts by type

## Components

### StartupType

```rust
pub enum StartupType {
    Auto,      // Start automatically on boot
    Manual,    // Start manually only
    Disabled,  // Disabled, will not start
}
```

### StartupEntry

Represents a startup entry with:
- Name and command
- Description
- Startup type
- Delay in seconds
- Enabled status

### BootManager

Main management interface with:
- Default system service registration
- Startup entry management
- Enable/disable operations
- Startup type configuration
- Boot time tracking
- Startup sequence simulation
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::system::BootManager;

let mut manager = BootManager::new();

// List all startup entries
let entries = manager.list_entries();

// Get boot time
let boot_time = manager.get_boot_time();
```

### Managing Startup Entries

```rust
// Add a custom startup entry
let entry = StartupEntry::new(
    "Custom Service".to_string(),
    "/usr/bin/custom-service".to_string(),
    "Custom background service".to_string(),
);
manager.add_entry(entry);

// Remove a startup entry
manager.remove_entry("Custom Service")?;
```

### Enable/Disable Entries

```rust
// Disable a startup entry
manager.disable_entry("Network Manager")?;

// Enable a startup entry
manager.enable_entry("Network Manager")?;
```

### Startup Type Configuration

```rust
// Set startup type to manual
manager.set_startup_type("Network Manager", StartupType::Manual)?;

// Set startup type to auto
manager.set_startup_type("Network Manager", StartupType::Auto)?;

// Disable startup
manager.set_startup_type("Network Manager", StartupType::Disabled)?;
```

### Startup Simulation

```rust
// Simulate startup sequence
let sequence = manager.simulate_startup();
for line in sequence {
    println!("{}", line);
}
```

### Filtering Entries

```rust
// List auto-start entries
let auto_entries = manager.list_by_type(StartupType::Auto);

// List manual entries
let manual_entries = manager.list_by_type(StartupType::Manual);

// List disabled entries
let disabled = manager.list_by_type(StartupType::Disabled);

// List enabled entries
let enabled = manager.list_enabled();

// List entries that should start
let should_start = manager.list_should_start();
```

### Boot Time Management

```rust
// Set boot time
manager.set_boot_time(1234567890);

// Get boot time
let boot_time = manager.get_boot_time();
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total entries: {}", stats.total_entries);
println!("Enabled: {}", stats.enabled_entries);
println!("Auto-start: {}", stats.auto_start);
println!("Manual: {}", stats.manual_start);
println!("Disabled: {}", stats.disabled);
```

## Default Startup Services

The Boot Manager includes these default startup services:

1. **Network Manager** - Network connection management
2. **Power Manager** - Power management daemon
3. **Display Manager** - Display and login manager
4. **Bluetooth** - Bluetooth daemon
5. **Audio** - Audio system

## AI Agent Maintenance Instructions

When maintaining the Boot Manager:

1. **Essential Services**: Ensure essential system services remain enabled by default
2. **Startup Order**: Maintain proper startup order based on dependencies
3. **Delay Handling**: Ensure startup delays are properly applied
4. **Entry Validation**: Validate startup commands before adding
5. **Boot Time Accuracy**: Maintain accurate boot time tracking
6. **Simulation Accuracy**: Ensure startup simulation reflects actual behavior

## Testing

Run the unit tests with:

```bash
cargo test --lib system::boot_manager
```

## Future Enhancements

- Integration with actual init system (systemd, runit, s6)
- Dependency management between services
- Service health monitoring
- Failed service restart
- Boot time optimization
- Startup performance profiling
- Service logs integration
