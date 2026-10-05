# Update Manager

## Overview

The Update Manager provides comprehensive system update management inspired by Linux Mint's mintupdate. It supports kernel updates, package updates, security patches, and version management with safety-level classification.

## Features

- **Update Classification**: Updates are classified by safety level (Safety, Recommended, Feature, Unstable)
- **Category-Based Filtering**: Kernel, Security, Bugfix, Feature, and Dependency updates
- **Auto-Installation Configurable**: Automatic installation of safety and security updates
- **Blacklist Support**: Exclude specific packages from updates
- **Security Update Detection**: Automatic identification of security patches
- **Update Statistics**: Comprehensive tracking of available and installed updates

## Components

### UpdateLevel

```rust
pub enum UpdateLevel {
    Safety = 1,       // Critical safety updates
    Recommended = 2, // Recommended updates
    Feature = 3,      // New features
    Unstable = 4,    // Unstable/experimental
}
```

### UpdateCategory

```rust
pub enum UpdateCategory {
    Kernel,      // Kernel updates
    Security,    // Security patches
    Bugfix,      // Bug fixes
    Feature,     // New features
    Dependency,  // Dependency updates
}
```

### UpdatePackage

Represents a single update package with:
- Package name and version information
- Update level and category
- Size and description
- Security flag
- Source repository

### UpdateManager

Main management interface with:
- Update checking and discovery
- Level-based filtering
- Installation management
- Blacklist management
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::system::UpdateManager;

let mut manager = UpdateManager::new();

// Check for updates
let updates = manager.check_updates();

// Install safety updates
let count = manager.install_safety_updates();

// Get statistics
let stats = manager.get_statistics();
```

### Configuration

```rust
use sigmaos::system::{UpdateManager, UpdateConfig};

let config = UpdateConfig {
    auto_check: true,
    auto_install_safety: true,
    auto_install_security: true,
    check_interval_hours: 24,
    notify_updates: true,
    blacklist: vec![],
};

let mut manager = UpdateManager::with_config(config);
```

### Filtering Updates

```rust
// Get security updates
let security_updates = manager.get_security_updates();

// Get kernel updates
let kernel_updates = manager.get_kernel_updates();

// Get updates by level
let recommended = manager.get_updates_by_level(UpdateLevel::Recommended);
```

## AI Agent Maintenance Instructions

When maintaining the Update Manager:

1. **Update Discovery**: Ensure the `check_updates()` method properly queries all configured repositories
2. **Security Detection**: Maintain accurate security update identification logic
3. **Blacklist Handling**: Ensure blacklist entries are properly respected during update operations
4. **Statistics Accuracy**: Keep statistics tracking accurate for all update operations
5. **Safety Classification**: Ensure update levels are correctly assigned based on impact

## Testing

Run the unit tests with:

```bash
cargo test --lib system::update_manager
```

## Future Enhancements

- Integration with actual package repositories
- Delta update support for bandwidth efficiency
- Rollback functionality for failed updates
- Update notification system integration
- Changelog display for updates
