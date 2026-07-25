# Sigma Config - Unified System Configuration Tool
# Inspired by YaST (Yet another Setup Tool) from openSUSE
# Provides centralized system configuration management

## Overview

Sigma Config is a unified system configuration tool that provides centralized management of all SigmaOS system settings through a modular configuration system.

## Features

### Configuration Modules

- **System**: Hostname, timezone, locale settings
- **Network**: Network interfaces, DNS, firewall settings
- **Security**: Security policies, user permissions
- **User**: User accounts, home directories
- **Software**: Package management, repositories
- **Hardware**: Device drivers, hardware configuration
- **Services**: System services, daemons

### Configuration Profiles

Pre-configured profiles for different use cases:

- **sigma-core**: Minimal configuration for base system
- **sigma-stable**: Stable configuration for production
- **sigma-rolling**: Rolling configuration for development

## Usage

### Command Line Interface

```bash
# List all configuration modules
sigma-config list-modules

# Get current configuration
sigma-config get system hostname

# Set configuration value
sigma-config set system hostname "sigmaos"

# Apply a profile
sigma-config apply-profile sigma-stable

# Create a new profile
sigma-config create-profile my-profile "Custom configuration"

# Validate configuration
sigma-config validate

# Export configuration
sigma-config export config-backup.json

# Import configuration
sigma-config import config-backup.json
```

### Configuration Files

Configuration is stored in TOML format:

```toml
# /etc/sigma/config/modules/system.toml
[system]
hostname = "sigmaos"
locale = "en_US.UTF-8"
timezone = "UTC"

[system.features]
desktop = true
ai_tools = true
```

## API Usage

### Rust API

```rust
use sigma_config::SigmaConfig;

// Initialize configuration
let config = SigmaConfig::new(PathBuf::from("/etc/sigma/config"))?;

// Set a value
config.set_value("system", "hostname", ConfigValue::String("sigmaos".to_string()))?;

// Get a value
if let Some(ConfigValue::String(hostname)) = config.get_value("system", "hostname") {
    println!("Hostname: {}", hostname);
}

// Apply a profile
config.apply_profile("sigma-stable")?;

// Create a custom profile
config.create_profile("my-profile", "Custom configuration")?;
```

## Module Categories

### System Module

Manages core system settings:
- Hostname configuration
- Locale and timezone
- System profile selection
- Feature flags

### Network Module

Manages network configuration:
- Interface configuration
- DNS settings
- Firewall rules
- Network profiles

### Security Module

Manages security settings:
- Security policies
- User permissions
- Secure boot settings
- TPM configuration

### User Module

Manages user settings:
- User accounts
- Home directories
- User-specific configurations
- Group memberships

### Software Module

Manages software settings:
- Package repositories
- Package manager settings
- Auto-update configuration
- Software profiles

### Hardware Module

Manages hardware configuration:
- Device drivers
- Hardware profiles
- Device-specific settings
- Hardware monitoring

### Services Module

Manages system services:
- Service configuration
- Daemon settings
- Autostart configuration
- Service dependencies

## Profile Management

### Creating Profiles

```bash
# Create a new profile from current configuration
sigma-config create-profile my-profile "My custom configuration"

# Edit profile
sigma-config edit-profile my-profile

# Delete profile
sigma-config delete-profile my-profile
```

### Profile Structure

```toml
# /etc/sigma/config/profiles/my-profile.toml
[profile]
name = "my-profile"
description = "My custom configuration"

[profile.system]
hostname = "my-sigmaos"
locale = "en_US.UTF-8"

[profile.network]
dhcp = true
firewall = "deny"
```

## Configuration Validation

Sigma Config includes built-in validation:

```bash
# Validate current configuration
sigma-config validate

# Validate specific module
sigma-config validate-module system

# Check for configuration errors
sigma-config check
```

## Backup and Restore

### Export Configuration

```bash
# Export all configuration
sigma-config export /backup/sigma-config.json

# Export specific module
sigma-config export-module system /backup/system-config.json
```

### Import Configuration

```bash
# Import configuration
sigma-config import /backup/sigma-config.json

# Import specific module
sigma-config import-module system /backup/system-config.json
```

## Implementation Status

| Feature | Status | Notes |
|---------|--------|-------|
| Configuration modules | ✅ Complete | All 7 modules implemented |
| Profile management | ✅ Complete | Create, apply, list profiles |
| Configuration validation | ✅ Complete | Built-in validation system |
| Import/Export | ✅ Complete | JSON format support |
| CLI interface | ✅ Complete | Full command-line tool |
| Rust API | ✅ Complete | Library for programmatic access |
| Web interface | ⏳ Planned | Future enhancement |
| GUI interface | ⏳ Planned | Future enhancement |

## Best Practices

1. **Use profiles**: Create profiles for different environments
2. **Validate before applying**: Always validate configuration changes
3. **Backup regularly**: Export configuration before major changes
4. **Document changes**: Use descriptive profile names and descriptions
5. **Test in staging**: Apply profiles in test environment first

## Troubleshooting

### Configuration Not Applied

```bash
# Check active profile
sigma-config get-active-profile

# Validate configuration
sigma-config validate

# Check module status
sigma-config status
```

### Profile Errors

```bash
# Validate profile
sigma-config validate-profile my-profile

# Check profile contents
sigma-config show-profile my-profile

# Reset to default
sigma-config reset-profile my-profile
```

## References

- YaST Documentation: https://en.opensuse.org/Portal:YaST
- TOML Specification: https://toml.io/en/
- SigmaOS Configuration Guide: [Configuration Guide](../README.md)
