# SigmaOS Hibernation Components

## Overview
SigmaOS hibernation manager providing hibernation setup and removal, competitive with Omarchy hibernation (omancy-hibernation-available, omancy-hibernation-remove, omancy-hibernation-setup).

## Features
- **Hibernation modes**: Swap, Zram, File
- **Availability check** - Check if hibernation is available
- **Setup/remove** - Configure and remove hibernation
- **Hibernate/resume** - Hibernate and resume system
- **Compression** - Configurable compression

## Structures

| Structure | Description |
|-----------|-------------|
| **HibernationManager** | Hibernation manager |
| **HibernationConfig** | Hibernation configuration |
| **HibernationStatus** | Available, Unavailable, Configured, NotConfigured, Error |
| **HibernationMode** | Swap, Zram, File |

## Usage Examples

```rust
// Create hibernation manager
let mut mgr = HibernationManager::new();

// Check availability
let status = mgr.check_available();

// Setup hibernation
mgr.setup(HibernationMode::Swap).unwrap();

// Hibernate system
mgr.hibernate().unwrap();

// Resume from hibernation
mgr.resume().unwrap();

// Remove hibernation
mgr.remove().unwrap();
```

## Status
- **Implementation**: Complete
- **Dependencies**: Btrfs tools, kernel hibernation support
- **Roadmap Phase**: Power Management (P1)

## Related Files
- `src/hibernation/manager.rs` - Hibernation manager implementation
- `wiki/Hibernation.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
POWER_MANAGEMENT, MEMORY_SAFETY, LOW_LEVEL_CONTROL