# SigmaOS Version Manager Components

## Overview
SigmaOS version manager providing version information management, competitive with Omarchy version (omancy-version, omancy-version-branch, omancy-version-channel, omancy-version-pkgs).

## Features
- **Version info** - Version, branch, channel, package count, build date, commit hash
- **Branch/channel** - Get and set branch and channel
- **Package count** - Track installed package count
- **Commit hash** - Track git commit hash
- **Full version string** - Formatted version string

## Structures

| Structure | Description |
|-----------|-------------|
| **VersionManager** | Version manager |
| **VersionInfo** | Version information |

## Usage Examples

```rust
// Create version manager
let mut mgr = VersionManager::new();

// Get version info
let version = mgr.get_version();
let branch = mgr.get_branch();
let channel = mgr.get_channel();

// Set branch and channel
mgr.set_branch("dev".to_string());
mgr.set_channel("edge".to_string());

// Set package count
mgr.set_package_count(42);

// Get full version string
let full = mgr.get_full_version();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: System Administration (P1)

## Related Files
- `src/version/manager.rs` - Version manager implementation
- `wiki/Version.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, VERSION_CONTROL_PRACTICES, STABLE_APIS,
TRANSPARENCY, USER_CONTROL